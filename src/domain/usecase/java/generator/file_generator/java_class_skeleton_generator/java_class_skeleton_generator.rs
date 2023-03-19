use std::path::Path;

use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;

pub fn generate(dir: &Path, skeleton: JavaClassSkeleton) {}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::domain::core::file_system::file_creator::file_creator::remove_file_if_exists;
    use crate::domain::core::testing::test_path::{get_test_dir, get_test_file};
    use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;
    use crate::domain::usecase::java::generator::file_generator::java_class_skeleton_generator::java_class_skeleton_generator::generate;

    #[test]
    fn generate_test() {
        let mut folder_path = get_test_dir(get_current_file_path(), "public_class_with_interface");
        folder_path.push("new_folder");
        let mut file_path_copy = folder_path.clone();
        file_path_copy.push("JavaService.rs");

        let skeleton = JavaClassSkeleton::builder()
            .name("JavaService")
            .implemented_interface("JavaInterface")
            .build();

        generate(&folder_path, skeleton);

        assert!(file_path_copy.exists());
        assert!(file_path_copy.is_file());
        fs::remove_dir_all(file_path_copy.as_path())
            .expect("Test must remove the created files & folders");
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
