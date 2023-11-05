#![allow(unused)]

use std::path::{Path, PathBuf};

pub fn ask_path(input_text: &str) -> PathBuf {
    let mut line = String::new();
    println!("{}", input_text);
    std::io::stdin()
        .read_line(&mut line)
        .expect("User input expected");
    PathBuf::from(line)
}

pub fn ask_input(input_text: &str) -> String {
    let mut line = String::new();
    println!("{}", input_text);
    std::io::stdin()
        .read_line(&mut line)
        .expect("User input expected");
    line.pop(); // Remove last newline
    line
}
