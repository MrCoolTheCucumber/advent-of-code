use std::{cell::RefCell, collections::VecDeque, fmt::Display, ops::Deref, rc::Rc};

use regex::Regex;

fn main() {
    calculate_size_total();
}

type FileNode = Rc<RefCell<FileSystem>>;

enum FileSystem {
    File {
        name: String,
        size: u64,
    },
    Directory {
        parent: Option<FileNode>,
        name: String,
        contents: Vec<FileNode>,
        size: Option<u64>,
    },
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystem::File { name, size } => write!(f, "{} {}", size, name),
            FileSystem::Directory { name, .. } => write!(f, "dir {}", name),
        }
    }
}

impl FileSystem {
    pub fn root() -> Self {
        FileSystem::Directory {
            parent: None,
            name: "/".to_owned(),
            contents: Vec::new(),
            size: None,
        }
    }

    pub fn get_size(&mut self) -> u64 {
        match self {
            FileSystem::File { size, .. } => *size,
            FileSystem::Directory { contents, size, .. } => {
                if let Some(size) = size {
                    return *size;
                }

                let calculated_size = contents
                    .iter_mut()
                    .map(|node| {
                        let mut node = (*node).borrow_mut();
                        node.get_size()
                    })
                    .sum::<u64>();

                *size = Some(calculated_size);
                calculated_size
            }
        }
    }

    pub fn cd_up(&self) -> FileNode {
        match self {
            FileSystem::Directory { parent, .. } => parent.as_ref().unwrap().clone(),
            _ => panic!("Invalid operation"),
        }
    }

    pub fn cd(&self, cd_arg: &str) -> FileNode {
        match self {
            FileSystem::File { .. } => panic!("Invalid operation, can't cd a file"),
            FileSystem::Directory { contents, .. } => contents
                .iter()
                .find(|f| {
                    let node = (**f).deref().borrow();
                    matches!(node.deref(), FileSystem::Directory { name, .. } if name.as_str() == cd_arg)
                })
                .unwrap()
                .clone(),
        }
    }

    pub fn mkdir(&mut self, dir_name: &str, parent_cell: FileNode) {
        match self {
            FileSystem::File { .. } => panic!("Invalid operation"),
            FileSystem::Directory { contents, .. } => {
                let dir = FileSystem::Directory {
                    parent: Some(parent_cell),
                    name: dir_name.to_owned(),
                    contents: Vec::new(),
                    size: None,
                };
                contents.push(Rc::new(RefCell::new(dir)));
            }
        }
    }

    pub fn touch(&mut self, file_name: &str, file_size: u64) {
        match self {
            FileSystem::File { .. } => panic!("Invalid operation"),
            FileSystem::Directory { contents, .. } => {
                let file = FileSystem::File {
                    name: file_name.to_owned(),
                    size: file_size,
                };
                contents.push(Rc::new(RefCell::new(file)));
            }
        }
    }
}

pub fn calculate_size_total() {
    let mut lines = common::get_lines!().map(|l| l.unwrap()).skip(1).peekable();
    let file_ls_regex = Regex::new(r"(\d+)\s([a-z\.]+)").unwrap();

    let root = Rc::new(RefCell::new(FileSystem::root()));
    let mut current_directory = root.clone();

    while let Some(line) = lines.next() {
        if !line.starts_with('$') {
            unreachable!("Unexpected state occured");
        }

        match &line[2..4] {
            "cd" => {
                let directory = &line[5..];
                match directory {
                    "/" => current_directory = root.clone(),
                    ".." => {
                        current_directory = {
                            let new_dir = current_directory.deref().borrow().cd_up();
                            new_dir
                        }
                    }
                    dir => {
                        current_directory = {
                            let new_dir = current_directory.deref().borrow().cd(dir);
                            new_dir
                        }
                    }
                }
            }
            "ls" => {
                while let Some(peek) = lines.peek() {
                    if peek.starts_with('$') {
                        break;
                    }

                    // unwrap: we just peeked so it must exist
                    let ls_result = unsafe { lines.next().unwrap_unchecked() };
                    if ls_result.starts_with("dir") {
                        let directory = &ls_result[4..];
                        current_directory
                            .borrow_mut()
                            .mkdir(directory, current_directory.clone());
                    } else {
                        let captures = file_ls_regex.captures(&ls_result).unwrap();
                        let file_name = captures.get(2).unwrap().as_str();
                        let file_size = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        current_directory.borrow_mut().touch(file_name, file_size);
                    }
                }
            }
            _ => unreachable!("Invalid command"),
        }
    }

    // search file system tree
    let total_space_taken = root.borrow_mut().get_size();
    let current_free_space = 70000000 - total_space_taken;
    let unused_threashold = 30000000;
    let mut sum_of_sizes = 0;
    let mut smallest_size_can_delete = u64::MAX;
    let mut nodes_to_search: VecDeque<FileNode> = VecDeque::new();
    nodes_to_search.push_front(root);

    while !nodes_to_search.is_empty() {
        let node = nodes_to_search.pop_front().unwrap();
        let mut borrowed = (*node).borrow_mut();

        let node_size = borrowed.get_size();

        if node_size <= 100_000 {
            sum_of_sizes += node_size;
        }

        if current_free_space + node_size >= unused_threashold
            && node_size < smallest_size_can_delete
        {
            smallest_size_can_delete = node_size;
        }

        if let FileSystem::Directory { contents, .. } = borrowed.deref() {
            contents
                .iter()
                .filter_map(|node| {
                    let borrowed = (*node).borrow();
                    match borrowed.deref() {
                        FileSystem::Directory { .. } => Some(node.clone()),
                        _ => None,
                    }
                })
                .for_each(|node| nodes_to_search.push_front(node));
        }
    }

    println!("Sum of sizes: {}", sum_of_sizes);
    println!("Size of smallest deletable: {}", smallest_size_can_delete);
}
