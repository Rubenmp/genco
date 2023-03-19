use crate::domain::usecase::java::scanner::dto::java_project_scan::JavaProjectScan;
use std::path::Path;

pub fn scan_java_project(path: &Path) -> JavaProjectScan {
    JavaProjectScan::new(path)
}

#[cfg(test)]
mod tests {
    use crate::domain::core::testing::test_path::get_test_dir;
    use crate::domain::usecase::java::scanner::java_scanner::java_scanner::scan_java_project;
    use std::path::{Path, PathBuf};

    #[test]
    fn scan_java_project_test() {
        let dir_path = get_test_dir(get_current_file_path(), "basic_project");

        let project_scan = scan_java_project(&dir_path);

        assert_eq!(1, project_scan.get_files_cache().count_files_added());
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
