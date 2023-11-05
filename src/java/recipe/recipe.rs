use crate::java::recipe::precondition::JavaRecipePrecondition;
use crate::java::recipe::step::JavaRecipeStep;
use crate::yaml::parser::dto::yaml_node::YamlNode;

pub(crate) struct JavaRecipe {
    precondition: Option<JavaRecipePrecondition>,
    steps: Vec<JavaRecipeStep>,
}

impl JavaRecipe {
    pub(crate) fn new(
        _precondition_node_opt: Option<&YamlNode>,
        _steps_block: &YamlNode,
    ) -> Result<Self, Vec<String>> {
        let mut precondition = None;
        if let Some(precondition_node) = _precondition_node_opt {
            precondition = Some(JavaRecipePrecondition::new(precondition_node)?);
        }

        let steps = JavaRecipeStep::from_block_sequence(_steps_block)?;
        let recipe = Self {
            precondition,
            steps,
        };
        Ok(recipe)
    }

    pub(crate) fn get_precondition(&self) -> &Option<JavaRecipePrecondition> {
        &self.precondition
    }

    pub(crate) fn get_steps(&self) -> &Vec<JavaRecipeStep> {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::script::recipe::Recipe;
    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path;
    use crate::java::recipe::step::JavaRecipeStep;

    #[test]
    fn new_java_recipe() {
        let file = get_local_test_file("java_dependency_upgrade_all.yaml");

        match Recipe::new(&file) {
            Ok(recipe) => match recipe {
                Recipe::Java(java_recipe) => {
                    assert!(java_recipe.get_precondition().is_some());
                    assert_eq!(2, java_recipe.get_steps().len());
                    if let Some(step) = java_recipe.get_steps().get(0) {
                        if let JavaRecipeStep::ReplaceImport(_replace_import) = step {
                        } else {
                            assert_fail("Expected step is ReplaceImport");
                        }
                    }
                    if let Some(step) = java_recipe.get_steps().get(1) {
                        if let JavaRecipeStep::ReplaceFunctionCall(_replace_import) = step {
                        } else {
                            assert_fail("Expected step is ReplaceFunctionCall");
                        }
                    }
                    return;
                }
            },
            Err(errors) => assert_fail(&format!("Number of errors: {}", errors.len())),
        };
    }

    #[test]
    fn new_java_recipe_invalid_step() {
        let file = get_local_test_file("java_invalid_step.yaml");

        match Recipe::new(&file) {
            Ok(_recipe) => assert_fail("Unexpected java recipe"),
            Err(errors) => {
                assert_eq!(1, errors.len());

                if let Some(err) = errors.get(0) {
                    assert_eq!("Unexpected step \"invalidStep\", the available steps are [replaceFunctionCall, replaceImport]", err)
                }
            }
        };
    }

    fn get_local_test_file(file_name: &str) -> PathBuf {
        get_test_folder().join(file_name)
    }

    fn get_test_folder() -> PathBuf {
        test_path::get_test_dir_raw(&get_current_file_path())
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
