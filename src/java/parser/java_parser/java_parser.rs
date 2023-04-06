#![allow(unused)]

use std::path::Path;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_file::JavaFile;
use crate::java::parser::dto::java_node::JavaNode;

pub fn parse(java_file_path: &Path) -> JavaFile {
    let _java_node = JavaNode::new(java_file_path);
    let _tree = _java_node.get_tree_str();

    let _a = 0;
    JavaFile::new(_java_node)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::core::testing::test_path::{
        get_java_test_file, get_test_dir, get_test_file,
    };
    use crate::java::parser::java_parser::java_parser;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "Java_17_Maven_JavaParser_Parse_BasicTest",
        );

        java_parser::parse(&file_path);
    }

    #[test]
    fn parse_database_entity() {
        let mut file_path = get_test_file(get_current_file_path(), "DatabaseEntity.java");

        java_parser::parse(&file_path);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
