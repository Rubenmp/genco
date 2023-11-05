#[cfg(test)]
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) fn get_test_dir(current_file: PathBuf, name: &str) -> PathBuf {
    let mut path = get_test_dir_raw(&current_file);
    path.push(name);

    if !path.exists() {
        panic!("Expected test directory does not exists: {:?}", path);
    } else if !path.is_dir() {
        panic!("Expected test directory is not a directory: {:?}", path);
    }
    path
}

#[cfg(test)]
pub(crate) fn get_test_dir_raw(current_file: &Path) -> PathBuf {
    let mut path = current_file.to_path_buf();
    path.pop();
    path.push("test");
    path
}

#[cfg(test)]
pub(crate) fn get_test_file(current_file: &Path, name: &str) -> PathBuf {
    let path = get_non_existing_test_file(current_file, name);

    if !path.exists() {
        panic!("Expected test resource does not exists: {:?}", path);
    } else if !path.is_file() {
        panic!("Expected test resource is not a resource: {:?}", path);
    }
    path
}

#[cfg(test)]
pub(crate) fn get_non_existing_test_file(current_file: &Path, name: &str) -> PathBuf {
    get_test_dir_raw(current_file).join(name)
}

// Java
#[cfg(test)]
pub(crate) fn get_java_test_file(
    current_file: PathBuf,
    test_folder: &str,
    java_file_name: &str,
) -> PathBuf {
    get_java_project_test_folder(current_file, test_folder).join(java_file_name)
}

#[cfg(test)]
pub(crate) fn get_java_project_test_folder(current_file: PathBuf, test_folder: &str) -> PathBuf {
    let mut path = get_test_dir_raw(&current_file);

    include_path_to_main_java_folder(&mut path, test_folder);

    path
}

#[cfg(test)]
fn include_path_to_main_java_folder(path: &mut PathBuf, test_folder: &str) {
    path.push(test_folder);
    path.push("src");
    path.push("main");
    path.push("java");
    path.push("org");
    path.push("test");
}
