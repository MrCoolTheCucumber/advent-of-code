use std::collections::HashSet;

fn main() {
    find_start_of_packet();
    find_start_of_message();
}

fn get_line() -> String {
    common::get_lines!()
        .into_iter()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
}

pub fn find_start_of_packet() {
    let line = get_line();

    let chars: Vec<char> = line.chars().collect();
    let found_idx = chars
        .windows(4)
        .enumerate()
        .find(|(_, window)| {
            let set: HashSet<char> = HashSet::from_iter(window.iter().copied());
            set.len() == 4
        })
        .map(|(idx, _)| idx + 4)
        .unwrap();

    println!("Characters processed: {}", found_idx);
}

pub fn find_start_of_message() {
    let line = get_line();

    let chars: Vec<char> = line.chars().collect();
    let found_idx = chars
        .windows(14)
        .enumerate()
        .find(|(_, window)| {
            let set: HashSet<char> = HashSet::from_iter(window.iter().copied());
            set.len() == 14
        })
        .map(|(idx, _)| idx + 14)
        .unwrap();

    println!("Characters processed: {}", found_idx);
}
