use typed_builder::TypedBuilder;

use crate::domain::usecase::java::generator::dto::java_annotation_generator::JavaAnnotationGenerator;
use crate::domain::usecase::java::generator::dto::java_data_type_generator::JavaDataTypeGenerator;
use crate::domain::usecase::java::generator::dto::java_steps_generator::JavaStepsGenerator;
use crate::domain::usecase::java::generator::dto::java_variable_generator::JavaVariableGenerator;
use crate::domain::usecase::java::generator::dto::java_visibility::JavaVisibility;

#[derive(TypedBuilder)]
pub struct JavaMethodGenerator<'a> {
    #[builder(default = Vec::new())]
    annotations: Vec<JavaAnnotationGenerator<'a>>,
    #[builder(default = JavaVisibility::Private)]
    visibility: JavaVisibility,
    #[builder(default = None, setter(strip_option))]
    return_type: Option<JavaDataTypeGenerator>,
    name: String,
    #[builder(default = Vec::new())]
    parameters: Vec<JavaVariableGenerator>,
    #[builder(default = None, setter(strip_option))]
    body: Option<JavaStepsGenerator>,
}

impl<'a> JavaMethodGenerator<'a> {
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationGenerator> {
        &self.annotations
    }

    pub fn get_visibility(&self) -> &JavaVisibility {
        &self.visibility
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl JavaMethodGenerator<'_> {}
