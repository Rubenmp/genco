#![allow(unused)]

use std::path::Path;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_node::JavaNode;

pub fn parse(java_file_path: &Path) -> Result<JavaNode, String> {
    JavaNode::new_with_result(java_file_path)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::core::testing::test_assert::{assert_dir_is, assert_fail};
    use crate::core::testing::test_path::{get_java_test_file, get_test_dir, get_test_file};
    use crate::java::parser::java_parser::java_parser;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "Java_17_Maven_JavaParser_Parse_BasicTest",
            "JavaParserTest.java",
        );

        if let Err(error) = java_parser::parse(&file_path) {
            assert_fail(&error);
        }
    }

    #[test]
    fn parse_database_entity() {
        let mut file_path = get_java_test_file(
            get_current_file_path(),
            "Java_17_Maven_JavaParser_Parse_BasicTest",
            "JavaParserDatabaseEntity.java",
        );

        if let Err(error) = java_parser::parse(&file_path) {
            assert_fail(&error);
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
