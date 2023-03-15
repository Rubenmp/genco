extern crate core;

use std::path::Path;
use crate::cli::custom::inditex::event::quality_control::request_quality_controls_db_snowflake::request_qc::request_quality_controls;

mod cli;
mod domain;

fn main() {
    request_quality_controls();
    /*
        let java_file_path = Path::new(r"C:\Users\Ruben Morales\MCA\Projects\Personal\genco\genco\resource\test\src\domain\usecase\java\parser\java_parser\Java_17_Maven_JavaParser_Parse_BasicTest\src\main\java\org\gencotest\Main.java");

        let _java_file = domain::usecase::java::parser::java_parser::parse(java_file_path);
    */
    println!("Finish")
}
