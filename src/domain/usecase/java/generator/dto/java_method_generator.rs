use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct JavaMethodGenerator {
    name: String,
    #[builder(default = None, setter(strip_option))]
    implemented_interface: Option<String>,
    #[builder(default = false)]
    is_abstract: bool,
}

impl JavaMethodGenerator {}
