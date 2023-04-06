use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::core::file_system::file_creator::file_creator::create_file_with_content;
use crate::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::core::file_system::file_reader::read_string;
use crate::core::user_input::cli_query;
use crate::core::user_input::user_input_function::UserInputFunction;

pub struct UserInput {
    file: PathBuf,
    variables: HashMap<String, Vec<VariableInstantiation>>,
}

impl UserInput {
    pub fn new(file: &PathBuf) -> Self {
        UserInput {
            file: file.to_owned(),
            variables: get_variables(file),
        }
    }

    pub fn request_missing_user_input(&mut self) {
        for (variable_id, variable_usages) in self.get_variables_mut() {
            let user_input = cli_query::ask_input(
                format!(
                    "Please, insert the content for variable \"{}\":",
                    variable_id
                )
                .as_str(),
            );

            for variable_usage in variable_usages {
                variable_usage.set_raw_user_input_value(user_input.to_string());
            }
        }
    }

    pub fn write_output(&self, output_file: &PathBuf) {
        let mut file_overwriting = self.generate_file_overwriting(output_file);
        file_overwriting.write_all_to_file(output_file);
    }

    fn generate_file_overwriting(&self, output_file: &PathBuf) -> FileOverwriting {
        create_file_with_content(output_file, self.get_file());
        let mut result = FileOverwriting::new(output_file);
        for variable_usages in self.get_variables().values() {
            for usage in variable_usages {
                let bytes_array = usage.get_bytes_array();
                if let Some(value) = usage.get_value() {
                    for &bytes in bytes_array {
                        result.replace(bytes.0, bytes.1, &value);
                    }
                }
            }
        }

        result
    }

    pub fn override_value(&mut self, variable_id: &str, variable_value: &str) {
        if let Some(variables) = self.variables.get_mut(variable_id) {
            for variable in variables {
                variable.override_value(variable_value);
            }
        }
    }

    pub(self) fn get_variables_mut(&mut self) -> &mut HashMap<String, Vec<VariableInstantiation>> {
        &mut self.variables
    }

    fn get_variables(&self) -> &HashMap<String, Vec<VariableInstantiation>> {
        &self.variables
    }

    pub fn get_file(&self) -> &PathBuf {
        &self.file
    }
}

#[derive(Debug)]
struct VariableUsage {
    usages: Vec<VariableInstantiation>,
}

#[derive(Debug)]
struct VariableInstantiation {
    raw_var_pattern: String,
    raw_user_input_value: Option<String>,
    var_id: String,
    override_value: Option<String>,
    bytes: Vec<(usize, usize)>,
    function: Option<UserInputFunction>,
}

impl VariableInstantiation {
    pub fn new(raw_var_pattern: String) -> Self {
        let parsed_value = parse_user_input_var(&raw_var_pattern);

        VariableInstantiation {
            raw_var_pattern,
            raw_user_input_value: None,
            var_id: parsed_value.0.unwrap(),
            override_value: None,
            bytes: Vec::new(),
            function: parsed_value.1,
        }
    }

    pub fn override_value(&mut self, variable_value: &str) {
        self.override_value = Some(variable_value.to_string());
    }

    pub fn get_variable_id(&self) -> String {
        self.var_id.to_owned()
    }

    pub fn add_bytes_indexes(&mut self, byte_indexes: (usize, usize)) {
        if byte_indexes.0 >= byte_indexes.1 {
            panic!("Bytes added to user input must have start_byte less than end_byte");
        }
        self.bytes.push(byte_indexes);
    }

    pub fn get_bytes_array(&self) -> &Vec<(usize, usize)> {
        &self.bytes
    }

    pub fn get_content_pattern(&self) -> &str {
        &self.raw_var_pattern
    }

    pub fn get_value(&self) -> Option<String> {
        if let Some(override_value) = &self.override_value {
            return Some(override_value.to_owned());
        }

        if let (Some(function_to_apply), Some(user_input)) =
            (&self.function, &self.raw_user_input_value)
        {
            return Some(function_to_apply.apply(user_input.to_string()));
        }

        return self.raw_user_input_value.to_owned();
    }

    pub fn set_raw_user_input_value(&mut self, input: String) {
        self.raw_user_input_value = Some(input);
    }
}

fn parse_user_input_var(pattern: &String) -> (Option<String>, Option<UserInputFunction>) {
    let internal_str = get_internal_string_from_var_pattern(&pattern);
    let type_to_value = split_var_and_type(&pattern, internal_str);

    let mut var_id = None;
    let mut function = None;
    if let Some(var_id_value) = type_to_value.get("var") {
        var_id = Some(var_id_value.to_string());

        if let Some(function_value) = type_to_value.get("function") {
            match UserInputFunction::parse(function_value.to_string()) {
                Ok(returned_function) => function = Some(returned_function),
                Err(e) => panic!("Invalid ParseError: {}", e),
            };
        }
    }

    (var_id, function)
}

fn split_var_and_type(pattern: &String, internal_str: String) -> HashMap<String, String> {
    let mut type_to_value = HashMap::new();
    let split_internal_str = internal_str.split('|');
    for split in split_internal_str {
        let mut type_and_value = split.split("=");
        if type_and_value.clone().count() != (2 as usize) {
            panic!(
                "Invalid parse_user_input_var for pattern \"{}\"",
                pattern.to_string()
            );
        }
        if let Some(var_type) = type_and_value.next() {
            if let Some(var_value) = type_and_value.next() {
                type_to_value.insert(var_type.to_string(), var_value.to_string());
            }
        }
    }

    type_to_value
}

fn get_internal_string_from_var_pattern(pattern: &String) -> String {
    let start_index = get_start_variable_pattern().as_bytes().len();
    let end_index = pattern.len() - get_end_variable_pattern().as_bytes().len();

    pattern[start_index..end_index].to_owned()
}

fn get_variables(file: &PathBuf) -> HashMap<String, Vec<VariableInstantiation>> {
    let mut result: HashMap<String, Vec<VariableInstantiation>> = HashMap::new();
    let file_content = fs::read_to_string(file)
        .expect(format!("Error reading resource {}", file.to_string_lossy()).as_ref());

    let mut start_index = 0;
    while let Some(next_indexes) = find_next_variable(&file_content, start_index) {
        let user_input_var = get_user_input_variable(file, next_indexes);
        let user_input_var_id = user_input_var.get_variable_id();
        if let Some(usages) = result.get_mut(&user_input_var_id) {
            usages.push(user_input_var);
        } else {
            let mut usages = Vec::new();
            usages.push(user_input_var);
            result.insert(user_input_var_id, usages);
        }

        start_index = next_indexes.1;
    }

    result
}

fn get_user_input_variable(file: &PathBuf, var_def_bytes: (usize, usize)) -> VariableInstantiation {
    let content = read_string(file, var_def_bytes.0, var_def_bytes.1);
    let mut input = VariableInstantiation::new(content.to_string());
    input.add_bytes_indexes(var_def_bytes);

    input
}

fn find_next_variable(file_content: &String, initial_index: usize) -> Option<(usize, usize)> {
    let mut start_var_pattern_with_var = get_start_variable_pattern().to_string();
    start_var_pattern_with_var.push_str("var=");
    if let Some(start_index) = find_index(
        file_content,
        initial_index,
        start_var_pattern_with_var.as_str(),
    ) {
        let end_variable_pattern = get_end_variable_pattern();
        if let Some(end_index) = find_index(file_content, start_index, end_variable_pattern) {
            return Some((start_index, end_index + end_variable_pattern.len()));
        }
    }

    None
}

fn find_index(file_content: &String, start_index: usize, pattern: &str) -> Option<usize> {
    file_content[start_index..]
        .find(pattern)
        .map(|i| i + start_index)
}

fn get_start_variable_pattern<'a>() -> &'a str {
    "#{"
}

fn get_end_variable_pattern<'a>() -> &'a str {
    "}#"
}

#[cfg(test)]
mod tests {
    use crate::core::user_input::user_input_handler::VariableInstantiation;

    #[test]
    fn user_input_variable_new_test() {
        let variable = "#{var=input_var_id}#".to_string();

        let user_var = VariableInstantiation::new(variable);

        assert_eq!(user_var.get_variable_id(), "input_var_id".to_string());
    }

    #[test]
    fn user_input_variable_new_with_function_test() {
        let variable =
            "#{var=input_var_id|function=to_lowercase_with_hyphens(input_var_id)}#".to_string();

        let user_var = VariableInstantiation::new(variable);

        assert_eq!(user_var.get_variable_id(), "input_var_id".to_string());
    }
}
