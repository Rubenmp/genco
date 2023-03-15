#![allow(unused)]

use std::fmt::{format, Write};
use std::{fmt, fs};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Tree};

use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::yaml::parser::dto::yaml_node_type::YamlNodeType;

#[derive(Debug, Clone)]
pub struct YamlNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<YamlNode>,
    node_type: Option<YamlNodeType>,
}


impl YamlNode {
    fn new_internal(node: Node, file_path: &Path) -> Self {
        let children = node.children(&mut node.walk())
            .map(|child| YamlNode::new_internal(child, file_path))
            .collect();

        YamlNode {
            file_path: PathBuf::from(file_path),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            children,
            node_type: match YamlNodeType::from_str(node.kind()) {
                Ok(v) => Some(v),
                Err(_e) => panic!("Not possible to parse YamlNode: {}", node.kind()),
            },
        }
    }

    pub fn get_children(&self) -> &Vec<YamlNode> {
        &self.children
    }

    pub fn get_node_type(&self) -> Option<YamlNodeType> {
        self.node_type.clone()
    }

    pub fn get_tree_with_bytes_str(&self) -> String {
        self.get_tree_str_internal(0, 1, true)
    }
}

impl ParserNode for YamlNode {
    fn new(file_path: &Path) -> Self {
        let file_path_str = file_path.to_str().unwrap();
        let file_content = fs::read_to_string(file_path_str)
            .unwrap_or_else(|_| panic!("File path \"{}\" should exists to parse yaml node", file_path_str));

        let _tree = parse_yaml(file_content.as_str());
        YamlNode::new_internal(_tree.root_node(), file_path)
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

    fn get_children_boxes(&self) -> Vec<Box<Self>> {
        let mut node_refs = Vec::new();
        for child in self.children.clone() {
            node_refs.push(Box::new(child.clone()));
        }
        node_refs
    }

    fn get_node_type_str(&self) -> Option<String> {
        if let Some(node_type) = &self.node_type {
            return Some(node_type.to_string());
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
            matches!(node_type,
                YamlNodeType::SingleQuoteScalar |
                YamlNodeType::DoubleQuoteScalar |
                YamlNodeType::StringScalar |
                YamlNodeType::BooleanScalar |
                YamlNodeType::BlockScalar)
        } else {
            false
        }
    }
}


fn parse_yaml(code: &str) -> Tree {
    tree_sitter_parsers::parse(code, "yaml")
}

