use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs};

use tree_sitter::{Node, Tree};

use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::json::parser::dto::json_node_type::JsonNodeType;

#[derive(Clone)]
pub struct JsonNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<JsonNode>,
    _type_str: String,
    node_type: Option<JsonNodeType>,
}

impl Debug for JsonNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        println!("test");
        Ok(())
    }
}

impl JsonNode {
    fn new_internal(node: Node, file_path: &Path) -> Self {
        let children = node
            .children(&mut node.walk())
            .map(|child| JsonNode::new_internal(child, file_path))
            .collect();

        JsonNode {
            file_path: PathBuf::from(file_path),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            children,
            _type_str: node.kind().to_string(),
            node_type: match JsonNodeType::from_str(node.kind()) {
                Ok(v) => Some(v),
                Err(_e) => panic!("Not possible to parse YamlNode: {}", node.kind()),
            },
        }
    }
}

impl ParserNode for JsonNode {
    fn new(file_path: &Path) -> Self {
        let file_path_str = file_path.to_str().unwrap();
        let file_content = fs::read_to_string(file_path_str).unwrap_or_else(|_| {
            panic!(
                "File path \"{}\" should exists to parse json node",
                file_path_str
            )
        });

        let _tree = parse_json(file_content.as_str());
        JsonNode::new_internal(_tree.root_node(), file_path)
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

    fn get_children_boxes(&self) -> Vec<Box<JsonNode>> {
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

    fn is_printable(&self) -> bool {
        if let Some(node_type) = &self.node_type {
            return matches!(
                node_type,
                JsonNodeType::QuotationMark
                    | JsonNodeType::String
                    | JsonNodeType::Comma
                    | JsonNodeType::Number
                    | JsonNodeType::LBrace
                    | JsonNodeType::RBrace
                    | JsonNodeType::Colon
                    | JsonNodeType::LBracket
                    | JsonNodeType::RBracket
                    | JsonNodeType::Null
            );
        }
        false
    }
}

impl JsonNode {
    pub fn get_node_type(&self) -> Option<JsonNodeType> {
        self.node_type.clone()
    }

    pub fn get_children(&self) -> &Vec<JsonNode> {
        &self.children
    }
}

fn parse_json(code: &str) -> Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_json::language())
        .expect("Error loading json grammar");
    parser.parse(code, None).unwrap()
}
