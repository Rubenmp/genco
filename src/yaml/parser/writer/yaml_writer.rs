use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::file_system::file_creator::file_creator;
use crate::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::yaml::parser::dto::yaml_node::YamlNode;
use crate::yaml::parser::dto::yaml_node_type::YamlNodeType;
use crate::yaml::parser::yaml_parser::parse;

/// Overrides a YAML resource [original_yaml_file] adding the tree structure from another YAML resource
/// [to_add_yaml_file]. In case of YAML properties collision, the previous properties in
/// [original_yaml_file] will be overwritten with the file_overwriting value(s).
/// It panics if the resource does not contain a valid YAML structure.
pub fn overwrite(original_yaml_file: &PathBuf, to_add_yaml_file: &PathBuf) {
    // TODO: check original_yaml_file extension
    file_creator::create_file_if_not_exist(original_yaml_file);
    let yaml_original = parse(original_yaml_file);
    let yaml_to_add = parse(to_add_yaml_file);

    write_yaml(&yaml_original, &yaml_to_add);
}

fn write_yaml(original: &YamlNode, to_add: &YamlNode) {
    let mut overwriting = FileOverwriting::new(&original.get_file_path());
    include_nodes_to_overwrite(&mut overwriting, original, to_add, 0);

    overwriting.write_all();
}

// Requires file_overwriting line: BlockMappingPair, BlockScalar
// Can be directly written: StringScalar, SingleQuoteScalar, DoubleQuoteScalar, BooleanScalar, BlockScalar
fn include_nodes_to_overwrite(
    overwriting: &mut FileOverwriting,
    original_node: &YamlNode,
    node_to_add: &YamlNode,
    depth: usize,
) {
    let original_mapping_pairs = filter_block_mapping_pairs_without_sequence_items(original_node);
    let mapping_pairs_to_add = filter_block_mapping_pairs_without_sequence_items(node_to_add);

    let (new_nodes, modification_nodes) =
        split_into_new_and_modifications(&original_mapping_pairs, &mapping_pairs_to_add);

    include_new_nodes_to_overwrite(overwriting, original_node, new_nodes, depth);
    include_modification_nodes_to_overwrite(
        overwriting,
        &original_mapping_pairs,
        &modification_nodes,
        depth,
    );
    merge_block_sequence_items(overwriting, &original_mapping_pairs, &modification_nodes)
}

fn merge_block_sequence_items(
    overwriting: &mut FileOverwriting,
    original_mapping_pairs: &Vec<YamlNode>,
    modification_mapping_pairs: &Vec<YamlNode>,
) {
    let mut content_key_to_block_sequence_item: HashMap<String, &YamlNode> = HashMap::new();
    let result_sequence_items = filter_block_sequence_items(
        modification_mapping_pairs,
        &mut content_key_to_block_sequence_item,
    );
    let current_sequence_items = filter_block_sequence_items(
        original_mapping_pairs,
        &mut content_key_to_block_sequence_item,
    );

    let sequence_items_to_add_map =
        filter_sequence_items_to_add(current_sequence_items, result_sequence_items);
    for sequence_items_to_add in sequence_items_to_add_map {
        if let Some(mapping_pair) = content_key_to_block_sequence_item.get(&sequence_items_to_add.0)
        {
            for sequence_item_to_add in sequence_items_to_add.1 {
                let content = sequence_item_to_add.get_content_bytes_with_previous_empty_space();
                overwriting
                    .insert_content_with_previous_newline_at(mapping_pair.get_end_byte(), &content);
            }
        }
    }
}

fn filter_sequence_items_to_add<'a>(
    current_sequence_items_map: HashMap<String, Vec<&'a YamlNode>>,
    result_sequence_items_map: HashMap<String, Vec<&'a YamlNode>>,
) -> HashMap<String, Vec<&'a YamlNode>> {
    let mut result = HashMap::new();

    for result_content_with_sequence_item in result_sequence_items_map {
        if let Some(current_sequence_items) =
            current_sequence_items_map.get::<String>(&result_content_with_sequence_item.0.clone())
        {
            let result_sequence_items = result_content_with_sequence_item.1;
            let missing_nodes: Vec<&YamlNode> =
                find_missing(current_sequence_items, result_sequence_items);

            if !missing_nodes.is_empty() {
                result.insert(result_content_with_sequence_item.0.clone(), missing_nodes);
            }
        }
    }

    result
}

fn find_missing<'a>(
    current_sequence_items: &[&YamlNode],
    result_sequence_items: Vec<&'a YamlNode>,
) -> Vec<&'a YamlNode> {
    let current_sequence_items_str: Vec<String> = current_sequence_items
        .iter()
        .map(|item| item.get_content())
        .collect();

    result_sequence_items
        .into_iter()
        .filter(|&item| !current_sequence_items_str.contains(&item.get_content()))
        .collect::<Vec<&YamlNode>>()
}

/// Return a map of pairs where the first item is the BlockMappingPair content
/// and the second its vector of BlockSequenceItem. This return type allows to include the item
/// in the required place.
fn filter_block_sequence_items<'a>(
    mapping_pairs: &'a Vec<YamlNode>,
    content_to_node: &mut HashMap<String, &'a YamlNode>,
) -> HashMap<String, Vec<&'a YamlNode>> {
    let mut hashmap = HashMap::new();
    for mapping_pair in mapping_pairs {
        let mapping_pair_key = get_key_from_block_mapping_pair(mapping_pair);
        let mut items = Vec::new();

        let first_level_node = get_mapped_value_from_block_mapping_pair(mapping_pair);
        if let Some(YamlNodeType::BlockNode) = first_level_node.get_node_type() {
            for second_level_node in first_level_node.get_children() {
                if let Some(YamlNodeType::BlockSequence) = second_level_node.get_node_type() {
                    for third_level_node in second_level_node.get_children() {
                        if let Some(YamlNodeType::BlockSequenceItem) =
                            third_level_node.get_node_type()
                        {
                            items.push(third_level_node);
                            content_to_node.insert(mapping_pair_key.clone(), third_level_node);
                        }
                    }
                }
            }
            if !items.is_empty() {
                hashmap.insert(mapping_pair_key, items);
            }
        }
    }

    hashmap
}

fn include_modification_nodes_to_overwrite(
    overwriting: &mut FileOverwriting,
    original_mapping_pairs: &Vec<YamlNode>,
    modification_nodes: &Vec<YamlNode>,
    depth: usize,
) {
    let original_mapping_pairs_from_key = get_mapping_pairs_from_key(original_mapping_pairs);
    for modification_node in modification_nodes {
        let key = get_key_from_block_mapping_pair(modification_node);
        if let Some(&original_mapping_pair) = original_mapping_pairs_from_key.get(&key) {
            let original_mapped_value =
                get_mapped_value_from_block_mapping_pair(original_mapping_pair);
            let modification_mapped_value =
                get_mapped_value_from_block_mapping_pair(modification_node);

            if let Some(mapped_value_node_type) = modification_mapped_value.get_node_type() {
                if YamlNodeType::FlowNode == mapped_value_node_type {
                    let content = modification_mapped_value.get_content();
                    overwriting.replace(
                        original_mapped_value.get_start_byte(),
                        original_mapped_value.get_end_byte(),
                        &content,
                    );
                } else {
                    include_nodes_to_overwrite(
                        overwriting,
                        original_mapped_value,
                        modification_mapped_value,
                        depth + 1,
                    );
                }
            }
        }
    }
}

fn include_new_nodes_to_overwrite(
    result: &mut FileOverwriting,
    original_node: &YamlNode,
    new_nodes: Vec<YamlNode>,
    depth: usize,
) {
    for new_node in new_nodes {
        if depth == 0 {
            result.append_with_previous_newline(&new_node.get_content());
        } else if is_required_a_newline_before(&new_node) {
            let content = new_node.get_content_bytes_with_previous_empty_space();
            result.insert_content_with_previous_newline_at(original_node.get_end_byte(), &content);
        } else {
            result.replace(
                original_node.get_start_byte(),
                original_node.get_end_byte(),
                &new_node.get_content(),
            );
        }
    }
}

fn is_required_a_newline_before(node: &YamlNode) -> bool {
    if let Some(node_type) = node.get_node_type() {
        if YamlNodeType::BlockMappingPair == node_type {
            return true;
        }
    }

    false
}

fn split_into_new_and_modifications(
    original_mapping_pairs: &Vec<YamlNode>,
    mapping_pairs_to_add: &Vec<YamlNode>,
) -> (Vec<YamlNode>, Vec<YamlNode>) {
    let original_mapping_pairs_from_key = get_mapping_pairs_from_key(original_mapping_pairs);

    let mut new_mapping_pairs = Vec::new();
    let mut present_mapping_pairs = Vec::new();
    for mapping_pair_to_add in mapping_pairs_to_add {
        let key: String = get_key_from_block_mapping_pair(mapping_pair_to_add);
        if original_mapping_pairs_from_key.contains_key::<String>(&key) {
            present_mapping_pairs.push(mapping_pair_to_add.clone());
        } else {
            new_mapping_pairs.push(mapping_pair_to_add.clone());
        }
    }

    (new_mapping_pairs, present_mapping_pairs)
}

fn get_mapping_pairs_from_key(mapping_pairs_to_add: &Vec<YamlNode>) -> HashMap<String, &YamlNode> {
    let mut mapping_pairs_from_key = HashMap::new();
    for mapping_pair_to_add in mapping_pairs_to_add {
        let key = get_key_from_block_mapping_pair(mapping_pair_to_add);
        mapping_pairs_from_key.insert(key, mapping_pair_to_add);
    }
    mapping_pairs_from_key
}

fn filter_block_mapping_pairs_without_sequence_items(root: &YamlNode) -> Vec<YamlNode> {
    let mut result_nodes: Vec<YamlNode> = Vec::new();
    if let Some(node_type) = root.get_node_type() {
        if YamlNodeType::BlockMappingPair == node_type {
            result_nodes.push(root.clone());
            if !result_nodes.is_empty() {
                return result_nodes;
            }
        }
    }

    for child in root.get_children() {
        let child_node_type = child.get_node_type();
        if Some(YamlNodeType::BlockSequence) != child_node_type
            && Some(YamlNodeType::BlockSequenceItem) != child_node_type
        {
            for mapping_pairs_node in filter_block_mapping_pairs_without_sequence_items(child) {
                result_nodes.push(mapping_pairs_node);
            }
        }
    }

    result_nodes
}

/// Example of BlockMappingPair yaml node for line "name: Martin D'vloper"
///
/// {
///   "1. BlockMappingPair": {
///     "1. FlowNode": {
///       "1. PlainScalar": {
///         "1. StringScalar": "name"
///       }
///     },
///     "2. Colon": ":",
///     "3. FlowNode": {
///       "1. PlainScalar": {
///         "1. StringScalar": "Martin D'vloper"
///       }
///     }
///   }
/// }
///
/// This method would return "name"
fn get_key_from_block_mapping_pair(mapping_pair_node: &YamlNode) -> String {
    return mapping_pair_node
        .get_children()
        .get(0)
        .unwrap()
        .get_children()
        .get(0)
        .unwrap()
        .get_children()
        .get(0)
        .unwrap()
        .get_content();
}

fn get_mapped_value_from_block_mapping_pair(mapping_pair_node: &YamlNode) -> &YamlNode {
    return mapping_pair_node.get_children().get(2).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::core::file_system::file_creator::file_creator::{
        create_file_with_content, remove_file_if_exists,
    };
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::get_non_existing_test_file;
    use crate::yaml::parser::writer::yaml_writer::overwrite;

    #[test]
    fn overwrite_test() {
        let original_file_path = get_yaml_test_file("overwrite_base.yaml");
        let file_to_add_path = get_yaml_test_file("overwrite_base_to_add.yaml");
        let copy_file_path = get_yaml_test_file("overwrite_base_copy.yaml");
        create_file_with_content(&copy_file_path, &original_file_path);

        overwrite(&original_file_path, &file_to_add_path);

        let result_data = fs::read_to_string(&original_file_path)
            .expect("Unable to read expected result resource");
        create_file_with_content(&original_file_path, &copy_file_path);

        let expect_result_file_path = get_yaml_test_file("overwrite_base_expected_result.yaml");
        assert_same_as_file(&expect_result_file_path, &result_data);
        remove_file_if_exists(&copy_file_path);
    }

    #[test]
    fn overwrite_new_hyphen_item() {
        let original_file_path = get_yaml_test_file("overwrite_new_hyphen_item.yaml");
        let file_to_add_path = get_yaml_test_file("overwrite_new_hyphen_item_to_add.yaml");
        let copy_file_path = get_yaml_test_file("overwrite_new_hyphen_item_copy.yaml");
        create_file_with_content(&copy_file_path, &original_file_path);

        overwrite(&original_file_path, &file_to_add_path);

        let result_data = fs::read_to_string(&original_file_path)
            .expect("Unable to read expected result resource");
        create_file_with_content(&original_file_path, &copy_file_path);

        let expect_result_file_path =
            get_yaml_test_file("overwrite_new_hyphen_item_expected_result.yaml");
        assert_same_as_file(&expect_result_file_path, &result_data);
        remove_file_if_exists(&copy_file_path);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }

    fn get_yaml_test_file(file_name: &str) -> PathBuf {
        get_non_existing_test_file(get_current_file_path(), file_name)
    }
}
