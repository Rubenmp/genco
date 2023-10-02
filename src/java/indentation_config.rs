#[derive(Debug)]
pub struct JavaIndentation {
    base_indentation: String,
    current_level: usize,
}

impl JavaIndentationBuilder {}
impl JavaIndentation {
    pub fn builder() -> JavaIndentationBuilder {
        JavaIndentationBuilder::new_builder()
    }
    pub(crate) fn default() -> Self {
        JavaIndentationBuilder::new_builder().build()
    }

    pub fn get_current_indentation(&self) -> String {
        self.base_indentation.repeat(self.current_level)
    }

    pub(crate) fn increase_level(&mut self) {
        self.current_level += 1;
    }

    pub(crate) fn decrease_level(&mut self) {
        self.current_level -= 1;
    }
}

pub struct JavaIndentationBuilder {
    base_indentation: String,
    current_level: usize,
}

impl JavaIndentationBuilder {
    fn new_builder() -> JavaIndentationBuilder {
        JavaIndentationBuilder {
            base_indentation: "    ".to_string(),
            current_level: 0,
        }
    }
    pub fn base_indentation(&mut self, input: &str) -> &mut Self {
        self.base_indentation = input.to_string();
        self
    }
    pub fn current_level(&mut self, input: usize) -> &mut Self {
        self.current_level = input;
        self
    }
    pub fn build(&mut self) -> JavaIndentation {
        JavaIndentation {
            base_indentation: self.base_indentation.to_owned(),
            current_level: self.current_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::java::indentation_config::JavaIndentation;

    #[test]
    fn builder_default() {
        let indentation = JavaIndentation::builder().build();

        assert_eq!("".to_string(), indentation.get_current_indentation());
    }

    #[test]
    fn builder_increase_level() {
        let mut indentation = JavaIndentation::builder().current_level(1).build();
        indentation.increase_level();

        assert_eq!(
            "        ".to_string(),
            indentation.get_current_indentation()
        );
    }

    #[test]
    fn builder_increase_level_twice() {
        let mut indentation = JavaIndentation::builder().build();
        indentation.increase_level();
        indentation.increase_level();

        assert_eq!(
            "        ".to_string(),
            indentation.get_current_indentation()
        );
    }
}
