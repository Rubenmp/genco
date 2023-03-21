use std::path::Path;

use crate::domain::core::file_system::directory_browser::directory_browser::check_dir;
use crate::domain::core::file_system::file_creator::file_creator;
use crate::domain::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::domain::usecase::java::generator::dto::java_method_generator::JavaMethodGenerator;
use crate::domain::usecase::java::scanner::java_package_scanner::java_package_scanner;

pub fn generate(dir: &Path, skeleton: JavaMethodGenerator) {
    check_exists(dir);

    let mut result = "package ".to_string();
    result += java_package_scanner::get_package(dir).as_str();
    result += ";\n\n";

    write_annotations(&mut result, &skeleton);
    write_visibility(&mut result, &skeleton);
    write_return_type(&mut result, &skeleton);
    result += skeleton.get_name();
    result += " {\n}";

    let mut file_path = dir.to_path_buf();
    file_path.push(format!("{}.java", &skeleton.get_name()));
    file_creator::create_file_if_not_exist(&file_path);
    let mut overwriting = FileOverwriting::new(&file_path);
    overwriting.append(&result);
    overwriting.write_all();
}

fn write_return_type(result: &mut String, skeleton: &JavaMethodGenerator) {
    todo!()
}

fn write_annotations(result: &mut String, skeleton: &JavaMethodGenerator) {
    for annotation in skeleton.get_annotations() {
        *result += format!("@{}\n", annotation.get_name()).as_str();
    }
}

fn write_visibility(result: &mut String, skeleton: &JavaMethodGenerator) {
    let visibility = &skeleton.get_visibility().to_string().to_lowercase();
    if !(visibility == "package") {
        *result += format!("{} ", visibility).as_str();
    }
    *result += "class "
}

fn check_exists(dir: &Path) {
    check_dir(
        dir,
        "Error generating JavaClassSkeleton due to invalid directory",
    );
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::domain::core::file_system::file_creator::file_creator::remove_file_if_exists;
    use crate::domain::core::testing::test_assert::assert_same_file;
    use crate::domain::core::testing::test_path::get_test_dir;
    use crate::domain::usecase::java::generator::dto::java_annotation_generator::JavaAnnotationGenerator;
    use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;
    use crate::domain::usecase::java::generator::dto::java_visibility::JavaVisibility::{Package, Protected, Public};
    use crate::domain::usecase::java::generator::file_generator::java_class_skeleton_generator::java_class_skeleton_generator::generate;

    #[test]
    fn generate_public_abstract_class_with_interface() {
        let mut folder_path = get_test_dir(get_current_file_path(), "generate_java_class_skeleton");
        let mut expected_file_content = folder_path.clone();
        expected_file_content.push("ExpectedPublicAbstractClassWithInterface.java");
        folder_path.push("src/main/java/com/org/demo");
        let mut file_path = folder_path.clone();
        file_path.push("JavaServiceAbstract.java");
        remove_file_if_exists(&file_path);

        let skeleton = JavaClassSkeleton::builder()
            .visibility(Public)
            .is_abstract(true)
            .name("JavaServiceAbstract")
            .implemented_interface("JavaInterface")
            .build();

        generate(&folder_path, skeleton);

        assert!(file_path.exists());
        assert!(file_path.is_file());
        assert_same_file(&expected_file_content, &file_path);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
