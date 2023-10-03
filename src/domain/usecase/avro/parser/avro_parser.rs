use std::collections::HashMap;
use std::path::Path;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::core::parser::string_helper::trim_quotation_marks;
use crate::domain::usecase::avro::parser::dto::avro_item::AvroItem;
use crate::domain::usecase::avro::parser::dto::avro_item_type::AvroItemType;
use crate::domain::usecase::json::parser::dto::json_node::JsonNode;
use crate::domain::usecase::json::parser::dto::json_node_type::JsonNodeType;
use crate::domain::usecase::json::parser::json_parser;

pub fn parse(json_file_path: &Path) -> Vec<AvroItem> {
    let json_root_node = json_parser::parse(json_file_path);

    let object_nodes = filter_json_nodes_first_level(&json_root_node, &JsonNodeType::Object);
    to_avro_items(object_nodes)
}

fn to_avro_items(json_nodes: Vec<JsonNode>) -> Vec<AvroItem> {
    let mut result = Vec::new();
    for json_node in json_nodes {
        result.push(to_avro_item(&json_node));
    }

    result
}

fn to_avro_item(json_node: &JsonNode) -> AvroItem {
    let pair_nodes = filter_json_nodes_first_level(json_node, &JsonNodeType::Pair);

    let mut json_node_pair_map = HashMap::new();
    for pair_node in pair_nodes {
        let key = get_key_from_pair(&pair_node);
        let value = get_value_node_from_pair(&pair_node);
        json_node_pair_map.insert(key, value);
    }

    AvroItem::new(
        get_content(&json_node_pair_map, "\"name\"".to_string()),
        get_content(&json_node_pair_map, "\"namespace\"".to_string()),
        get_content(&json_node_pair_map, "\"doc\"".to_string()),
        get_item_type(&json_node_pair_map),
        get_avro_symbols(&json_node_pair_map),
        get_content(&json_node_pair_map, "\"default\"".to_string()),
        get_fields(&json_node_pair_map),
    )
}

fn get_fields(pair_map: &HashMap<String, JsonNode>) -> Option<Vec<AvroItem>> {
    let json_node_opt = pair_map.get("\"fields\"");
    if let Some(json_node) = json_node_opt {
        if let Some(JsonNodeType::Array) = json_node.get_node_type() {
            let object_nodes = filter_json_nodes_first_level(json_node, &JsonNodeType::Object);
            return Some(to_avro_items(object_nodes));
        }
    }
    None
}

fn get_item_type(json_node_pair_map: &HashMap<String, JsonNode>) -> AvroItemType {
    let type_node_opt = json_node_pair_map.get("\"type\"");
    if let Some(type_node) = type_node_opt {
        if let Some(JsonNodeType::String) = type_node.get_node_type() {
            let content = type_node.get_content();
            return match content.as_str() {
                "\"array\"" => {
                    if let Some(item_type) = json_node_pair_map.get("\"items\"") {
                        let items_type = get_item_type_base(Some(item_type));
                        return AvroItemType::ArrayItems(Box::new(items_type));
                    } else {
                        panic!("Avro resource must have \"items\" when \"type\" is provided.");
                    }
                }
                _ => get_item_type_base(type_node_opt),
            };
        }
    }

    get_item_type_base(type_node_opt)
}

fn get_item_type_base(type_node_opt: Option<&JsonNode>) -> AvroItemType {
    if let Some(type_node) = type_node_opt {
        let json_node_type = type_node.get_node_type();
        if let Some(JsonNodeType::String) = json_node_type {
            let content = type_node.get_content();
            return match content.as_str() {
                "\"record\"" => AvroItemType::RecordSimple,
                "\"enum\"" => AvroItemType::Enum,
                "\"null\"" => AvroItemType::Null,
                "\"int\"" => AvroItemType::Int,
                "\"long\"" => AvroItemType::Long,
                "\"float\"" => AvroItemType::Float,
                "\"double\"" => AvroItemType::Double,
                "\"string\"" => AvroItemType::String,
                "\"bytes\"" => AvroItemType::Bytes,
                "\"boolean\"" => AvroItemType::Boolean,
                "\"map\"" => AvroItemType::Map,
                _ => AvroItemType::RecordName(trim_quotation_marks(content)),
            };
        } else if let Some(JsonNodeType::Array) = json_node_type {
            let object_nodes = filter_types_in_array(type_node);
            let array_item_types = object_nodes
                .iter()
                .map(|item| get_item_type_base(Some(item)))
                .collect();
            return AvroItemType::Array(array_item_types);
        } else if let Some(JsonNodeType::Object) = json_node_type {
            let avro_item = Box::new(to_avro_item(type_node));
            return AvroItemType::Record(avro_item);
        }
    }

    panic!("Avro resource must have base type.");
}

fn get_content(key_to_value: &HashMap<String, JsonNode>, key: String) -> Option<String> {
    key_to_value
        .get(key.as_str())
        .map(|node| trim_quotation_marks(node.get_content()))
}

fn get_avro_symbols(symbols: &HashMap<String, JsonNode>) -> Option<Vec<String>> {
    if let Some(json_node) = symbols.get("\"symbols\"") {
        let node_contents = filter_json_nodes_first_level(json_node, &JsonNodeType::String)
            .iter()
            .map(|json_node| trim_quotation_marks(json_node.get_content()))
            .collect();
        return Some(node_contents);
    }

    None
}

fn get_value_node_from_pair(node: &JsonNode) -> JsonNode {
    node.get_children().get(2).unwrap().clone()
}

fn get_key_from_pair(node: &JsonNode) -> String {
    node.get_children().get(0).unwrap().get_content()
}

fn filter_json_nodes_first_level(root: &JsonNode, json_node_type: &JsonNodeType) -> Vec<JsonNode> {
    let mut result_nodes: Vec<JsonNode> = Vec::new();
    if let Some(node_type) = root.get_node_type() {
        if json_node_type == &node_type {
            result_nodes.push(root.clone());
            if !result_nodes.is_empty() {
                return result_nodes;
            }
        }
    }

    for child in root.get_children() {
        for mapping_pairs_node in filter_json_nodes_first_level(child, json_node_type) {
            result_nodes.push(mapping_pairs_node);
        }
    }

    result_nodes
}

fn filter_types_in_array(array_node: &JsonNode) -> Vec<JsonNode> {
    let mut result_nodes: Vec<JsonNode> = Vec::new();
    if let Some(node_type) = array_node.get_node_type() {
        if !matches!(
            node_type,
            JsonNodeType::Array
                | JsonNodeType::Comma
                | JsonNodeType::LBrace
                | JsonNodeType::RBrace
                | JsonNodeType::Colon
                | JsonNodeType::LBracket
                | JsonNodeType::RBracket
                | JsonNodeType::Null
        ) {
            result_nodes.push(array_node.clone());
            if !result_nodes.is_empty() {
                return result_nodes;
            }
        }
    }

    for child in array_node.get_children() {
        for mapping_pairs_node in filter_types_in_array(child) {
            result_nodes.push(mapping_pairs_node);
        }
    }

    result_nodes
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_path::get_test_file;
    use crate::domain::usecase::avro::parser::avro_parser;
    use crate::domain::usecase::avro::parser::dto::avro_item::AvroItem;
    use crate::domain::usecase::avro::parser::dto::avro_item_type::AvroItemType;

    #[test]
    #[ignore = "ArrayItems not yet implemented"]
    fn parse_basic() {
        let file_path = get_test_file(get_current_file_path(), "avro.avsc");

        let avro_items = avro_parser::parse(&file_path);

        assert_eq!(2, avro_items.len());
        let avro_item = avro_items.get(0).unwrap();
        assert_eq!(Some("EnumExample".to_string()), avro_item.get_name());
        assert_eq!(Some("com.parser".to_string()), avro_item.get_namespace());
        assert_eq!(
            Some("Enum example for avro".to_string()),
            avro_item.get_doc()
        );
        assert_eq!(&AvroItemType::Enum, avro_item.get_item_type());
        assert_eq!(
            Some(Vec::from([
                "EnumValue1".to_string(),
                "EnumValue2".to_string()
            ])),
            avro_item.get_symbols()
        );
        assert_eq!(None, avro_item.get_default());
        assert_eq!(None, avro_item.get_default());

        let avro_item = avro_items.get(1).unwrap();
        assert_eq!(Some("ComplexFields".to_string()), avro_item.get_name());
        assert_eq!(Some("com.parser".to_string()), avro_item.get_namespace());
        assert_eq!(Some("Docs".to_string()), avro_item.get_doc());
        assert_eq!(&AvroItemType::RecordSimple, avro_item.get_item_type());
        assert_eq!(None, avro_item.get_symbols());
        assert_eq!(None, avro_item.get_default());
        let fields = avro_item.get_fields().as_ref().unwrap();
        assert_eq!(2, fields.len());
        let first_field = fields.get(0).unwrap();
        assert_eq!(Some("field1".to_string()), first_field.get_name());
        assert_eq!(None, first_field.get_namespace());
        assert_eq!(Some("Field 1".to_string()), first_field.get_doc());
        let array_strings_avro_item = AvroItem::new(
            None,
            None,
            None,
            AvroItemType::Array(Vec::new()),
            None,
            None,
            None,
        );
        let array_strings_item_type = AvroItemType::Record(Box::new(array_strings_avro_item));
        let item_types = Vec::from([
            AvroItemType::Null,
            AvroItemType::Int,
            AvroItemType::Long,
            AvroItemType::Float,
            AvroItemType::Double,
            AvroItemType::String,
            AvroItemType::Bytes,
            AvroItemType::Boolean,
            AvroItemType::Map,
            array_strings_item_type,
        ]);
        assert_eq!(
            &AvroItemType::Array(item_types),
            first_field.get_item_type()
        );
        assert_eq!(None, first_field.get_symbols());
        assert_eq!(Some("null".to_string()), first_field.get_default());
        let second_field = fields.get(1).unwrap();
        assert_eq!(Some("enum_example".to_string()), second_field.get_name());
        assert_eq!(None, second_field.get_namespace());
        assert_eq!(Some("Enum example".to_string()), second_field.get_doc());
        assert_eq!(
            &AvroItemType::RecordName("EnumExample".to_string()),
            second_field.get_item_type()
        );
        assert_eq!(None, second_field.get_symbols());
        assert_eq!(None, second_field.get_default());
        assert_eq!(None, second_field.get_default());
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
