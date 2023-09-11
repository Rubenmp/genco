use crate::java::scanner::project::dto::java_project_scan::JavaProjectScan;
use std::path::Path;

pub fn scan(path: &Path) -> JavaProjectScan {
    JavaProjectScan::new(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_path::get_test_dir;
    use crate::java::scanner::project::java_project_scanner::scan;

    #[test]
    fn scan_java_project_test() {
        let dir_path = get_test_dir(get_current_file_path(), "basic_project");

        let project_scan = scan(&dir_path);

        assert_eq!(1, project_scan.get_files_cache().count_files_added());
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
