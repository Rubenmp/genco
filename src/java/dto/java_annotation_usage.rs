use std::fmt;
use std::path::Path;

use crate::core::file_system::path_helper;
use crate::core::observability::logger::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_indentation_config::JavaIndentation;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_imports_scan::JavaImportsScan;

#[derive(Debug, Clone)]
pub struct JavaAnnotationUsage {
    explicit_import: JavaImport,
    // TODO: allow annotation parameters
}

impl JavaAnnotationUsage {
    // Public methods
    pub fn builder() -> JavaAnnotationUsageBuilder {
        JavaAnnotationUsageBuilder::new_builder()
    }

    pub fn get_name(&self) -> String {
        self.get_import().get_last_node().to_owned()
    }
}
impl JavaAnnotationUsage {
    // Crate or private methods
    pub(crate) fn to_file_string(&self, indentation: &JavaIndentation) -> String {
        format!(
            "{}@{}\n",
            indentation.get_current_indentation(),
            self.get_name()
        )
    }

    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        vec![self.get_import().to_owned()]
    }

    fn get_import(&self) -> &JavaImport {
        &self.explicit_import
    }

    pub(crate) fn new(explicit_import: JavaImport) -> JavaAnnotationUsage {
        JavaAnnotationUsage { explicit_import }
    }

    pub(crate) fn new_from_java_node(
        root_java_node: &JavaNode,
        file_imports: &JavaImportsScan,
        input_java_file: &Path,
    ) -> Result<JavaAnnotationUsage, String> {
        let mut name_opt = None;
        if is_java_node_annotation_opt(&root_java_node.get_node_type_opt()) {
            if let Some(id) = Self::get_annotation_id(root_java_node) {
                name_opt = Some(id);
            }
        } else {
            return Err(format!(
                "Unexpected java annotation \"{}\" in file:\n\"{}\"\n",
                root_java_node.get_content(),
                input_java_file.to_string_lossy()
            ));
        }

        let result_name = name_opt.ok_or(format!(
            "Unexpected java annotation name parsing:\n\"{:?}\"in file:\n\"{}\"\n",
            root_java_node.get_content(),
            path_helper::to_absolute_path_str(input_java_file)
        ))?;

        let explicit_import =
            Self::get_explicit_import(root_java_node, file_imports, input_java_file, &result_name)?;

        Ok(JavaAnnotationUsage { explicit_import })
    }

    fn get_explicit_import(
        root_java_node: &JavaNode,
        file_imports: &JavaImportsScan,
        input_java_file: &Path,
        result_name: &str,
    ) -> Result<JavaImport, String> {
        return match file_imports.get_explicit_import(&result_name) {
            Ok(explicit_import) => Ok(explicit_import),
            Err(_) => Err(format!(
                "Unexpected java annotation \"{}\" without import in file:\n\"{}\"\n",
                root_java_node.get_content(),
                path_helper::to_absolute_path_str(input_java_file)
            )),
        };
    }

    fn get_annotation_id(node: &JavaNode) -> Option<String> {
        if let Some(id_node) = node.get_children().get(1) {
            let content = id_node.get_content();
            if !content.is_empty() {
                return Some(content);
            }
        }
        None
    }
}

pub(crate) fn is_java_node_annotation_opt(node_type_opt: &Option<JavaNodeType>) -> bool {
    if let Some(node_type) = node_type_opt {
        return is_java_node_annotation(node_type);
    }
    false
}
pub(crate) fn is_java_node_annotation(node_type: &JavaNodeType) -> bool {
    &JavaNodeType::Annotation == node_type || &JavaNodeType::MarkerAnnotation == node_type
}

impl fmt::Display for JavaAnnotationUsage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "@{}", self.get_name())?;

        Ok(())
    }
}

pub struct JavaAnnotationUsageBuilder {
    import: Option<JavaImport>,
}
impl JavaAnnotationUsageBuilder {
    fn new_builder() -> Self {
        Self { import: None }
    }

    pub fn import(&mut self, input: JavaImport) -> &mut Self {
        if !input.is_explicit_import() {
            let error = format!("Trying to create invalid JavaAnnotationUsage.\n\tExpected: explicit import (ex. \"import javax.persistence.Entity\")\n\tFound: \"{}\"", input.get_route());
            logger::log_unrecoverable_error(&error);
        }
        self.import = Some(input);
        self
    }
    pub fn build(&mut self) -> JavaAnnotationUsage {
        JavaAnnotationUsage::new(self.import.clone().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
    use crate::java::dto::java_import::JavaImport;

    #[test]
    fn to_string() {
        let import = JavaImport::new_explicit_import_requiring_m2_repo_scan(
            "org.springframework.stereotype.Service",
        )
        .expect("Java explicit import is valid");
        let annotation = JavaAnnotationUsage::builder().import(import).build();

        assert_eq!("@Service", annotation.to_string());
    }

    #[test]
    fn get_imports() {
        let import_package = "org.springframework.stereotype.Service";
        let annotation = JavaAnnotationUsage::builder()
            .import(
                JavaImport::new_explicit_import_requiring_m2_repo_scan(import_package)
                    .expect("Java explicit import is valid"),
            )
            .build();

        let result_imports = annotation.get_imports();

        assert_eq!(1, result_imports.len());
        if let Some(result_import) = result_imports.get(0) {
            assert_eq!(import_package, result_import.get_route());
        }
    }
}
