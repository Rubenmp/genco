use std::fs;
use std::path::PathBuf;

pub fn assert_same_as_file(expect_result_file_path: PathBuf, result: String) {
    let data = fs::read_to_string(expect_result_file_path).expect("Unable to read expected result file");

    assert_eq!(result.replace("\r\n", "\n"), data.replace("\r\n", "\n"));
}

pub fn assert_same_file(expect_result_file: &PathBuf, actual_result_file: &PathBuf) {
    let expected_data = fs::read_to_string(expect_result_file).expect("Unable to read expected result file");
    let actual_data = fs::read_to_string(actual_result_file).expect("Unable to read expected result file");

    assert_eq!(actual_data.replace("\r\n", "\n"), expected_data.replace("\r\n", "\n"));
}
