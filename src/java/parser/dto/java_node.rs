use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Parser};

use crate::core::file_system::file_reader;
use crate::core::observability::logger::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;

#[derive(Debug, Clone)]
pub struct JavaNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<JavaNode>,
    node_type: Option<JavaNodeType>,
}

impl JavaNode {
    fn new_internal(node: Node, file_path: &Path) -> Self {
        let children = node
            .children(&mut node.walk())
            .map(|child| JavaNode::new_internal(child, file_path))
            .collect();

        JavaNode {
            file_path: PathBuf::from(file_path),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            children,
            node_type: match JavaNodeType::from_str(node.kind()) {
                Ok(v) => Some(v),
                Err(_e) => {
                    logger::log_warning(
                        format!(
                            "Unrecognized node type \"{}\" in expression \"{:?}\"",
                            node.kind(),
                            file_reader::read_string(file_path, node.start_byte(), node.end_byte(),)
                        )
                        .as_str(),
                    );
                    None
                }
            },
        }
    }

    pub fn get_node_type_opt(&self) -> Option<JavaNodeType> {
        self.node_type.to_owned()
    }

    pub fn get_children(&self) -> &Vec<JavaNode> {
        &self.children
    }
}

impl ParserNode for JavaNode {
    fn new(file_path: &Path) -> Self {
        let mut parser = build_parser();

        let file_path_str = file_path.to_str().unwrap();
        let file_content = fs::read_to_string(file_path_str).unwrap_or_else(|_| {
            panic!(
                "File path \"{}\" should exists to parse java node",
                file_path_str
            )
        });
        let parsed_tree = parser.parse(file_content, None);
        let _tree = parsed_tree.unwrap();
        JavaNode::new_internal(_tree.root_node(), file_path)
    }

    fn new_with_result(file_path: &Path) -> Result<Self, String> {
        let mut parser = build_parser();

        let file_path_str = file_path.to_str().unwrap();
        let file_content = fs::read_to_string(file_path_str).unwrap_or_else(|_| {
            panic!(
                "File path \"{}\" should exists to parse java node",
                file_path_str
            )
        });
        if let Some(parsed_tree) = parser.parse(file_content, None) {
            let node = JavaNode::new_internal(parsed_tree.root_node(), file_path);
            return Ok(node);
        }

        Err(String::from("Error parsing java node"))
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

    fn is_printable(&self) -> bool {
        if let Some(node_type) = self.node_type.to_owned() {
            return matches!(
                node_type,
                JavaNodeType::Id
                    | JavaNodeType::Package
                    | JavaNodeType::Import
                    | JavaNodeType::Public
                    | JavaNodeType::Protected
                    | JavaNodeType::Private
                    | JavaNodeType::Static
                    | JavaNodeType::Final
                    | JavaNodeType::Class
                    | JavaNodeType::Interface
                    | JavaNodeType::EnumConstant
                    | JavaNodeType::Extends
                    | JavaNodeType::Implements
                    | JavaNodeType::TypeIdentifier
                    | JavaNodeType::Dot
                    | JavaNodeType::Semicolon
                    | JavaNodeType::LParentheses
                    | JavaNodeType::RParentheses
                    | JavaNodeType::LBrace
                    | JavaNodeType::RBrace
                    | JavaNodeType::LBracket
                    | JavaNodeType::RBracket
                    | JavaNodeType::Comma
                    | JavaNodeType::Equals
                    | JavaNodeType::StringLiteral
                    | JavaNodeType::At
                    | JavaNodeType::VoidType
                    | JavaNodeType::Int
                    | JavaNodeType::Boolean
            );
        }
        false
    }
}

fn build_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    parser
}
