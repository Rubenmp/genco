use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::core::file_system::directory_browser::directory_browser;
use crate::core::file_system::file_browser::file_browser;
use crate::java::scanner::file::java_file_scanner;
use crate::java::scanner::project::dto::java_project_files_cache::JavaProjectFilesCache;

pub struct JavaProjectScan {
    files_cache: JavaProjectFilesCache,
}

impl JavaProjectScan {
    pub fn new(path: &Path) -> Self {
        check_base_java_project(path);
        let mut scan = JavaProjectScan {
            files_cache: JavaProjectFilesCache::new(),
        };

        recursive_scan(path, &mut scan);

        scan
    }

    pub fn get_files_cache(&self) -> &JavaProjectFilesCache {
        &self.files_cache
    }
}

fn recursive_scan(path: &Path, scan: &mut JavaProjectScan) {
    let file_map = file_browser::get_file_map(path);
    for (file_name, file_path) in file_map {
        if is_java_file(file_name) {
            scan_file(&file_path, scan);
        }
    }
    let dir_map = directory_browser::get_dir_map(path);
    for dir_path in dir_map.values() {
        if should_scan_dir(&dir_path) {
            recursive_scan(&dir_path, scan);
        }
    }
}

fn is_java_file(file_name: String) -> bool {
    file_name.ends_with(".java")
}

fn should_scan_dir(dir_path: &Path) -> bool {
    let last_folder = dir_path.iter().last();
    if dir_path.ends_with("java") {
        return !dir_path.ends_with("src/test/java");
    } else if dir_path.ends_with("target") {
        //     } else if HashSet::from(["target", ".mvn"]).contains("TODCHANGE") {
        return false;
    }

    true
}

fn scan_file(java_file: &Path, scan: &mut JavaProjectScan) {
    scan.files_cache.try_to_add_file(java_file);
}

fn get_src_main_java_dir(path: &Path) -> Option<PathBuf> {
    if let Some(src_dir) = directory_browser::get_dir(path, "src") {
        if let Some(main_dir) = directory_browser::get_dir(&src_dir, "main") {
            if let Some(java_dir) = directory_browser::get_dir(&main_dir, "java") {
                return Some(java_dir);
            }
        }
    }

    None
}

fn check_base_java_project(path: &Path) {
    let mut files = Vec::new();
    files.push("build.gradle");
    files.push("pom.xml");

    if file_browser::get_first_file_if_exists(&path, files).is_none() {
        panic!("Invalid java project root path found: {:?}", path);
    }
}
