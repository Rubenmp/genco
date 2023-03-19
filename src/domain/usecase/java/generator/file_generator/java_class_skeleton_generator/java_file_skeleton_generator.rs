use std::path::Path;

use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;

pub fn generate(dir: &Path, skeleton: JavaClassSkeleton) {}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::domain::core::file_system::file_creator::file_creator::{
        create_file_with_content, remove_file_if_exists,
    };
    use crate::domain::core::testing::test_path::{get_test_dir_raw, get_test_file};
    use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;
    use crate::domain::usecase::java::generator::file_generator::java_class_skeleton_generator::java_file_skeleton_generator::generate;

    #[test]
    fn generate_test() {
        let mut folder_path = get_test_dir_raw(get_current_file_path());
        folder_path.push("new_folder");
        let file_path_copy = folder_path.clone();
        folder_path.push("new_file.rs");

        let mut skeleton = JavaClassSkeleton::builder()
            .name("aaa")
            .implemented_interface("aefa")
            .build();

        generate(&folder_path, skeleton);

        assert!(folder_path.exists());
        fs::remove_dir_all(file_path_copy.as_path())
            .expect("Test must remove the created files & folders");
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
