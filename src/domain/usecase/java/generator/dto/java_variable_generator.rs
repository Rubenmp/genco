use typed_builder::TypedBuilder;

use crate::domain::usecase::java::generator::dto::java_data_type::JavaDataType::Basic;
use crate::domain::usecase::java::generator::dto::java_data_type::{
    JavaBasicDataType, JavaDataType,
};

#[derive(TypedBuilder)]
pub struct JavaVariableGenerator<'a> {
    #[builder(default = false)]
    is_final: bool,
    data_type: JavaDataType,
    name: &'a str,
}

impl JavaVariableGenerator<'_> {
    pub fn new_final_int(var_name: &str) -> JavaVariableGenerator {
        JavaVariableGenerator::builder()
            .is_final(true)
            .data_type(Basic(JavaBasicDataType::Int))
            .name(var_name)
            .build()
    }
    pub fn new_final_string(var_name: &str) -> JavaVariableGenerator {
        JavaVariableGenerator::builder()
            .is_final(true)
            .data_type(Basic(JavaBasicDataType::String))
            .name(var_name)
            .build()
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        if self.is_final {
            string += "final ";
        }
        let mut data_type_str = self.data_type.to_string();
        if let JavaDataType::Basic(JavaBasicDataType::String) = self.data_type {
        } else {
            data_type_str = data_type_str.to_lowercase();
        }
        string += format!("{} {}", data_type_str, self.name).as_str();
        string
    }
}
