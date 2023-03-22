use typed_builder::TypedBuilder;

use crate::java::generator::dto::java_annotation_generator::JavaAnnotationGenerator;
use crate::java::generator::dto::java_data_type::JavaDataType;
use crate::java::generator::dto::java_steps_generator::JavaStepsGenerator;
use crate::java::generator::dto::java_variable_generator::JavaVariableGenerator;
use crate::java::generator::dto::java_visibility::JavaVisibility;

#[derive(TypedBuilder)]
pub struct JavaMethodGenerator<'a> {
    #[builder(default = Vec::new())]
    annotations: Vec<JavaAnnotationGenerator<'a>>,
    #[builder(default = JavaVisibility::Private)]
    visibility: JavaVisibility,
    #[builder(default = None, setter(strip_option))]
    return_type: Option<JavaDataType>,
    name: &'a str,
    #[builder(default = Vec::new())]
    parameters: Vec<JavaVariableGenerator<'a>>,
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

    pub fn get_return_type(&self) -> &Option<JavaDataType> {
        &self.return_type
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parameters(&self) -> &Vec<JavaVariableGenerator> {
        &self.parameters
    }

    pub fn get_body(&self) -> &Option<JavaStepsGenerator> {
        &self.body
    }
}

impl JavaMethodGenerator<'_> {}
