use std::path::Path;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::core::script::recipe_type::RecipeType;
use crate::core::script::semver::SemVer;
use crate::java::recipe::recipe::JavaRecipe;
use crate::yaml::parser::dto::yaml_node::YamlNode;

pub(crate) enum Recipe {
    Java(JavaRecipe),
}

const PROGRAM_VERSION_STR: &str = "genco";
const PROGRAM_TYPE_STR: &str = "type";
const PRECONDITION_STR: &str = "precondition";
const PROGRAM_STEPS_STR: &str = "run";

// Public crate methods
impl Recipe {
    pub(crate) fn new(script_path: &Path) -> Result<Self, Vec<String>> {
        Self::new_internal(script_path)
    }
}

// Private crate methods
impl Recipe {
    fn new_internal(script_path: &Path) -> Result<Self, Vec<String>> {
        check_script_path(script_path).map_err(|e| vec![e])?;
        let _yaml_root = YamlNode::from_path(script_path).map_err(|e| vec![e])?;
        let root = _yaml_root
            .get_children()
            .get(0)
            .ok_or(vec!["Not possible to read test".to_string()])?
            .get_children()
            .get(0)
            .ok_or(vec!["Not possible to read test".to_string()])?
            .get_children()
            .get(0)
            .ok_or(vec!["Not possible to read test".to_string()])?;

        let nodes = root.get_children();
        let _tool_version_in_file = Self::detect_semver(nodes).map_err(|e| vec![e]);
        let type_in_file = Self::detect_type(nodes).map_err(|e| vec![e])?;

        let mut node_index = 2;
        let third_node = nodes.get(node_index).ok_or(vec![
            get_expected_precondition_or_steps_after_type_found_nothing_error(),
        ])?;
        let third_node_mapping_opt: Option<(String, &YamlNode)> =
            third_node.get_block_mapping_pair_string_to_block();

        let mut precondition_block_opt = None;
        let mut run_block_opt = None;
        if let Some((key, value_node)) = third_node_mapping_opt {
            if key == PRECONDITION_STR {
                precondition_block_opt = Some(value_node);
            } else if key == PROGRAM_STEPS_STR {
                run_block_opt = Some(value_node);
            } else {
                return Err(vec![
                    get_expected_precondition_or_steps_after_type_but_found_error(key),
                ]);
            }
        } else {
            return Err(vec![
                get_expected_precondition_or_steps_after_type_mapping_error(third_node),
            ]);
        }

        let mut script_opt = None;
        if run_block_opt.is_none() {
            node_index += 1;
            let node_mapping_opt = nodes
                .get(node_index)
                .ok_or(vec![
                    get_expected_steps_after_precondition_found_nothing_error(),
                ])?
                .get_block_mapping_pair_string_to_block();
            if let Some((key, value_node)) = node_mapping_opt {
                if key == PROGRAM_STEPS_STR {
                    let script = get_script(type_in_file, precondition_block_opt, value_node)?;
                    script_opt = Some(script);
                } else {
                    return Err(vec![get_expected_steps_after_precondition_but_found_error(
                        value_node,
                    )]);
                }
            } else {
                return Err(vec![get_expected_steps_after_precondition_error()]);
            };
        } else if let Some(run_block) = run_block_opt {
            let script = get_script(type_in_file, precondition_block_opt, run_block)?;
            script_opt = Some(script);
        }

        if let Some(_extra_node) = nodes.get(node_index + 1) {
            return Err(vec![get_unexpected_node_after_run_error_msg()]);
        }

        Ok(script_opt.expect("Script is always initialized"))
    }

    fn detect_semver(nodes: &Vec<YamlNode>) -> Result<SemVer, String> {
        let tool_version_in_file: Option<(String, String)> = nodes
            .get(0)
            .ok_or(format!(
                "Expected \"{}: x.y.z\" at first line, found nothing",
                PROGRAM_VERSION_STR
            ))?
            .get_block_mapping_pair_strings();

        match tool_version_in_file {
            Some(pair) => {
                if pair.0 != PROGRAM_VERSION_STR {
                    return Err(format!(
                        "Expected \"{}\" found \"{}\")",
                        PROGRAM_VERSION_STR, pair.0
                    )
                    .to_string());
                }
                match SemVer::new(&pair.1) {
                    Ok(_semver) => {
                        if _semver.is_greater_than_program_version() {
                            // TODO: Update program version automatically, then continue
                        }
                        return Ok(_semver);
                    }
                    Err(err) => Err(format!("Error: {}", err).to_string()),
                }
            }
            None => Err(
                format!("Expected \"{}: x.y.z\" at first line", PROGRAM_VERSION_STR).to_string(),
            ),
        }
    }

    fn detect_type(nodes: &Vec<YamlNode>) -> Result<RecipeType, String> {
        let script_type_in_file: Option<(String, String)> = nodes
            .get(1)
            .ok_or(get_unexpected_type_message())?
            .get_block_mapping_pair_strings();

        match script_type_in_file {
            Some(pair) => {
                if pair.0 != PROGRAM_TYPE_STR {
                    return Err(format!("Expected \"{}: <script_type>\" where <script_type> is one of {} found \"{}\")", PROGRAM_TYPE_STR, RecipeType::get_all_types_set_str(), pair.0).to_string());
                }
                match RecipeType::new(&pair.1) {
                    Ok(type_in_file) => Ok(type_in_file),
                    Err(err) => Err(format!("Error: {}", err).to_string()),
                }
            }
            None => Err(get_unexpected_type_message()),
        }
    }
}

fn check_script_path(script_path: &Path) -> Result<(), String> {
    if !script_path.exists() {
        return Err(format!(
            "Script path \"{}\" must exist",
            script_path
                .to_str()
                .expect("Not able to convert file to string")
        ));
    }
    if script_path.is_dir() {
        return Err(format!(
            "Script path \"{}\" can not be a directory",
            script_path
                .to_str()
                .expect("Not able to convert directory to string")
        ));
    }

    let script_file_name = script_path
        .iter()
        .last()
        .ok_or("Last item in path must exists")?
        .to_string_lossy()
        .to_string();
    if !script_file_name.ends_with(".yaml") {
        return Err(format!(
            "Script path \"{}\" must be a .yaml file",
            script_path
                .to_str()
                .expect("Not able to convert file to string")
        ));
    }
    Ok(())
}

fn get_expected_precondition_or_steps_after_type_found_nothing_error() -> String {
    format!(
        "Expected \"{}\" or \"{}\" after \"{}\" declaration, found nothing",
        PRECONDITION_STR, PROGRAM_STEPS_STR, PROGRAM_TYPE_STR
    )
}

fn get_expected_precondition_or_steps_after_type_but_found_error(key: String) -> String {
    format!(
        "Expected \"{}\" or \"{}\" after \"{}\" declaration, found \"{}\"",
        PRECONDITION_STR, PROGRAM_STEPS_STR, PROGRAM_TYPE_STR, key
    )
}

fn get_expected_precondition_or_steps_after_type_mapping_error(third_node: &YamlNode) -> String {
    format!(
        "Expected \"{}\" or \"{}\" after \"{}\" declaration, found invalid mapping \"{}\"",
        PRECONDITION_STR,
        PROGRAM_STEPS_STR,
        PROGRAM_TYPE_STR,
        third_node.get_content()
    )
}

fn get_expected_steps_after_precondition_found_nothing_error() -> String {
    format!(
        "Expected \"{}\" after precondition declaration, found nothing",
        PROGRAM_STEPS_STR
    )
}

fn get_expected_steps_after_precondition_but_found_error(value_node: &YamlNode) -> String {
    format!(
        "Expected \"{}\" after \"{}\" declaration, found \"{}\"",
        PROGRAM_STEPS_STR,
        PRECONDITION_STR,
        value_node.get_content()
    )
}

fn get_expected_steps_after_precondition_error() -> String {
    format!(
        "Expected \"{}\" after \"{}\" declaration, found invalid format",
        PROGRAM_STEPS_STR, PRECONDITION_STR
    )
}

fn get_unexpected_node_after_run_error_msg() -> String {
    format!(
        "Unexpected node after \"{}\" step, invalid format",
        PROGRAM_STEPS_STR
    )
}

fn get_script(
    script_type: RecipeType,
    precondition_block: Option<&YamlNode>,
    run_block: &YamlNode,
) -> Result<Recipe, Vec<String>> {
    let script = match script_type {
        RecipeType::Java => Recipe::Java(JavaRecipe::new(precondition_block, run_block)?),
    };

    Ok(script)
}

fn get_unexpected_type_message() -> String {
    format!("Expected \"{}: <script_type>\" at second line (where <script_type> is one of {}), found nothing", PROGRAM_TYPE_STR, RecipeType::get_all_types_set_str()).to_string()
}
