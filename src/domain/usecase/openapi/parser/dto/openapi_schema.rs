use std::fmt;
use std::fmt::Formatter;

use crate::domain::usecase::openapi::parser::dto::openapi_data_type::OpenapiDataType;

#[derive(Debug, Clone)]
pub struct OpenapiSchema {
    name: String,
    schema_type: Option<OpenapiDataType>,
    description: Option<String>,
    enum_values: Option<Vec<String>>,
    dollar_ref: Option<String>,
    example: Option<String>,
    properties: Option<Vec<OpenapiSchema>>,
}

impl OpenapiSchema {
    pub fn new_enum(name: String, description: Option<String>, enum_values: Vec<String>) -> Self {
        OpenapiSchema {
            name,
            schema_type: Some(OpenapiDataType::String),
            description,
            enum_values: Some(enum_values),
            dollar_ref: None,
            example: None,
            properties: None,
        }
    }

    pub fn new_record(
        name: String,
        description: Option<String>,
        fields: Vec<OpenapiSchema>,
    ) -> Self {
        OpenapiSchema {
            name,
            schema_type: Some(OpenapiDataType::ObjectSimple),
            description,
            enum_values: None,
            dollar_ref: None,
            example: None,
            properties: Some(fields),
        }
    }

    pub fn new_ref(name: String, dollar_ref: String) -> Self {
        OpenapiSchema {
            name,
            schema_type: Some(OpenapiDataType::ObjectSimple),
            description: None,
            enum_values: None,
            dollar_ref: Some(dollar_ref),
            example: None,
            properties: None,
        }
    }

    pub fn new_basic_type(
        name: String,
        description: Option<String>,
        data_type: OpenapiDataType,
    ) -> Self {
        OpenapiSchema {
            name,
            schema_type: Some(data_type),
            description,
            enum_values: None,
            dollar_ref: None,
            example: None,
            properties: None,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_enum_values(&self) -> &Option<Vec<String>> {
        &self.enum_values
    }

    pub fn get_schema_type(&self) -> &Option<OpenapiDataType> {
        &self.schema_type
    }

    pub fn get_properties(&self) -> &Option<Vec<OpenapiSchema>> {
        &self.properties
    }

    pub fn get_format(&self) -> Option<String> {
        if let Some(schema_type) = self.get_schema_type() {
            return Self::get_format_from_schema_type(schema_type);
        }
        None
    }

    fn get_format_from_schema_type(schema_type: &OpenapiDataType) -> Option<String> {
        if let OpenapiDataType::Integer(integer_format) = schema_type {
            return Some(integer_format.to_string().to_lowercase());
        } else if let OpenapiDataType::Number(number_format) = schema_type {
            return Some(number_format.to_string().to_lowercase());
        } else if let OpenapiDataType::Array(subtypes) = schema_type {
            let subtypes_without_null = get_sub_types_without_null(subtypes);
            if subtypes_without_null.len() == 1 {
                if let Some(&subtype) = subtypes_without_null.get(0) {
                    return Self::get_format_from_schema_type(subtype);
                }
            }
        }

        None
    }

    pub fn get_required_properties(&self) -> Vec<String> {
        if let Some(properties) = &self.properties {
            let mut result = Vec::new();
            for property in properties {
                if let Some(schema_type) = &property.get_schema_type() {
                    if let OpenapiDataType::Array(data_types) = &schema_type {
                        if !contains_null_type(data_types) {
                            result.push(property.get_name().clone());
                        }
                    } else {
                        result.push(property.get_name().clone());
                    }
                }
            }
            return result;
        }

        Vec::new()
    }
}

fn write_openapi_schema(fmt: &mut Formatter, depth: usize, schema: &OpenapiSchema) {
    fmt.write_str(format!("{}{}:\n", get_indentation(depth), schema.get_name().clone()).as_str())
        .expect("Error writing OpenapiSchema name");
    if let Some(description) = &schema.get_description() {
        fmt.write_str(
            format!(
                "{}description: {}\n",
                get_indentation(depth + 1),
                description
            )
            .as_str(),
        )
        .expect("Error writing OpenapiSchema description");
    }
    if let Some(example) = &schema.example {
        fmt.write_str(format!("{}example: {}\n", get_indentation(depth + 1), example).as_str())
            .expect("Error writing OpenapiSchema example");
    }
    if let Some(schema_type) = &schema.get_schema_type() {
        let schema_type_str = get_str(schema_type);
        fmt.write_str(
            format!("{}type: {}\n", get_indentation(depth + 1), schema_type_str).as_str(),
        )
        .expect("Error writing OpenapiSchema type");
    }
    if let Some(format) = &schema.get_format() {
        fmt.write_str(format!("{}format: {}\n", get_indentation(depth + 1), format).as_str())
            .expect("Error writing OpenapiSchema type");
    }

    if let Some(enum_values) = schema.get_enum_values() {
        fmt.write_str(format!("{}enum:\n", get_indentation(depth + 1)).as_str());
        for enum_value in enum_values {
            fmt.write_str(format!("{}- {}\n", get_indentation(depth + 2), enum_value).as_str());
        }
    }
    let required_properties = schema.get_required_properties();
    if !required_properties.is_empty() {
        fmt.write_str(format!("{}required:\n", get_indentation(depth + 1)).as_str());
        for property in required_properties {
            fmt.write_str(format!("{}- \"{}\"\n", get_indentation(depth + 2), property).as_str());
        }
    }
    if let Some(dollar_ref) = &schema.dollar_ref {
        fmt.write_str(format!("{}$ref: {}\n", get_indentation(depth + 1), dollar_ref).as_str())
            .expect("Error writing OpenapiSchema $ref");
    }
    if let Some(properties) = &schema.get_properties() {
        fmt.write_str(format!("{}properties:\n", get_indentation(depth + 1)).as_str());
        for property in properties {
            write_openapi_schema(fmt, depth + 2, property);
        }
    }
}

fn get_str(data_type: &OpenapiDataType) -> String {
    if let OpenapiDataType::ObjectSimple = data_type {
        return "object".to_string();
    } else if let OpenapiDataType::Object(_) = data_type {
        return "object".to_string();
    } else if let OpenapiDataType::Integer(_) = data_type {
        return "integer".to_string();
    } else if let OpenapiDataType::Number(_) = data_type {
        return "number".to_string();
    } else if let OpenapiDataType::Array(subtypes) = data_type {
        let subtypes_without_null = get_sub_types_without_null(subtypes);
        if subtypes_without_null.len() == 1 {
            if let Some(subtype) = subtypes_without_null.get(0) {
                return get_str(subtype);
            }
        }
    }

    data_type.to_string()
}

fn get_sub_types_without_null(subtypes: &Vec<OpenapiDataType>) -> Vec<&OpenapiDataType> {
    let mut subtypes_without_null = Vec::new();
    for subtype in subtypes {
        if let OpenapiDataType::Null = subtype {
        } else {
            subtypes_without_null.push(subtype);
        }
    }
    subtypes_without_null
}

fn contains_null_type(data_types: &Vec<OpenapiDataType>) -> bool {
    for data_type in data_types {
        if let OpenapiDataType::Null = data_type {
            return true;
        }
    }

    false
}

impl fmt::Display for OpenapiSchema {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write_openapi_schema(fmt, 0, self);
        Ok(())
    }
}

fn get_indentation(depth: usize) -> String {
    "  ".repeat(depth)
}
