use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Parser};

use crate::core::file_system::file_reader;
use crate::core::observability::logger;
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
                            file_reader::read_string(file_path, node.start_byte(), node.end_byte())
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
    fn new(file_path: &Path) -> Result<Self, String> {
        let mut parser = build_parser();

        let file_path_str = file_path.to_str().unwrap();
        if let Ok(file_content) = fs::read_to_string(file_path_str) {
            return if let Some(parsed_tree) = parser.parse(file_content, None) {
                let node = JavaNode::new_internal(parsed_tree.root_node(), file_path);
                Ok(node)
            } else {
                Err(format!(
                    "Error parsing java in file path \"{}\"",
                    file_path_str
                ))
            };
        }

        Err(format!(
            "Error reading file content from \"{}\"",
            file_path_str
        ))
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
                    | JavaNodeType::Void
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
