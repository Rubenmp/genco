use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Parser};

use crate::core::file_system::file_reader;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::java_node_type::JavaNodeType;

#[derive(Debug, Clone)]
pub(crate) struct JavaNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<JavaNode>,
    node_type: Option<JavaNodeType>,
}

impl JavaNode {
    pub(crate) fn get_node_type_opt(&self) -> Option<JavaNodeType> {
        self.node_type.to_owned()
    }

    pub(crate) fn get_children(&self) -> &Vec<JavaNode> {
        &self.children
    }

    pub(crate) fn get_import_decl_content(import_decl_node: JavaNode) -> Result<String, String> {
        if Some(JavaNodeType::ImportDecl) != import_decl_node.get_node_type_opt() {
            return Err("Java import declaration node required".to_string());
        }

        for children_level_one in import_decl_node.get_children() {
            if Some(JavaNodeType::ScopedIdentifier) == children_level_one.get_node_type_opt() {
                return Ok(children_level_one.get_content());
            }
        }

        Err("Import scoped identifier not found".to_string())
    }
}

impl ParserNode<JavaNodeType> for JavaNode {
    fn new(file_path: &Path) -> Result<Self, String> {
        let file_path_str = file_path
            .to_str()
            .expect("ParserNode::new expect a valid file_path input parameter");

        if let Ok(file_content) = fs::read_to_string(file_path_str) {
            let mut parser = build_parser();
            let parsed = parser.parse(file_content, None);
            let result: Result<Self, String> = if let Some(parsed_tree) = parsed {
                let node = JavaNode::new_internal(parsed_tree.root_node(), file_path);
                Ok(node)
            } else {
                Err(format!(
                    "Error parsing java in file path \"{}\"",
                    file_path_str
                ))
            };
            return result;
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

    fn get_children(&self) -> Vec<Box<Self>> {
        let mut node_refs = Vec::new();
        for child in self.children.clone() {
            node_refs.push(Box::new(child.clone()));
        }
        node_refs
    }

    fn get_node_type(&self) -> Option<JavaNodeType> {
        if let Some(node_type) = &self.node_type {
            return Some(node_type.to_owned());
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

// Private methods
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
                    Self::log_unrecognized_node_type(node, file_path);
                    None
                }
            },
        }
    }

    fn log_unrecognized_node_type(node: Node, file_path: &Path) {
        logger::log_warning(
            format!(
                "Unrecognized node type \"{}\" in expression \"{}\" in file:\n{}\n",
                node.kind(),
                file_reader::read_string(file_path, node.start_byte(), node.end_byte()),
                try_to_absolute_path(file_path)
            )
                .as_str(),
        );
    }
}

fn build_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    parser
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parser::parser_node_trait::ParserNode;
    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path;
    use crate::java::parser::java_node::JavaNode;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_local_java_project_test_folder().join("JavaParserTest.java");

        if let Err(error) = JavaNode::new(&file_path) {
            assert_fail(&error);
        }
    }

    #[test]
    fn parse_database_entity() {
        let file_path = get_local_java_project_test_folder().join("JavaParserDatabaseEntity.java");

        if let Err(error) = JavaNode::new(&file_path) {
            assert_fail(&error);
        }
    }

    fn get_local_java_project_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "java_node")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
