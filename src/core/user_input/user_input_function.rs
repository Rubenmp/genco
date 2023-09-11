use std::string::ParseError;

use regex::{Match, Regex};

use crate::core::parser::string_helper;

#[derive(Debug)]
pub struct UserInputFunction {
    raw_function_pattern: String,
    function_name: String,
    function_parameter: String,
    function_reference: fn(String) -> String,
}

/// Available public UserInputFunctions
pub fn to_lowercase_with_hyphens(upper_camel_case_str: String) -> String {
    string_helper::to_lowercase_with_hyphens(upper_camel_case_str)
}

pub fn to_lowercase_space_separated(upper_camel_case_str: String) -> String {
    string_helper::to_lowercase_space_separated(upper_camel_case_str)
}

pub fn to_medial_case(upper_camel_case_str: String) -> String {
    string_helper::to_medial_case(&upper_camel_case_str)
}

///

fn to_lowercase_with_hyphens_internal(upper_camel_case_str: String) -> String {
    let mut result = "".to_string();
    let matches = find_word_matches(&upper_camel_case_str);
    let matches_count = matches.len();
    for (index, word) in matches.into_iter().enumerate() {
        if index != 0 && index < matches_count {
            result.push_str("-");
        }
        result.push_str(word.as_str().to_lowercase().as_str());
    }

    result
}

pub fn to_lowercase_space_separated_internal(upper_camel_case_str: String) -> String {
    let mut result = "".to_string();
    let matches = find_word_matches(&upper_camel_case_str);
    let matches_count = matches.len();
    for (index, word) in matches.into_iter().enumerate() {
        if index != 0 && index < matches_count {
            result.push_str(" ");
        }
        result.push_str(word.as_str().to_lowercase().as_str());
    }

    result
}

fn find_word_matches(upper_camel_case_str: &String) -> Vec<Match> {
    let mut matches = Vec::new();
    let re = Regex::new(r"([A-Z][a-z]+)").expect("Unable to create regex pattern");
    for field in re.find_iter(&upper_camel_case_str) {
        matches.push(field);
    }
    matches
}

impl UserInputFunction {
    pub fn parse(raw_function_pattern: String) -> Result<Self, ParseError> {
        let mut split_function = raw_function_pattern.split("(");
        if split_function.clone().count() != (2 as usize) {
            panic!("Invalid UserInputFunction \"{}\"", raw_function_pattern);
        }

        if let Some(function_name_str) = split_function.next() {
            let function_name = function_name_str.to_string();
            let function_reference = get_function_reference(&function_name);

            if let Some(params_with_last_parenthesis) = split_function.next() {
                if params_with_last_parenthesis.ends_with(")") {
                    let mut function_parameter = params_with_last_parenthesis.to_string();
                    function_parameter.pop();
                    check_valid_function_string(&function_parameter);
                    return Ok(UserInputFunction {
                        raw_function_pattern,
                        function_name,
                        function_parameter,
                        function_reference,
                    });
                }
            }
        }

        todo!()
    }

    pub fn get_function_name(&self) -> String {
        self.function_name.to_owned()
    }

    pub fn get_function_reference(&self) -> fn(String) -> String {
        self.function_reference
    }

    pub fn apply(&self, input: String) -> String {
        self.get_function_reference()(input)
    }
}

fn check_valid_function_string(_input: &String) {}

fn get_function_reference(function_name: &str) -> fn(String) -> String {
    match function_name {
        "to_lowercase_with_hyphens" => to_lowercase_with_hyphens,
        "to_lowercase_space_separated" => to_lowercase_space_separated,
        "to_medial_case" => to_medial_case,
        _ => panic!("Invalid function \"{}\"", function_name),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::user_input::user_input_function::{
        to_lowercase_space_separated, to_lowercase_with_hyphens, UserInputFunction,
    };

    #[test]
    #[ignore = "User input not yet mocked"]
    fn user_input_function_parse_to_lowercase_with_hyphens() {
        let variable = "to_lowercase_with_hyphens(input_var_id)".to_string();

        let user_input_function = UserInputFunction::parse(variable);

        match user_input_function {
            Ok(returned_function) => {
                assert_eq!(
                    returned_function.get_function_name(),
                    "to_lowercase_with_hyphens"
                );
                assert_eq!(
                    returned_function.apply("UpperCamelCase".to_string()),
                    "upper-camel-case".to_string()
                );
            }
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    #[ignore = "User input not yet mocked"]
    fn to_lowercase_with_hyphens_test() {
        assert_eq!(
            to_lowercase_with_hyphens("UpperCamelCase".to_string()),
            "upper-camel-case".to_string()
        );
    }

    #[test]
    #[ignore = "User input not yet mocked"]
    fn user_input_function_parse_to_lowercase_space_separated() {
        let variable = "to_lowercase_space_separated(input_var_id)".to_string();

        let user_input_function = UserInputFunction::parse(variable);

        match user_input_function {
            Ok(returned_function) => {
                assert_eq!(
                    returned_function.get_function_name(),
                    "to_lowercase_space_separated"
                );
                assert_eq!(
                    returned_function.apply("UpperCamelCase".to_string()),
                    "upper camel case".to_string()
                );
            }
            Err(e) => panic!("Error: {}", e),
        };
    }
    #[test]
    #[ignore = "User input not yet mocked"]
    fn to_lowercase_space_separated_test() {
        assert_eq!(
            to_lowercase_space_separated("UpperCamelCase".to_string()),
            "upper camel case".to_string()
        );
    }
}
