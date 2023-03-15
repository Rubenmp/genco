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
    content_pattern: Vec<u8>,
    bytes: Vec<(usize, usize)>,
    value: Option<String>,
}

impl UserInputVariable {
    pub fn new(content_pattern: Vec<u8>, byte_indexes: (usize, usize)) -> Self {
        let bytes = vec![byte_indexes];

        UserInputVariable {
            content_pattern,
            bytes,
            value: None,
        }
    }

    pub fn get_variable_id(&self) -> Vec<u8> {
        let start_index = get_start_variable_pattern().as_bytes().len();
        let end_index =
            self.content_pattern.clone().len() - get_end_variable_pattern().as_bytes().len();
        self.content_pattern.clone()[start_index..end_index].to_owned()
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

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    pub fn set_value(&mut self, content: String) {
        self.value = Some(content);
    }
}

fn get_variables(file: &PathBuf) -> HashMap<Vec<u8>, UserInputVariable> {
    let mut result: HashMap<Vec<u8>, UserInputVariable> = HashMap::new();
    let file_content = fs::read_to_string(file)
        .expect(format!("Error reading file {}", file.to_string_lossy()).as_ref());

    let mut start_index = 0;
    while let Some(next_indexes) = find_next_variable(&file_content, start_index) {
        let user_input_var = get_user_input_variable(file, next_indexes);
        let user_input_var_id = user_input_var.get_variable_id();
        if let Some(input_variable) = result.get_mut(&user_input_var_id) {
            input_variable.add_bytes_indexes(next_indexes);
        } else {
            result.insert(user_input_var_id, user_input_var);
        }

        start_index = next_indexes.1;
    }

    result
}

fn get_user_input_variable(file: &PathBuf, var_def_bytes: (usize, usize)) -> UserInputVariable {
    let bytes = read_bytes(file, var_def_bytes.0, var_def_bytes.1);
    UserInputVariable::new(bytes, var_def_bytes)
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

#[cfg(test)]
mod tests {
    use crate::domain::core::user_input::user_input_handler::UserInputVariable;

    #[test]
    fn user_input_variable_new_test() {
        let variable = "#{input_var_id}#";

        let user_var = UserInputVariable::new(Vec::from(variable), (0, 1));

        assert_eq!(user_var.get_variable_id(), Vec::from("input_var_id"))
    }
}
