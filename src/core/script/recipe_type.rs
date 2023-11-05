#[derive(Debug, PartialEq)]
pub(crate) enum RecipeType {
    // Update method get_all_types_set_str if new language is added
    Java,
}

impl RecipeType {
    pub(crate) fn get_all_types_set_str() -> String {
        "{java}".to_string()
    }
}

impl RecipeType {
    pub(crate) fn new(input: &str) -> Result<Self, String> {
        match input {
            "java" => Ok(RecipeType::Java),
            _ => Err(format!(
                "Unexpected test type \"{}\", only values in {} are allowed",
                input,
                Self::get_all_types_set_str()
            )
            .to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::script::recipe_type::RecipeType;

    #[test]
    fn new_positive() {
        let language = RecipeType::new("java").expect("Valid ScriptLanguage");
        assert_eq!(RecipeType::Java, language);
    }
}
