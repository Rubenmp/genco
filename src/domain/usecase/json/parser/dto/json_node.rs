use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tree_sitter::{Node, Tree};

use crate::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::json::parser::dto::json_node_type::JsonNodeType;

#[derive(Debug, Clone)]
pub(crate) struct JsonNode {
    file_path: PathBuf,
    start_byte: usize,
    end_byte: usize,
    children: Vec<JsonNode>,
    node_type: Option<JsonNodeType>,
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
            node_type: match JsonNodeType::from_str(node.kind()) {
                Ok(v) => Some(v),
                Err(_e) => panic!("Not possible to parse YamlNode: {}", node.kind()),
            },
        }
    }
}

impl ParserNode<JsonNodeType> for JsonNode {
    fn from_path(file_path: &Path) -> Result<Self, String> {
        let file_path_str = file_path.to_str().expect("File path must exist");
        let file_content = fs::read_to_string(file_path_str).unwrap_or_else(|_| {
            panic!(
                "File path \"{}\" should exists to parse json node",
                file_path_str
            )
        });

        let _tree = parse_json(file_content.as_str());
        let new_json_node = JsonNode::new_internal(_tree.root_node(), file_path);
        Ok(new_json_node)
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

    fn get_children(&self) -> &Vec<JsonNode> {
        &self.children
    }

    fn get_node_type(&self) -> Option<JsonNodeType> {
        self.node_type
    }

    fn is_composed_node_printable(&self) -> bool {
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
        self.node_type
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
    parser.parse(code, None).expect("Parsing json")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parser::parser_node_trait::ParserNode;
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::get_test_file;
    use crate::domain::usecase::json::parser::dto::json_node::JsonNode;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_test_file(&get_current_file_path(), "basic.json");
        let expect_result_file_path =
            get_test_file(&get_current_file_path(), "basic-expected_node_tree.json");

        let root_node =
            JsonNode::from_path(&file_path).expect("Json node should be parsed correctly");

        let tree_str = root_node.get_tree_str();
        assert_same_as_file(&expect_result_file_path, &tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
