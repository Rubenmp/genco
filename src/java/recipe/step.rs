use std::collections::HashSet;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::core::script::recipe_step_scanner;
use crate::yaml::parser::dto::yaml_node::YamlNode;
use crate::yaml::parser::dto::yaml_node_type::YamlNodeType;

pub(crate) enum JavaRecipeStep {
    ReplaceImport(JavaRecipeStepReplaceImport),
    ReplaceFunctionCall(JavaRecipeStepReplaceFunctionCall),
}

impl JavaRecipeStep {
    fn replace_import(_block_mapping: &YamlNode) -> Self {
        JavaRecipeStep::ReplaceImport(JavaRecipeStepReplaceImport::from_block_mapping(
            _block_mapping,
        ))
    }

    fn replace_function_call(_block_mapping: &YamlNode) -> Self {
        JavaRecipeStep::ReplaceFunctionCall(JavaRecipeStepReplaceFunctionCall::from_block_mapping(
            _block_mapping,
        ))
    }
}

pub(crate) struct JavaRecipeStepReplaceImport {}

impl JavaRecipeStepReplaceImport {
    fn from_block_mapping(_block_mapping: &YamlNode) -> Self {
        Self {}
    }
}

pub(crate) struct JavaRecipeStepReplaceFunctionCall {}

impl JavaRecipeStepReplaceFunctionCall {
    fn from_block_mapping(_block_mapping: &YamlNode) -> Self {
        Self {}
    }
}

const JAVA_STEP_REPLACE_IMPORT: &str = "replaceImport";
const JAVA_STEP_REPLACE_FUNCTION_CALL: &str = "replaceFunctionCall";

impl JavaRecipeStep {
    pub(crate) fn from_block_sequence(steps_block: &YamlNode) -> Result<Vec<Self>, Vec<String>> {
        let step_nodes = steps_block
            .get_children()
            .get(0)
            .ok_or(vec!["Not possible to read java steps".to_string()])?
            .get_children();
        sanity_check_all_children_are_sequence_items(step_nodes)?;
        let mut errors = Vec::new();
        let mut steps = Vec::with_capacity(step_nodes.len());
        for step_node in step_nodes {
            match Self::from_block_sequence_item(&step_node) {
                Ok(new_step) => steps.push(new_step),
                Err(mut err) => errors.append(&mut err),
            };
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(steps)
    }
}

fn sanity_check_all_children_are_sequence_items(
    step_nodes: &Vec<YamlNode>,
) -> Result<(), Vec<String>> {
    let number_of_sequence_items = step_nodes
        .iter()
        .filter(|node| Some(YamlNodeType::BlockSequenceItem) == node.get_node_type())
        .count();

    if number_of_sequence_items < step_nodes.len() {
        return Err(vec!["Invalid Java steps block".to_string()]);
    }
    Ok(())
}

impl JavaRecipeStep {
    fn from_block_sequence_item(_sequence_item: &YamlNode) -> Result<Self, Vec<String>> {
        let java_steps = HashSet::from([JAVA_STEP_REPLACE_IMPORT, JAVA_STEP_REPLACE_FUNCTION_CALL]);
        let (key, block_mapping) =
            recipe_step_scanner::from_block_sequence_item(_sequence_item, java_steps)
                .map_err(|err| vec![err])?;

        if key == JAVA_STEP_REPLACE_IMPORT {
        } else if key == JAVA_STEP_REPLACE_FUNCTION_CALL {
        }
        let step = match key.as_str() {
            JAVA_STEP_REPLACE_IMPORT => Ok(JavaRecipeStep::replace_import(block_mapping)),
            JAVA_STEP_REPLACE_FUNCTION_CALL => {
                Ok(JavaRecipeStep::replace_function_call(block_mapping))
            }
            _ => Err(vec!["Java step not yet implemented".to_string()]),
        }?;

        Ok(step)
    }
}
