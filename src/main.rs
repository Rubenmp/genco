use std::env;
use std::path::{Path, PathBuf};

use crate::cli::custom::inditex::event::producer::itx_producer::create_producer;
use crate::java::parser::java_parser::java_parser;

mod cli;
mod core;
mod domain;
mod java;
mod yaml;

fn main() {
    create_producer();
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
