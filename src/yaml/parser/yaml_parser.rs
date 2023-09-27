use crate::core::parser::parser_node_trait::ParserNode;
use crate::yaml::parser::dto::yaml_node::YamlNode;
use std::path::Path;

pub(crate) fn parse(yaml_file_path: &Path) -> YamlNode {
    YamlNode::new(yaml_file_path).expect("Yaml node should be parsed correctly")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::parser::parser_node_trait::ParserNode;
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::get_test_file;
    use crate::yaml::parser::yaml_parser::parse;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let file_path = get_test_file(get_current_file_path(), "basic.yaml");
        let expect_result_file_path =
            get_test_file(get_current_file_path(), "basic-yaml-expected-result.json");

        let root_node = parse(&file_path);

        let tree_str = root_node.get_tree_str();
        assert_same_as_file(&expect_result_file_path, &tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
