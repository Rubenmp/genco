use crate::java::parser::java_node::JavaNode;

#[derive(Debug, Clone)]
pub struct JavaStatement {}

impl JavaStatement {
    pub(crate) fn new(_node: &JavaNode) -> Self {
        JavaStatement {}
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_java_test_file;
    use crate::java::scanner::file::java_file::JavaFile;

    #[test]
    fn get_imports_empty() {
        let java_file = get_java_test_file(
            get_current_file_path(),
            "java_expression",
            "JavaExpressionArithmetic.java",
        );

        match JavaFile::from_user_input_path(&java_file) {
            Ok(file) => {
                let methods = file.get_structure().get_methods();
                assert_eq!(1, methods.len());
                match methods.get(0) {
                    Some(_method) => {}
                    _ => {}
                }
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
