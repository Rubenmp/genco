use std::collections::HashSet;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::yaml::parser::dto::yaml_node::YamlNode;
use crate::yaml::parser::dto::yaml_node_type::YamlNodeType;

/// Given
/// - A Yaml node of type BlockSequenceItem (assumption)
/// - A set of possible keys for the item
/// it returns the key as string and the block, or a custom error for invalid structures
///
/// This is the structure of a Yaml BlockSequenceItem:
///
/// ```json
/// {
///   "1. BlockSequenceItem": {
///     "1. Hyphen": "-",
///     "2. BlockNode": {
///       "1. BlockMapping": {
///         "1. BlockMappingPair": {
///           "1. FlowNode": {
///             "1. PlainScalar": {
///               "1. StringScalar": "replaceImport"
///             }
///           },
///           "2. Colon": ":",
///           "3. BlockNode": {
///             "1. BlockMapping": {
///               "1. BlockMappingPair": {}
///               ...
///               "n. BlockMappingPair": {}
///             }
///           }
///         }
///       }
///     }
///   }
/// }
/// ```
/// and this method will return `("replaceImport", BlockMapping)` or an error.
pub(crate) fn from_block_sequence_item<'a>(
    block_sequence_item: &'a YamlNode,
    possible_keys: HashSet<&str>,
) -> Result<(String, &'a YamlNode), String> {
    let children = block_sequence_item.get_children();
    if Some(Some(YamlNodeType::Hyphen)) != children.get(0).map(|c| c.get_node_type()) {
        return Err("Expected hyphen in Yaml item".to_string());
    }
    let second_child = children
        .get(1)
        .ok_or("Expected node after hyphen in Yaml item")?;
    if Some(YamlNodeType::BlockNode) != second_child.get_node_type() {
        return Err("Expected node after hyphen in Yaml item".to_string());
    }
    let block_node_child = second_child
        .get_children()
        .get(0)
        .ok_or("Expected block mapping node after hyphen in Yaml item".to_string())?;
    if Some(YamlNodeType::BlockMapping) != block_node_child.get_node_type() {
        return Err("Expected block mapping pair node after hyphen in Yaml item".to_string());
    }
    let block_mapping_child = block_node_child
        .get_children()
        .get(0)
        .ok_or("Expected block mapping pair node after block mapping in Yaml item")?;
    if Some(YamlNodeType::BlockMappingPair) != block_mapping_child.get_node_type() {
        return Err(
            "Expected block mapping pair node after block mapping in Yaml item".to_string(),
        );
    }
    let key = block_mapping_child
        .get_children()
        .get(0)
        .ok_or("Expected FlowNode in Yaml item")?
        .get_children()
        .get(0)
        .ok_or("Expected PlainScalar in Yaml item")?
        .get_children()
        .get(0)
        .ok_or("Expected StringScalar in Yaml item")?;
    let key_str = key.get_content();

    if !possible_keys.contains(key_str.as_str()) {
        let err = format!(
            "Unexpected step \"{}\", the available steps are [{}]",
            key_str,
            join_keys_sorted(possible_keys)
        )
        .to_string();
        return Err(err);
    }

    let block_mapping_node = block_mapping_child
        .get_children()
        .get(2)
        .ok_or("Expected BlockNode in Yaml BlockMappingPair item")?
        .get_children()
        .get(0)
        .ok_or("Expected BlockMapping in Yaml BlockNode item")?;
    Ok((key_str, &block_mapping_node))
}

fn join_keys_sorted(possible_keys: HashSet<&str>) -> String {
    let mut vec = possible_keys.into_iter().collect::<Vec<&str>>();
    vec.sort();
    vec.join(", ")
}
