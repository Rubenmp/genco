#![allow(unused)]

use std::collections::HashMap;
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
    pub(crate) fn get_block_mapping_pair_strings(&self) -> Option<(String, String)> {
        if Some(YamlNodeType::BlockMappingPair) != self.get_node_type() {
            return None;
        }

        let children = self.get_children();
        if let (Some(key), Some(_), Some(value)) =
            (children.get(0), children.get(1), children.get(2))
        {
            return Some((key.get_content(), value.get_content()));
        }

        None
    }

    pub(crate) fn get_block_mapping_pair_string_to_block(&self) -> Option<(String, &Self)> {
        if Some(YamlNodeType::BlockMappingPair) != self.get_node_type() {
            return None;
        }

        let children = self.get_children();
        if let (Some(key), Some(_), Some(value)) =
            (children.get(0), children.get(1), children.get(2))
        {
            return Some((key.get_content(), value));
        }

        None
    }
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
                                        "Not possible to parse YamlNode \"{}\"",
                                        node.kind()
                                    );
                                }*/
            },
        }
    }
}

impl ParserNode<YamlNodeType> for YamlNode {
    fn from_path(file_path: &Path) -> Result<Self, String> {
        let file_content = match fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(err) => {
                let file_path_str = file_path
                    .to_str()
                    .expect("Not able to convert file to string");
                let result_str = format!(
                    "File path \"{}\" should exists to parse yaml node",
                    file_path_str
                );
                Err(result_str)
            }
        }?;

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

    fn get_children(&self) -> &Vec<Self> {
        &self.children
    }

    fn get_node_type(&self) -> Option<YamlNodeType> {
        self.node_type
    }

    fn get_tree_str(&self) -> String {
        self.get_tree_str_internal(0, 1, false)
    }

    fn is_composed_node_printable(&self) -> bool {
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
        let current_file_path = get_current_file_path();
        let file_path = get_test_file(&current_file_path, "basic.yaml");
        let expect_result_file_path =
            get_test_file(&current_file_path, "basic-yaml-expected-result.json");

        let root_node =
            YamlNode::from_path(&file_path).expect("Yaml node should be parsed correctly");

        let tree_str = root_node.get_tree_str();
        assert_same_as_file(&expect_result_file_path, &tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
