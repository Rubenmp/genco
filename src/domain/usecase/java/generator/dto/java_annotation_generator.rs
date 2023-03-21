use std::fmt;

use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct JavaAnnotationGenerator<'a> {
    name: &'a str,
}

impl JavaAnnotationGenerator<'_> {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl fmt::Display for JavaAnnotationGenerator<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "@{:?}", self.name)?;

        Ok(())
    }
}
