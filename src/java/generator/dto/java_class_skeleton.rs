use typed_builder::TypedBuilder;

use crate::java::generator::dto::java_annotation_generator::JavaAnnotationGenerator;
use crate::java::generator::dto::java_visibility::JavaVisibility;

#[derive(TypedBuilder)]
pub struct JavaClassSkeleton<'a> {
    #[builder(default = Vec::new())]
    annotations: Vec<JavaAnnotationGenerator<'a>>,
    #[builder(default = JavaVisibility::Package)]
    visibility: JavaVisibility,
    #[builder(default = false)]
    is_abstract: bool,
    name: &'a str,
    #[builder(default = None, setter(strip_option))]
    extended_class: Option<&'a str>,
    #[builder(default = None, setter(strip_option))]
    implemented_interface: Option<&'a str>,
    #[builder(default = Vec::new())]
    implemented_interfaces: Vec<&'a str>,
}

impl JavaClassSkeleton<'_> {
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationGenerator> {
        &self.annotations
    }

    pub fn get_visibility(&self) -> &JavaVisibility {
        &self.visibility
    }
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_extended_class(&self) -> &Option<&str> {
        &self.extended_class
    }
    pub fn get_implemented_interfaces(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for &interface in &self.implemented_interfaces {
            result.push(interface);
        }
        if let Some(interface) = &self.implemented_interface {
            result.push(interface);
        }
        result
    }
}
