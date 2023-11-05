use crate::core::parser::parser_node_trait::ParserNode;
use crate::yaml::parser::dto::yaml_node::YamlNode;

pub(crate) struct JavaRecipePrecondition {
    language: Option<String>,
    dependency: Option<String>,
}

const JAVA_VERSION_STR: &str = "java";
const JAVA_DEPENDENCY_STR: &str = "dependency";

// Public crate methods
impl JavaRecipePrecondition {
    pub(crate) fn new(_nodes: &YamlNode) -> Result<Self, Vec<String>> {
        Self::new_internal(_nodes)
    }
}

// Private methods
impl JavaRecipePrecondition {
    fn new_internal(_nodes: &YamlNode) -> Result<JavaRecipePrecondition, Vec<String>> {
        let block_mapping_pairs = _nodes
            .get_children()
            .get(0)
            .ok_or(vec!["Not possible to read java precondition".to_string()])?
            .get_children();

        if block_mapping_pairs.is_empty() {
            let error_str = "Expected java precondition(s), found nothing".to_string();
            return Err(vec![error_str]);
        }

        let mut java_version = None;
        let mut java_dependency = None;
        if let Some(first_pair) = block_mapping_pairs.get(0) {
            if let Some((key, value)) = first_pair.get_block_mapping_pair_string_to_block() {
                if key == JAVA_VERSION_STR {
                    java_version = Some(value.get_content());
                } else if key == JAVA_DEPENDENCY_STR {
                    let java_dependency_result = get_java_dependency(value).map_err(|e| vec![e])?;
                    java_dependency = Some(java_dependency_result);
                } else {
                    let error_str = format!(
                        "Unexpected java precondition key \"{}\", expected one of {{{},{}}}",
                        key, JAVA_VERSION_STR, JAVA_DEPENDENCY_STR
                    )
                    .to_string();
                    return Err(vec![error_str]);
                }
            } else {
                let error_str = "Unexpected java precondition block".to_string();
                return Err(vec![error_str]);
            }
        }
        if let Some(second_pair) = block_mapping_pairs.get(1) {
            if let Some((key, value)) = second_pair.get_block_mapping_pair_string_to_block() {
                if key == JAVA_DEPENDENCY_STR {
                    let java_dependency_result = get_java_dependency(value).map_err(|e| vec![e])?;
                    java_dependency = Some(java_dependency_result);
                } else {
                    let error_str = format!(
                        "Unexpected java precondition key \"{}\", expected \"{}\" or none",
                        key, JAVA_DEPENDENCY_STR
                    )
                    .to_string();
                    return Err(vec![error_str]);
                }
            }
        }

        Ok(JavaRecipePrecondition {
            language: java_version,
            dependency: java_dependency,
        })
    }
}

fn get_java_dependency(node: &YamlNode) -> Result<String, String> {
    Ok(node.get_content())
}
