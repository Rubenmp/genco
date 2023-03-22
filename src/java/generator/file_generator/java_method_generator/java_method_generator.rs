use std::path::Path;

use crate::domain::core::file_system::directory_browser::directory_browser::check_dir;
use crate::domain::core::file_system::file_creator::file_creator;
use crate::domain::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::java::generator::dto::java_method_generator::JavaMethodGenerator;

pub fn generate(dir: &Path, skeleton: &JavaMethodGenerator) {
    check_exists(dir);

    let mut result = "".to_string();
    write_annotations(&mut result, &skeleton);
    write_visibility(&mut result, &skeleton);
    write_return_type(&mut result, &skeleton);
    result += skeleton.get_name();
    write_parameters(&mut result, &skeleton);
    result += " {\n}";

    write_to_file(dir, &skeleton, &mut result);
}

fn write_parameters(result: &mut String, skeleton: &&JavaMethodGenerator) {
    let parameters = skeleton.get_parameters();
    *result += "(";
    for (index, parameter) in parameters.iter().enumerate() {
        if index > 0 {
            *result += ", ";
        }
        *result += parameter.to_string().as_str();
    }
    *result += ")";
}

fn write_to_file(dir: &Path, skeleton: &&JavaMethodGenerator, result: &mut String) {
    let mut file_path = dir.to_path_buf();
    file_path.push(format!("{}.java", &skeleton.get_name()));
    file_creator::create_file_if_not_exist(&file_path);
    let mut overwriting = FileOverwriting::new(&file_path);
    overwriting.append(&result);
    overwriting.write_all();
}

fn write_return_type(result: &mut String, skeleton: &JavaMethodGenerator) {
    if let Some(return_type) = skeleton.get_return_type() {
        *result += format!("{} ", return_type.to_string()).as_str();
    } else {
        *result += "void ";
    }
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
    use crate::java::generator::dto::java_annotation_generator::JavaAnnotationGenerator;
    use crate::java::generator::dto::java_method_generator::JavaMethodGenerator;
    use crate::java::generator::dto::java_variable_generator::JavaVariableGenerator;
    use crate::java::generator::dto::java_visibility::JavaVisibility::{Protected, Public};
    use crate::java::generator::file_generator::java_method_generator::java_method_generator;

    #[test]
    fn generate_java_method() {
        let mut folder_path = get_test_dir(get_current_file_path(), "generate_java_method");
        let mut expected_file_content = folder_path.clone();
        expected_file_content.push("ExpectedTestMethodWithParameters.java");
        folder_path.push("src/main/java/com/org/demo");
        let mut file_path = folder_path.clone();
        file_path.push("new_method.java");
        remove_file_if_exists(&file_path);

        let mut annotations = Vec::new();
        annotations.push(JavaAnnotationGenerator::builder().name("Test").build());
        let mut parameters = Vec::new();
        parameters.push(JavaVariableGenerator::new_final_int("id"));
        parameters.push(JavaVariableGenerator::new_final_string("name"));
        let skeleton = JavaMethodGenerator::builder()
            .annotations(annotations)
            .visibility(Public)
            .name("new_method")
            .parameters(parameters)
            .build();

        java_method_generator::generate(&folder_path, &skeleton);

        assert!(file_path.exists());
        assert!(file_path.is_file());
        assert_same_file(&expected_file_content, &file_path);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
