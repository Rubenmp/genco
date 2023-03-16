extern crate core;

use crate::cli::custom::inditex::event::quality_control::endpoint_event_publish_qc::itx_endpoint_event_publish_qc::create_endpoint_event_publish;

mod cli;
mod domain;

fn main() {
    create_endpoint_event_publish();
    /*
        let java_file_path = Path::new(r"C:\Users\Ruben Morales\MCA\Projects\Personal\genco\genco\resource\test\src\domain\usecase\java\parser\java_parser\Java_17_Maven_JavaParser_Parse_BasicTest\src\main\java\org\gencotest\Main.java");

        let _java_file = domain::usecase::java::parser::java_parser::parse(java_file_path);
    */
    println!("Finish")
}
