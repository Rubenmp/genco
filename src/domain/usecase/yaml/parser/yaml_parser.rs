use std::path::PathBuf;

use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::yaml::parser::dto::yaml_node::YamlNode;

pub fn parse(yaml_file_path: &PathBuf) -> YamlNode {
    YamlNode::new(yaml_file_path.as_ref())
}

#[cfg(test)]
mod tests {
    use crate::domain::core::parser::parser_node_trait::ParserNode;
    use crate::domain::core::test::test_assert::assert_same_as_file;
    use std::path::{Path, PathBuf};

    use crate::domain::core::test::test_path::get_test_file_path;
    use crate::domain::usecase::yaml::parser::yaml_parser::parse;

    #[test]
    fn parse_single_file_recognizes_all_tokens() {
        let test_name = "basic.yaml";
        let file_path = get_test_file_path(get_current_file_path(), test_name);

        let root_node = parse(&file_path);

        let expect_result_file_path =
            get_test_file_path(get_current_file_path(), "basic-yaml-expected-result.json");
        let tree_str = root_node.get_tree_str();
        assert_same_as_file(expect_result_file_path, tree_str)
    }

    fn get_current_file_path() -> PathBuf {
        Path::new(file!()).to_path_buf()
    }
}
