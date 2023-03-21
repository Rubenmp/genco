use typed_builder::TypedBuilder;

use crate::domain::usecase::java::generator::dto::java_data_type_generator::JavaDataTypeGenerator;

#[derive(TypedBuilder)]
pub struct JavaVariableGenerator {
    #[builder(default = false)]
    is_final: bool,
    data_type: JavaDataTypeGenerator,
    name: String,
}

impl JavaVariableGenerator {}
