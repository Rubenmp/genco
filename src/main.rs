use std::env;
use std::path::{Path, PathBuf};

use crate::java::parser::java_parser::java_parser;

mod cli;
mod domain;
mod java;

fn main() {
    println!("Finish")
}

fn get_current_file_path() -> PathBuf {
    let path = Path::new(file!());
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    }
}
