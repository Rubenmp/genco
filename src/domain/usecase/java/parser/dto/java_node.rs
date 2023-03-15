use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Parser};

use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::java::parser::dto::java_node_type::JavaNodeType;

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
                Err(_e) => None,
            },
        }
    }

    pub fn get_node_type(&self) -> Option<JavaNodeType> {
        self.node_type.clone()
    }

    pub fn get_children(self) -> Vec<JavaNode> {
        self.children
    }

    fn get_node_type_str(&self) -> String {
        if let Some(some_node_type) = self.get_node_type() {
            some_node_type.to_string()
        } else {
            "None".to_string()
        }
    }

    fn get_tree_str(&self, depth: usize) -> String {
        let mut tree_str: String = "  ".repeat(depth);
        tree_str.push_str(self.get_node_type_str().as_str());
        let children = self.clone().get_children();
        if !children.is_empty() {
            tree_str.push_str(" {\n");

            for child in children {
                tree_str.push_str(child.get_tree_str(depth + 1).as_str());
                tree_str.push('\n');
            }

            tree_str.push_str("  ".repeat(depth).as_str());
            tree_str.push('}');
        }
        tree_str
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
        let _tree = parser.parse(file_content, None).unwrap();
        JavaNode::new_internal(_tree.root_node(), file_path)
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
        todo!()
    }

    fn get_node_type_str(&self) -> Option<String> {
        todo!()
    }

    fn get_tree_str(&self) -> String {
        self.get_tree_str(0)
    }

    fn is_printable(&self) -> bool {
        todo!()
    }
}

fn build_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    parser
}
