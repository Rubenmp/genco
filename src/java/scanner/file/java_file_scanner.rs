use std::path::Path;

use crate::java::parser::java_parser::java_parser;
use crate::java::scanner::file::dto::java_file::JavaFile;

pub fn scan(java_file_path: &Path) -> Result<JavaFile, String> {
    let java_node = java_parser::parse(java_file_path)?;

    JavaFile::new(java_node)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_test_file;
    use crate::java::scanner::file::java_file_scanner::scan;

    #[test]
    fn scan_basic_application() {
        let dir_path = get_test_file(get_current_file_path(), "BasicApplication.java");

        match scan(&dir_path) {
            Ok(java_file) => {
                assert_eq!("com.org.demo", java_file.get_package().to_string())
            }
            Err(e) => assert_fail(&e),
        }
    }

    #[test]
    fn scan_invalid() {
        let dir_path = get_test_file(get_current_file_path(), "Invalid.java");

        match scan(&dir_path) {
            Ok(java_file) => assert_fail("It should not return a valid java file struct"),
            Err(e) => assert_eq!("Java package not found.", e),
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
