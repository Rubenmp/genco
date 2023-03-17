#![allow(unused)]

use super::dto::java_node;
use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::java::parser::dto::java_file::JavaFile;
use std::path::Path;

pub fn parse(java_file_path: &Path) -> JavaFile {
    let _java_node = java_node::JavaNode::new(java_file_path);
    let _tree = _java_node.get_tree_str();

    JavaFile::new(_java_node)
}

#[cfg(test)]
mod tests {
    use crate::domain::core::test::test_path::get_java_test_file_path;
    use crate::domain::usecase::java::parser::java_parser::parse;
    use std::path::{Path, PathBuf};

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let test_name = "Java_17_Maven_JavaParser_Parse_BasicTest";
        let file_path = get_java_test_file_path(get_current_file_path(), test_name);

        parse(&file_path);
    }

    fn get_current_file_path() -> PathBuf {
        Path::new(file!()).to_path_buf()
    }
}
