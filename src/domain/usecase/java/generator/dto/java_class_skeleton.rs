use typed_builder::TypedBuilder;

use crate::domain::usecase::java::generator::dto::java_visibility::JavaVisibility;
use crate::domain::usecase::java::parser::dto::java_annotation::JavaAnnotation;

#[derive(TypedBuilder)]
pub struct JavaClassSkeleton<'a> {
    #[builder(default = Vec::new())]
    annotations: Vec<JavaAnnotation>,
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

impl JavaClassSkeleton<'_> {}
