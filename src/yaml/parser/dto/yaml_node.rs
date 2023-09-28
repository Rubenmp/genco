#![allow(unused)]

use std::fmt::{format, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs};

use tree_sitter::{Node, Tree};

use crate::core::file_system::file_reader;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::yaml::parser::dto::yaml_node_type::YamlNodeType;

#[derive(Debug, Clone)]
pub(crate) struct YamlNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<YamlNode>,
    node_type: Option<YamlNodeType>,
}

impl YamlNode {
    fn new_internal(node: Node, file_path: &Path) -> Self {
        let children = node
            .children(&mut node.walk())
            .map(|child| YamlNode::new_internal(child, file_path))
            .collect();

        YamlNode {
            file_path: PathBuf::from(file_path),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            children,
            node_type: match YamlNodeType::from_str(node.kind()) {
                Ok(v) => Some(v),
                Err(e) => None, /*{
                                    let _file_content = file_reader::read_string(
                                        &file_path.to_path_buf(),
                                        node.start_byte(),
                                        node.end_byte(),
                                    );
                                    panic!(
                                        "Not possible to parse YamlNode \"{}\": {:?}",
                                        node.kind(),
                                        e
                                    )
                                    return None;
                                }*/
            },
        }
    }

    pub fn get_children(&self) -> &Vec<YamlNode> {
        &self.children
    }

    pub fn get_node_type(&self) -> Option<YamlNodeType> {
        self.node_type
    }

    pub fn get_tree_with_bytes_str(&self) -> String {
        self.get_tree_str_internal(0, 1, true)
    }
}

impl ParserNode<YamlNodeType> for YamlNode {
    fn new(file_path: &Path) -> Result<Self, String> {
        let file_path_str = file_path.to_str().unwrap();
        let file_content = fs::read_to_string(file_path_str).unwrap_or_else(|_| {
            panic!(
                "File path \"{}\" should exists to parse yaml node",
                file_path_str
            )
        });

        let _tree = parse_yaml(file_content.as_str());
        let new_yaml_node = YamlNode::new_internal(_tree.root_node(), file_path);
        Ok(new_yaml_node)
    }

    fn get_start_byte(&self) -> usize {
        self.start_byte
    }

    fn get_end_byte(&self) -> usize {
        self.end_byte
    }

    fn get_file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn get_children(&self) -> Vec<Box<Self>> {
        let mut node_refs = Vec::new();
        for child in self.children.clone() {
            node_refs.push(Box::new(child.clone()));
        }
        node_refs
    }

    fn get_node_type(&self) -> Option<YamlNodeType> {
        if let Some(node_type) = &self.node_type {
            return Some(node_type.to_owned());
        }
        None
    }

    fn get_tree_str(&self) -> String {
        self.get_tree_str_internal(0, 1, false)
    }

    fn is_printable(&self) -> bool {
        if self.get_children().is_empty() {
            return true;
        }

        if let Some(node_type) = self.get_node_type() {
            matches!(
                node_type,
                YamlNodeType::SingleQuoteScalar
                    | YamlNodeType::DoubleQuoteScalar
                    | YamlNodeType::StringScalar
                    | YamlNodeType::BooleanScalar
                    | YamlNodeType::BlockScalar
            )
        } else {
            false
        }
    }
}

fn parse_yaml(code: &str) -> Tree {
    tree_sitter_parsers::parse(code, "yaml")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parser::parser_node_trait::ParserNode;
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::get_test_file;
    use crate::yaml::parser::dto::yaml_node::YamlNode;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_test_file(get_current_file_path(), "basic.yaml");
        let expect_result_file_path =
            get_test_file(get_current_file_path(), "basic-yaml-expected-result.json");

        let root_node = YamlNode::new(&file_path).expect("Yaml node should be parsed correctly");

        let tree_str = root_node.get_tree_str();
        assert_same_as_file(&expect_result_file_path, &tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
