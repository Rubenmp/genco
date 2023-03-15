use crate::domain::core::file_system::file_creator::{
    create_file_if_not_exist, create_file_with_content,
};
use crate::domain::core::file_system::file_overwriting::FileOverwriting;
use crate::domain::core::file_system::file_reader::read_bytes;
use crate::domain::core::user_input::cli_query;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct UserInput {
    variables: HashMap<Vec<u8>, UserInputVariable>,
    file: PathBuf,
}

impl UserInput {
    pub fn new(file: &PathBuf) -> Self {
        UserInput {
            variables: get_variables(file),
            file: file.to_owned(),
        }
    }

    pub(crate) fn request_missing_user_input(&mut self) {
        for (variable_id, input_variable) in self.get_variables_mut() {
            let variable_id_str = String::from_utf8(variable_id.clone()).unwrap();
            let user_input = cli_query::ask_input(
                format!(
                    "Please, insert the content for variable \"{}\":",
                    variable_id_str
                )
                .as_str(),
            );
            input_variable.set_value(user_input);
        }
    }

    pub(crate) fn write_output(&self, output_file: &PathBuf) {
        let mut file_overwriting = self.generate_file_overwriting(output_file);
        file_overwriting.write_all_to_file(output_file);
    }

    fn generate_file_overwriting(&self, output_file: &PathBuf) -> FileOverwriting {
        create_file_with_content(output_file, self.get_file());
        let mut result = FileOverwriting::new(output_file);
        for variable in self.variables.values() {
            let bytes_array = variable.get_bytes_array();
            if let Some(value) = variable.get_value() {
                for &bytes in bytes_array {
                    result.replace(bytes.0, bytes.1, value.to_string());
                }
            }
        }

        result
    }

    pub(self) fn get_variables_mut(&mut self) -> &mut HashMap<Vec<u8>, UserInputVariable> {
        &mut self.variables
    }

    pub fn get_file(&self) -> &PathBuf {
        &self.file
    }
}

struct UserInputVariable {
    content_pattern: String,
    bytes: Vec<(usize, usize)>,
    value: Option<String>,
}

impl UserInputVariable {
    pub fn new(byte_indexes: (usize, usize)) -> Self {
        let bytes = vec![byte_indexes];

        UserInputVariable {
            content_pattern: "".to_string(),
            bytes,
            value: None, //Some("test_topic".to_string()),
        }
    }

    pub fn add_bytes_indexes(&mut self, byte_indexes: (usize, usize)) {
        self.bytes.push(byte_indexes);
    }

    pub fn get_bytes_array(&self) -> &Vec<(usize, usize)> {
        &self.bytes
    }

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    pub fn set_value(&mut self, content: String) {
        self.value = Some(content);
    }
}

struct FileInputVariable {
    content_pattern: Vec<u8>,
}

impl FileInputVariable {
    pub fn new(content_pattern: Vec<u8>) -> Self {
        FileInputVariable { content_pattern }
    }

    pub fn get_variable_id(&self) -> Vec<u8> {
        let start_index = get_start_variable_pattern().as_bytes().len();
        let end_index =
            self.content_pattern.clone().len() - get_end_variable_pattern().as_bytes().len();
        self.content_pattern.clone()[start_index..end_index].to_owned()
    }
}

fn get_variables(file: &PathBuf) -> HashMap<Vec<u8>, UserInputVariable> {
    let mut result: HashMap<Vec<u8>, UserInputVariable> = HashMap::new();
    let file_content = fs::read_to_string(file)
        .expect(format!("Error reading file {}", file.to_string_lossy()).as_ref());

    let mut start_index = 0;
    while let Some(next_indexes) = find_next_variable(&file_content, start_index) {
        let var_identifier = get_var_identifier(file, next_indexes).get_variable_id();
        if let Some(input_variable) = result.get_mut(&var_identifier) {
            input_variable.add_bytes_indexes(next_indexes);
        } else {
            let new_input_variable = UserInputVariable::new(next_indexes);
            result.insert(var_identifier, new_input_variable);
        }

        start_index = next_indexes.1;
    }

    result
}

fn get_var_identifier(file: &PathBuf, var_def_bytes: (usize, usize)) -> FileInputVariable {
    let bytes = read_bytes(file, var_def_bytes.0, var_def_bytes.1);
    FileInputVariable::new(bytes)
}

fn find_next_variable(file_content: &String, initial_index: usize) -> Option<(usize, usize)> {
    if let Some(start_index) = file_content[initial_index..]
        .find(get_start_variable_pattern())
        .map(|i| i + initial_index)
    {
        let end_variable_pattern = get_end_variable_pattern();
        if let Some(end_index) = file_content[start_index..]
            .find(end_variable_pattern)
            .map(|i| i + start_index)
        {
            return Some((start_index, end_index + end_variable_pattern.len()));
        }
    }

    None
}

fn get_start_variable_pattern<'a>() -> &'a str {
    "#{"
}

fn get_end_variable_pattern<'a>() -> &'a str {
    "}#"
}
