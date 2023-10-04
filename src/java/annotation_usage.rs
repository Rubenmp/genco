use std::fmt;

use crate::core::file_system::file_cache::FileCache;

use crate::core::observability::logger;
use crate::java::import::JavaImport;
use crate::java::indentation_config::JavaIndentation;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file_imports::JavaFileImports;

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
        self.get_self_import().get_last_node().clone()
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

    pub(crate) fn get_imports(&self) -> Vec<&JavaImport> {
        vec![self.get_self_import()]
    }

    pub(crate) fn get_self_import(&self) -> &JavaImport {
        &self.explicit_import
    }

    pub(crate) fn new(explicit_import: JavaImport) -> JavaAnnotationUsage {
        JavaAnnotationUsage { explicit_import }
    }

    /// TODO: add annotation parameters analysis
    pub(crate) fn new_from_java_node_unchecked(
        root_java_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<JavaAnnotationUsage, String> {
        let id_node = Self::get_annotation_id_node(root_java_node).expect("Expected id");
        let explicit_import =
            file_imports.get_explicit_import_from_identifier(id_node, java_file_cache)?;

        Ok(JavaAnnotationUsage { explicit_import })
    }

    fn get_annotation_id_node(node: &JavaNode) -> Option<&JavaNode> {
        node.get_children().get(1)
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

    pub(crate) fn import(&mut self, input: JavaImport) -> &mut Self {
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
    use crate::java::annotation_usage::JavaAnnotationUsage;
    use crate::java::import::JavaImport;

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
