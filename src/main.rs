extern crate core;

use crate::domain::usecase::java::parser::java_parser::java_parser;
use std::env;
use std::path::{Path, PathBuf};

mod cli;
mod domain;

fn main() {
    let java_file_path = get_java_file_path();

    let _java_file = java_parser::parse(&java_file_path);

    println!("Finish")
}

fn get_java_file_path() -> PathBuf {
    let mut path = get_current_file_path();
    path.pop();
    path.pop();
    path.push("resource");
    path.push("test");
    path.push("src");
    path.push("domain");
    path.push("usecase");
    path.push("java");
    path.push("parser");
    path.push("java_parser");
    path.push("Java_17_Maven_JavaParser_Parse_BasicTest");
    path.push("src");
    path.push("main");
    path.push("java");
    path.push("org");
    path.push("gencotest");
    path.push("Main.java");

    path.to_path_buf()
}

fn get_current_file_path() -> PathBuf {
    let path = Path::new(file!());
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    }
}
