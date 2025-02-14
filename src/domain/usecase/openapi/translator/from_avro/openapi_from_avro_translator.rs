use crate::domain::usecase::avro::parser::dto::avro_item::AvroItem;
use crate::domain::usecase::avro::parser::dto::avro_item_type::AvroItemType;
use crate::domain::usecase::openapi::parser::dto::openapi_data_type::OpenapiDataType;
use crate::domain::usecase::openapi::parser::dto::openapi_schema::OpenapiSchema;

pub fn avro_to_openapi_str(schemas: &[AvroItem]) -> String {
    let schemas = to_component_schemas(schemas);

    to_openapi_str(&schemas)
}

fn to_component_schemas(avro_items: &[AvroItem]) -> Vec<OpenapiSchema> {
    avro_items.iter().map(to_component_schema).collect()
}

fn to_openapi_str(schemas: &Vec<OpenapiSchema>) -> String {
    let mut result = String::new();
    let schemas_len = schemas.len();
    for (index, schema) in schemas.iter().enumerate() {
        result += schema.to_string().as_str();
        if index < (schemas_len - 1) {
            result += "\n"
        }
    }
    result
}

pub fn to_component_schema(avro_item: &AvroItem) -> OpenapiSchema {
    if let AvroItemType::Enum = avro_item.get_item_type() {
        if let Some(symbols) = avro_item.get_symbols() {
            return OpenapiSchema::new_enum(
                &avro_item.get_name().expect("Avro name expected"),
                avro_item.get_doc(),
                symbols,
            );
        }
    } else if let AvroItemType::RecordSimple = avro_item.get_item_type() {
        let mut properties_result = Vec::new();
        if let Some(fields) = avro_item.get_fields() {
            for field in fields {
                properties_result.push(to_component_schema(field));
            }
        }

        return OpenapiSchema::new_record(
            avro_item.get_name().unwrap(),
            avro_item.get_doc(),
            properties_result,
        );
    } else if let AvroItemType::Array(subtypes) = avro_item.get_item_type() {
        let mut subtypes_result = Vec::new();
        for subtype in subtypes {
            subtypes_result.push(to_data_type(subtype))
        }

        return OpenapiSchema::new_basic_type(
            avro_item.get_name().unwrap(),
            avro_item.get_doc(),
            OpenapiDataType::Array(subtypes_result),
        );
    } else if let AvroItemType::Record(_record) = avro_item.get_item_type() {
        let _a = 0;
    } else {
        return OpenapiSchema::new_basic_type(
            avro_item.get_name().unwrap(),
            avro_item.get_doc(),
            to_data_type(avro_item.get_item_type()),
        );
    }

    panic!(
        "Error translating avro item {} (doc: {})",
        avro_item.get_name().unwrap(),
        avro_item.get_doc().unwrap()
    );
}

pub fn to_data_type(avro_item_type: &AvroItemType) -> OpenapiDataType {
    if let AvroItemType::Int = avro_item_type {
        return OpenapiDataType::new_int32_type();
    } else if let AvroItemType::Long = avro_item_type {
        return OpenapiDataType::new_int64_type();
    } else if let AvroItemType::Float = avro_item_type {
        return OpenapiDataType::new_float_type();
    } else if let AvroItemType::Double = avro_item_type {
        return OpenapiDataType::new_double_type();
    } else if let AvroItemType::Null = avro_item_type {
        return OpenapiDataType::Null;
    } else if let AvroItemType::String = avro_item_type {
        return OpenapiDataType::String;
    } else if let AvroItemType::Boolean = avro_item_type {
        return OpenapiDataType::Boolean;
    } else if let AvroItemType::Bytes = avro_item_type {
        return OpenapiDataType::Bytes;
    } else if let AvroItemType::RecordName(record_name) = avro_item_type {
        return OpenapiDataType::ObjectName(record_name.clone());
    } else if let AvroItemType::Array(subtypes) = avro_item_type {
        let mut translated_types = Vec::new();
        for subtype in subtypes {
            translated_types.push(to_data_type(subtype));
        }
        return OpenapiDataType::Array(translated_types);
    } else if let AvroItemType::ArrayItems(items_type) = avro_item_type {
        let translated_type = to_data_type(items_type);
        return OpenapiDataType::ArrayItems(Box::new(translated_type));
    } else if let AvroItemType::Record(record_box) = avro_item_type {
        if record_box.is_just_type() {
            return to_data_type(record_box.get_item_type());
        } else {
            panic!("TODO: translate compound types");
        }
    }

    panic!("Error translating avro item type");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_edition::file_editor;
    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path::{get_non_existing_test_file, get_test_file};
    use crate::domain::usecase::avro::parser::avro_parser;
    use crate::domain::usecase::openapi::translator::from_avro::openapi_from_avro_translator::avro_to_openapi_str;

    #[test]
    fn avro_to_openapi_str_enum() {
        let file_path = get_test_file(&get_current_file_path(), "enum.avsc");
        let avro_items = avro_parser::parse(&file_path);

        let openapi_str = avro_to_openapi_str(&avro_items);

        let expect_result_file_path =
            get_test_file(&get_current_file_path(), "enum_translated.yaml");
        assert_same_as_file(&expect_result_file_path, &openapi_str)
    }

    #[test]
    fn avro_to_openapi_str_basic_fields() {
        let file_path = get_test_file(&get_current_file_path(), "avro_basic_fields.avsc");
        let avro_items = avro_parser::parse(&file_path);

        let openapi_str = avro_to_openapi_str(&avro_items);

        let expect_result_file_path = get_test_file(
            &get_current_file_path(),
            "avro_basic_fields_translated_to_openapi.yaml",
        );
        assert_same_as_file(&expect_result_file_path, &openapi_str)
    }

    #[test]
    fn avro_to_openapi_str_array_fields() {
        let current_file_path = get_current_file_path();
        let file_path = get_test_file(&current_file_path, "avro_array_fields.avsc");
        let avro_items = avro_parser::parse(&file_path);

        let openapi_str = avro_to_openapi_str(&avro_items);

        let expect_result_file_path = get_test_file(
            &current_file_path,
            "avro_array_fields_translated_to_openapi.yaml",
        );
        assert_same_as_file(&expect_result_file_path, &openapi_str)
    }

    #[test]
    #[ignore]
    fn test() {
        let current_file_path = get_current_file_path();
        let file_path = get_test_file(&current_file_path, "test.avsc");
        let avro_items = avro_parser::parse(&file_path);

        let openapi_str = avro_to_openapi_str(&avro_items);

        let result_file_path =
            get_non_existing_test_file(&current_file_path, "test_translated_to_openapi.yaml");

        file_editor::create_or_replace_file_with_bytes(&result_file_path, &openapi_str.as_bytes())
            .expect("File creation must succeed");
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
