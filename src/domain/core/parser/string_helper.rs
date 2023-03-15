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

#[cfg(test)]
mod tests {
    use crate::domain::core::parser::string_helper::trim_quotation_marks;

    #[test]
    fn trim_quotation_marks_tests() {
        assert_eq!("", trim_quotation_marks("".to_string()));
        assert_eq!("abc", trim_quotation_marks("\"abc\"".to_string()));
        assert_eq!("a\"b\"c", trim_quotation_marks("\"a\"b\"c\"".to_string()));
    }
}
