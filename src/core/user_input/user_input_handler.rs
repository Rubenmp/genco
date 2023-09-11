use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::file_system::file_creator::file_creator::create_file_with_content;
use crate::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::core::file_system::file_reader::read_string;
use crate::core::user_input::cli_query;

#[derive(Debug)]
pub struct UserInput {
    variables: HashMap<String, VariableUsage>,
    config_files_used: HashSet<String>,
}

impl UserInput {
    pub fn new_empty() -> Self {
        UserInput {
            variables: HashMap::new(),
            config_files_used: HashSet::new(),
        }
    }

    pub fn new(file: &PathBuf) -> Self {
        let mut new_entity = UserInput::new_empty();
        new_entity.add_variables_from(file);
        new_entity
    }

    pub fn add_variables_from(&mut self, new_file: &Path) {
        let new_file_str = new_file.to_string_lossy().to_string();
        if !self.config_files_used.contains(&new_file_str) {
            let new_user_input = get_variables(new_file);
            self.merge(&new_user_input);
            self.config_files_used.insert(new_file_str);
        }
    }

    pub fn request_missing_user_input(&mut self) {
        for (variable_id, variable_usage) in self.get_variables_mut() {
            let user_input = cli_query::ask_input(
                format!(
                    "Please, insert the content for variable \"{}\":",
                    variable_id
                )
                .as_str(),
            );

            variable_usage.set_raw_user_input_value(user_input.to_string());
        }
    }

    pub fn write_output(&self, input_file: &PathBuf, output_file: &PathBuf) {
        let mut file_overwriting = self.generate_file_overwriting(input_file, output_file);
        file_overwriting.write_all_to_file(output_file);
    }

    // TODO: update structure (?)
    fn generate_file_overwriting(
        &self,
        input_file: &PathBuf,
        output_file: &PathBuf,
    ) -> FileOverwriting {
        create_file_with_content(output_file, input_file);
        let mut result = FileOverwriting::new(output_file);
        for variable_usage in self.get_variables().values() {
            for instantiation in variable_usage.get_instantiations() {
                let bytes = instantiation.get_bytes();
                if let Some(value) = variable_usage.get_value() {
                    result.replace(bytes.0, bytes.1, &value);
                }
            }
        }

        result
    }

    pub fn override_value(&mut self, variable_id: &str, variable_value: &str) {
        if let Some(variable) = self.variables.get_mut(variable_id) {
            variable.override_value(variable_value);
        } else {
            panic!("Variable to override value not found \"{}\"", variable_id);
        }
    }

    pub(self) fn get_variables_mut(&mut self) -> &mut HashMap<String, VariableUsage> {
        &mut self.variables
    }

    fn get_variables(&self) -> &HashMap<String, VariableUsage> {
        &self.variables
    }

    fn merge(&mut self, var_usgaes: &HashMap<String, VariableUsage>) {
        for (var_id, var_usage) in var_usgaes.clone() {
            if let Some(match_usage) = self.variables.get_mut(&var_id) {
                match_usage.merge(&var_usage);
            } else {
                self.variables
                    .insert(var_id.to_owned(), var_usage.to_owned());
            }
        }
    }
}

#[derive(Debug, Clone)]
struct VariableUsage {
    raw_user_input_value: Option<String>,
    var_id: String,
    override_value: Option<String>,
    instantiations: Vec<VariableInstantiation>,
}

impl VariableUsage {
    pub fn new(
        raw_var_pattern: String,
        var_def_bytes: (usize, usize),
        file: &Path,
    ) -> Option<Self> {
        let var_name_opt = parse_user_input_var(&raw_var_pattern);
        let mut instantiations = Vec::new();
        instantiations.push(VariableInstantiation::new(file, var_def_bytes));
        if let Some(var_name) = var_name_opt {
            return Some(VariableUsage {
                raw_user_input_value: None,
                var_id: var_name,
                override_value: None,
                instantiations,
            });
        }

        None
    }

    pub(crate) fn merge(&mut self, new_usage: &VariableUsage) {
        for instantiation in new_usage.get_instantiations() {
            self.add_instantiation(instantiation);
        }
    }

    pub fn override_value(&mut self, variable_value: &str) {
        self.override_value = Some(variable_value.to_string());
    }

    pub fn get_variable_id(&self) -> String {
        self.var_id.to_owned()
    }

    fn add_instantiation(&mut self, instantiation: &VariableInstantiation) {
        let file = &instantiation.file;
        let byte_indexes = &instantiation.bytes;
        if byte_indexes.0 >= byte_indexes.1 {
            panic!("Bytes added to user input must have start_byte less than end_byte");
        }
        self.instantiations
            .push(VariableInstantiation::new(file, *byte_indexes));
    }

    pub fn get_value(&self) -> Option<String> {
        if let Some(override_value) = &self.override_value {
            return Some(override_value.to_owned());
        }

        self.raw_user_input_value.to_owned()
    }

    pub fn get_instantiations(&self) -> &Vec<VariableInstantiation> {
        &self.instantiations
    }

    pub fn set_raw_user_input_value(&mut self, input: String) {
        self.raw_user_input_value = Some(input);
    }
}

#[derive(Debug, Clone)]
struct VariableInstantiation {
    file: PathBuf,
    bytes: (usize, usize),
}

impl VariableInstantiation {}

impl VariableInstantiation {
    pub fn new(file_path: &Path, bytes: (usize, usize)) -> Self {
        Self::check_var_instantiation(file_path, bytes);
        VariableInstantiation {
            file: file_path.to_owned(),
            bytes,
        }
    }

    fn get_file(&self) -> &PathBuf {
        &self.file
    }

    fn get_bytes(&self) -> (usize, usize) {
        self.bytes
    }

    fn check_var_instantiation(file: &Path, bytes: (usize, usize)) {
        if !file.exists() || !file.is_file() {
            panic!("Trying to create VariableInstantiation with invalid file")
        }
        if bytes.0 > bytes.1 {
            panic!(
                "Trying to create VariableInstantiation with invalid bytes ({},{})",
                bytes.0, bytes.1
            );
        }
    }
}

fn parse_user_input_var(pattern: &String) -> Option<String> {
    let internal_str = get_internal_string_from_var_pattern(pattern);
    get_var_name(pattern, internal_str)
}

fn get_var_name(pattern: &String, internal_str: String) -> Option<String> {
    let mut type_and_value = internal_str.split('=');
    if type_and_value.clone().count() != 2_usize {
        panic!("Invalid parse_user_input_var for pattern \"{}\"", pattern);
    }
    if let Some(var_type) = type_and_value.next() {
        if var_type != "var" {
            return None;
        }
        if let Some(var_value) = type_and_value.next() {
            return Some(var_value.to_string());
        }
    }

    None
}

fn get_internal_string_from_var_pattern(pattern: &String) -> String {
    let start_index = get_start_variable_pattern().as_bytes().len();
    let end_index = pattern.len() - get_end_variable_pattern().as_bytes().len();

    pattern[start_index..end_index].to_owned()
}

fn get_variables(file: &Path) -> HashMap<String, VariableUsage> {
    let mut result: HashMap<String, VariableUsage> = HashMap::new();
    let file_content = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Error reading resource {}", file.to_string_lossy()));

    let mut start_index = 0;
    while let Some(next_indexes) = find_next_variable(&file_content, start_index) {
        if let Some(var_usage) = get_var_usage(file, next_indexes) {
            let var_id = var_usage.get_variable_id();
            if let Some(current_usage) = result.get_mut(&var_id) {
                current_usage.merge(&var_usage);
            } else {
                result.insert(var_id, var_usage);
            }
        } else {
            panic!("Invalid var_usage in file {:?}", file);
        }
        start_index = next_indexes.1;
    }

    result
}

fn get_var_usage(file: &Path, var_def_bytes: (usize, usize)) -> Option<VariableUsage> {
    let content = read_string(file, var_def_bytes.0, var_def_bytes.1);
    VariableUsage::new(content.to_string(), var_def_bytes, file)
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
    use std::path::PathBuf;

    use crate::core::testing::test_path::get_test_file;
    use crate::core::user_input::user_input_handler::{UserInput, VariableInstantiation};

    #[test]
    #[ignore = "User input not yet mocked"]
    fn user_input_new() {
        let test_file = get_test_file(get_current_file_path(), "user_input_var_input_var_id.txt");

        let user_var = UserInput::new(&test_file);

        let usages = user_var.get_variables();
        assert_eq!(1, usages.len());
        for (var_id, var_usage) in usages.iter() {
            assert_eq!("input_var_id".to_string(), var_id.to_owned());
            let instantiations = var_usage.get_instantiations();
            assert_eq!(1, instantiations.len());
            if let Some(instantiation) = instantiations.get(0) {
                check_instantiation(&test_file, instantiation, (0, 20));
            }
        }
    }

    #[test]
    #[ignore = "User input not yet mocked"]
    fn user_input_merge_same_variable() {
        let current_file_path = get_current_file_path();
        let first_test_file = get_test_file(
            current_file_path.to_owned(),
            "user_input_var_input_var_id.txt",
        );
        let second_test_file = get_test_file(
            current_file_path.to_owned(),
            "user_input_var_input_var_id_copy.txt",
        );

        let mut user_var = UserInput::new(&first_test_file);
        user_var.add_variables_from(&second_test_file);

        let usages = user_var.get_variables();
        assert_eq!(1, usages.len());
        for (var_id, var_usage) in usages.iter() {
            assert_eq!("input_var_id".to_string(), var_id.to_owned());
            let instantiations = var_usage.get_instantiations();
            assert_eq!(2, instantiations.len());
            if let Some(instantiation) = instantiations.get(0) {
                check_instantiation(&first_test_file, instantiation, (0, 20));
            }
            if let Some(instantiation) = instantiations.get(1) {
                check_instantiation(&second_test_file, instantiation, (0, 20));
            }
        }
    }

    #[test]
    #[ignore = "User input not yet mocked"]
    fn user_input_merge_different_variable() {
        let current_file_path = get_current_file_path();
        let first_test_file = get_test_file(
            current_file_path.to_owned(),
            "user_input_var_input_var_id.txt",
        );
        let second_test_file = get_test_file(
            current_file_path.to_owned(),
            "user_input_var_new_var_id.txt",
        );

        let mut user_var = UserInput::new(&first_test_file);
        user_var.add_variables_from(&second_test_file);

        let usages = user_var.get_variables();
        assert_eq!(2, usages.len());
        if let Some(var_usage) = usages.get("input_var_id") {
            let instantiations = var_usage.get_instantiations();
            assert_eq!(1, instantiations.len());
            if let Some(instantiation) = instantiations.get(0) {
                check_instantiation(&first_test_file, instantiation, (0, 20));
            }
        }
        if let Some(var_usage) = usages.get("new_var_id") {
            let instantiations = var_usage.get_instantiations();
            assert_eq!(1, instantiations.len());
            if let Some(instantiation) = instantiations.get(0) {
                check_instantiation(&second_test_file, instantiation, (0, 18));
            }
        }
    }

    fn check_instantiation(
        test_file: &PathBuf,
        instantiation: &VariableInstantiation,
        bytes: (usize, usize),
    ) {
        assert_eq!(bytes, instantiation.get_bytes());
        assert_eq!(
            test_file.to_string_lossy(),
            instantiation.get_file().to_string_lossy()
        );
    }

    pub fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
