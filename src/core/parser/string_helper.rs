use std::str;

use regex::{Match, Regex};

pub fn escape_str_for_json(input_string: String) -> String {
    let mut return_value = "".to_string();

    let lines_count = input_string.lines().count();
    if lines_count == 1 {
        return escape_str_line_internal(&input_string);
    }
    for (index, line) in input_string.lines().enumerate() {
        if index == 0 {
            return_value.push('[');
            return_value.push_str(&escape_str_line_with_quotes(line));
        } else if index == (lines_count - 1) {
            return_value.push_str(&escape_str_line_with_quotes(line));
            return_value.push(']');
        } else {
            return_value.push_str(&escape_str_line_with_quotes(line));
        }
        if index < (lines_count - 1) {
            return_value.push(',');
        }
    }

    return_value
}

pub fn trim_quotation_marks(string: String) -> String {
    if string.is_empty() {
        return string;
    }

    let initial_offset = if string.starts_with('\"') { 1 } else { 0 };
    let last_offset = if string.ends_with('\"') { 1 } else { 0 };
    let range_last = string.len() - last_offset;
    string[initial_offset..range_last].to_string()
}

fn escape_str_line_internal(line: &str) -> String {
    line.to_string().replace('\"', "\\\"")
}

fn escape_str_line_with_quotes(line: &str) -> String {
    format!("\\\"{}\\\"", escape_str_line_internal(line))
}

pub fn to_medial_case(upper_camel_case_str: &str) -> String {
    let mut result = "".to_string();
    let matches = find_word_matches(&upper_camel_case_str);
    for word in matches.into_iter() {
        result.push_str(first_letter_uppercase(word.as_str().to_lowercase()).as_str());
    }

    first_letter_lowercase(result)
}

fn first_letter_lowercase(input: String) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn first_letter_uppercase(input: String) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_lowercase_with_hyphens(upper_camel_case_str: String) -> String {
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

pub fn to_lowercase_space_separated(upper_camel_case_str: String) -> String {
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

fn find_word_matches(upper_camel_case_str: &str) -> Vec<Match> {
    let mut matches = Vec::new();
    let re = Regex::new(r"([A-Z][a-z]+)").expect("Unable to create regex pattern");
    for field in re.find_iter(&upper_camel_case_str) {
        matches.push(field);
    }
    matches
}

pub(crate) fn to_str(buf: &Vec<u8>) -> String {
    let s = match str::from_utf8(&*buf) {
        Ok(content) => content,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_string()
}

#[cfg(test)]
mod tests {
    use crate::core::parser::string_helper;

    #[test]
    fn trim_quotation_marks_tests() {
        assert_eq!("", string_helper::trim_quotation_marks("".to_string()));
        assert_eq!(
            "abc",
            string_helper::trim_quotation_marks("\"abc\"".to_string())
        );
        assert_eq!(
            "a\"b\"c",
            string_helper::trim_quotation_marks("\"a\"b\"c\"".to_string())
        );
    }

    #[test]
    fn to_medial_case_test() {
        assert_eq!("medialCase", string_helper::to_medial_case("MedialCase"));
    }

    #[test]
    fn to_str_from_byte_vector() {
        let example = "StringExample";
        assert_eq!(example, string_helper::to_str(&example.as_bytes().to_vec()));
    }
}
