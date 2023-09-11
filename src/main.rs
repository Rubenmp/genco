use std::env;
use std::path::{Path, PathBuf};

mod cli;
mod core;
mod domain;
pub mod java;
pub mod yaml;

fn main() {}

fn get_current_file_path() -> PathBuf {
    let path = Path::new(file!());
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    }
}
