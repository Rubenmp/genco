use std::fs;
use std::path::{Path, PathBuf};

pub fn assert_same_as_file(expect_result_file_path: &Path, result: String) {
    let data = fs::read_to_string(expect_result_file_path)
        .expect("Unable to read expected result resource");

    assert_eq!(result.replace("\r\n", "\n"), data.replace("\r\n", "\n"));
}

pub fn assert_same_file(expect_result_file: &PathBuf, actual_result_file: &PathBuf) {
    let expected_data =
        fs::read_to_string(expect_result_file).expect("Unable to read expected result resource");
    let actual_data =
        fs::read_to_string(actual_result_file).expect("Unable to read expected result resource");

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
