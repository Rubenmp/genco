use std::fs;
use std::path::Path;

use crate::core::file_system::file_reader;

pub fn assert_same_as_file(expect_result_file_path: &Path, result: &str) {
    let data = fs::read_to_string(expect_result_file_path).expect(
        format!(
            "Unable to read expected result file:\n{}\n",
            expect_result_file_path.to_string_lossy()
        )
        .as_str(),
    );

    assert_eq!(result.replace("\r\n", "\n"), data.replace("\r\n", "\n"));
}

pub fn assert_same_file(expect_result_file: &Path, actual_result_file: &Path) {
    assert!(actual_result_file.exists());
    assert!(actual_result_file.is_file());
    let expected_data = file_reader::read_to_string(expect_result_file);
    let actual_data = file_reader::read_to_string(actual_result_file);

    assert_eq!(
        actual_data.replace("\r\n", "\n"),
        expected_data.replace("\r\n", "\n")
    );
}

pub fn assert_file_is(result: &Path, expected_file: &str) {
    assert!(result.exists());
    assert!(result.is_file());
    if let Some(file_str) = result.iter().map(|i| i.to_str()).last() {
        assert_eq!(Some(expected_file), file_str);
    } else {
        assert!(false, "Expected \"{}\" resource not found.", expected_file);
    }
}

pub fn assert_dir_is(result: &Path, expected_file: &str) {
    assert!(result.exists());
    assert!(result.is_dir());
    if let Some(dir_entry_str) = result.iter().map(|i| i.to_str()).last() {
        assert_eq!(Some(expected_file), dir_entry_str);
    } else {
        assert!(false, "Expected \"{}\" directory not found.", expected_file);
    }
}

pub fn assert_fail(error_message: &str) {
    assert!(false, "{}", error_message);
}
