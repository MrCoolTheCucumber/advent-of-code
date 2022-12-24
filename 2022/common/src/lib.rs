use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_lines(pkg_name: &str) -> Lines<BufReader<File>> {
    let path = env::current_dir()
        .unwrap()
        .join(format!("{}/input.txt", pkg_name));
    let input_file = File::open(&path).expect("Input file not found");
    BufReader::new(input_file).lines()
}

#[macro_export]
macro_rules! get_lines {
    () => {
        $crate::get_lines(env!("CARGO_PKG_NAME"))
    };
}
