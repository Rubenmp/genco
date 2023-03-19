use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct JavaAnnotationGenerator<'a> {
    name: &'a str,
}

impl JavaAnnotationGenerator<'_> {}
