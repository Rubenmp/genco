#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::Path;

#[cfg(test)]
use crate::core::file_system::file_reader;
#[cfg(test)]
use crate::core::parser::string_helper;

#[cfg(test)]
pub(crate) fn assert_same_bytes_than_file(expect_result_file_path: &Path, input_bytes: &[u8]) {
    let input_str = string_helper::to_str(input_bytes);
    assert_same_as_file(expect_result_file_path, &input_str);
}

#[cfg(test)]
pub(crate) fn assert_same_as_file(expect_result_file_path: &Path, result: &str) {
    let data = fs::read_to_string(expect_result_file_path).unwrap_or_else(|_| {
        panic!(
            "Unable to read expected result file:\n{}\n",
            expect_result_file_path.to_string_lossy()
        )
    });

    assert_eq!(result.replace("\r\n", "\n"), data.replace("\r\n", "\n"));
}

#[cfg(test)]
pub(crate) fn assert_same_file(expect_result_file: &Path, actual_result_file: &Path) {
    assert!(actual_result_file.exists());
    assert!(actual_result_file.is_file());
    let expected_data = file_reader::read_to_string(expect_result_file);
    let actual_data = file_reader::read_to_string(actual_result_file);

    assert_eq!(
        actual_data.replace("\r\n", "\n"),
        expected_data.replace("\r\n", "\n")
    );
}

#[cfg(test)]
pub(crate) fn assert_file_is(result: &Path, expected_file: &str) {
    assert!(result.exists());
    assert!(result.is_file());
    if let Some(file_str) = result.iter().map(|i| i.to_str()).last() {
        assert_eq!(Some(expected_file), file_str);
    } else {
        assert_fail(format!("Expected \"{}\" resource not found.", expected_file).as_str());
    }
}

#[cfg(test)]
pub(crate) fn assert_dir_is(result: &Path, expected_file: &str) {
    assert!(result.exists());
    assert!(result.is_dir());
    if let Some(dir_entry_str) = result.iter().map(|i| i.to_str()).last() {
        assert_eq!(Some(expected_file), dir_entry_str);
    } else {
        assert_fail(format!("Expected \"{}\" directory not found.", expected_file).as_str());
    }
}

#[cfg(test)]
pub(crate) fn assert_fail(error_message: &str) {
    panic!("{}", error_message);
}
