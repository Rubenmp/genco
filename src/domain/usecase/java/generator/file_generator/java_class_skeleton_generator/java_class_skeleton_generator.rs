use std::path::Path;

use crate::domain::core::file_system::directory_browser::directory_browser::check_dir;
use crate::domain::core::file_system::file_creator::file_creator;
use crate::domain::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::domain::usecase::java::generator::dto::java_class_skeleton::JavaClassSkeleton;
use crate::domain::usecase::java::scanner::java_package_scanner::java_package_scanner;

pub fn generate(dir: &Path, skeleton: JavaClassSkeleton) {
    check_exists(dir);

    let mut result = "package ".to_string();
    result += java_package_scanner::get_package(dir).as_str();
    result += ";\n\n";

    write_annotations(&mut result, &skeleton);
    write_visibility(&mut result, &skeleton);
    result += skeleton.get_name();
    write_extensions_and_implementations(&mut result, &skeleton);
    result += " {\n}";

    write_to_file(dir, &skeleton, &mut result);
}

fn write_to_file(dir: &Path, skeleton: &JavaClassSkeleton, result: &mut String) {
    let mut file_path = dir.to_path_buf();
    file_path.push(format!("{}.java", &skeleton.get_name()));
    file_creator::create_file_if_not_exist(&file_path);
    let mut overwriting = FileOverwriting::new(&file_path);
    overwriting.append(&result);
    overwriting.write_all();
}

fn write_visibility(result: &mut String, skeleton: &JavaClassSkeleton) {
    let visibility = &skeleton.get_visibility().to_string().to_lowercase();
    if !(visibility == "package") {
        *result += format!("{} ", visibility).as_str();
    }
    if skeleton.is_abstract() {
        *result += "abstract ";
    }
    *result += "class "
}

fn write_extensions_and_implementations(result: &mut String, skeleton: &JavaClassSkeleton) {
    if let Some(extension) = skeleton.get_extended_class() {
        *result += " extends ";
        *result += extension;
    }

    let interfaces = skeleton.get_implemented_interfaces();
    if interfaces.len() > 0 {
        *result += " implements ";
        for (index, interface) in interfaces.iter().enumerate() {
            if index > 0 {
                *result += ", ";
            }
            *result += interface;
        }
    }
}

fn write_annotations(result: &mut String, skeleton: &JavaClassSkeleton) {
    for annotation in skeleton.get_annotations() {
        *result += format!("@{}\n", annotation.get_name()).as_str();
    }
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
        let mut folder_path = get_test_dir(get_current_file_path(), "generate_java_method");
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

    #[test]
    fn generate_package_class_with_interfaces_and_extension() {
        let mut folder_path = get_test_dir(get_current_file_path(), "generate_java_method");
        let mut expected_file_content = folder_path.clone();
        expected_file_content.push("ExpectedPackageClassWithInterfacesAndExtension.java");
        folder_path.push("src/main/java/com/org/demo");
        let mut file_path = folder_path.clone();
        file_path.push("JavaChildServiceImpl.java");
        remove_file_if_exists(&file_path);

        let mut interfaces = Vec::new();
        interfaces.push("JavaInterface1");
        interfaces.push("JavaInterface2");
        let skeleton = JavaClassSkeleton::builder()
            .visibility(Package)
            .name("JavaChildServiceImpl")
            .implemented_interfaces(interfaces)
            .extended_class("ParentClass")
            .build();

        generate(&folder_path, skeleton);

        assert!(file_path.exists());
        assert!(file_path.is_file());
        assert_same_file(&expected_file_content, &file_path);
    }

    #[test]
    fn generate_class_with_annotations() {
        let mut folder_path = get_test_dir(get_current_file_path(), "generate_java_method");
        let mut expected_file_content = folder_path.clone();
        expected_file_content.push("ExpectedTestMethodWithParameters.java");
        folder_path.push("src/main/java/com/org/demo");
        let mut file_path = folder_path.clone();
        file_path.push("JavaServiceBean.java");
        remove_file_if_exists(&file_path);

        let mut annotations = Vec::new();
        annotations.push(JavaAnnotationGenerator::builder().name("Service").build());

        let skeleton = JavaClassSkeleton::builder()
            .visibility(Protected)
            .name("JavaServiceBean")
            .annotations(annotations)
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
