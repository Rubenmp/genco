use std::path::PathBuf;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::json::parser::dto::json_node::JsonNode;

pub fn parse(yaml_file_path: &PathBuf) -> JsonNode {
    JsonNode::new(yaml_file_path.as_ref())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parser::parser_node_trait::ParserNode;
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::get_test_file;
    use crate::domain::usecase::json::parser::json_parser::json_parser::parse;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_test_file(get_current_file_path(), "basic.json");

        let root_node = parse(&file_path);

        let tree_str = root_node.get_tree_str();
        let expect_result_file_path =
            get_test_file(get_current_file_path(), "basic-json-expected-result.json");
        assert_same_as_file(&expect_result_file_path, tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
