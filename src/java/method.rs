use crate::core::file_system::file_cache::FileCache;

use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::annotation_usage::JavaAnnotationUsage;
use crate::java::data_type::JavaDataType;
use crate::java::import::JavaImport;
use crate::java::indentation_config::JavaIndentation;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file_imports::JavaFileImports;
use crate::java::statement::JavaStatement;
use crate::java::variable::JavaVariable;
use crate::java::visibility::JavaVisibility;
use crate::java::{annotation_usage, visibility};

#[derive(Debug, Clone)]
pub struct JavaMethod {
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    return_type: Option<JavaDataType>,
    name: String,
    parameters: Vec<JavaVariable>,
    _statements: Vec<JavaStatement>,
}

// Public methods
impl JavaMethod {
    /// # Builder pattern
    /// This method allows to create a new Java Method
    /// and export it to a file. The "name" parameter is mandatory.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::method::JavaMethod;
    ///
    /// let java_method = JavaMethod::builder().name("newMethod").build();
    /// ```
    pub fn builder() -> JavaMethodBuilder {
        JavaMethodBuilder::new_builder()
    }

    /// # get_annotations
    /// Get the java annotations of the method
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        &self.annotations
    }

    /// # get_visibility
    /// Get the java visibility of the method
    pub fn get_visibility(&self) -> JavaVisibility {
        self.visibility
    }

    /// # is_static
    /// It returns if the current JavaMethod is static.
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// # get_return_type
    /// It returns the JavaMethod return JavaDataType.
    pub fn get_return_type(&self) -> &Option<JavaDataType> {
        &self.return_type
    }

    /// # get_name
    /// It returns the current JavaMethod name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// # get_parameters
    /// Get the method parameters sorted from the first one to the last one.
    pub fn get_parameters(&self) -> &Vec<JavaVariable> {
        &self.parameters
    }
}

// Crate related methods
impl JavaMethod {
    pub(crate) fn new_from_node(
        root_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<JavaMethod, String> {
        let mut annotations = Vec::new();
        let mut visibility = JavaVisibility::Package;
        let mut is_static = false;
        let mut return_type_opt = None;
        let mut return_type_detected = false;
        let mut name_opt = None;
        let mut parameters = Vec::new();

        for child_node in root_node.get_children() {
            if let Some(node_type) = child_node.get_node_type() {
                if JavaNodeType::Modifiers == node_type {
                    for modifier in child_node.get_children() {
                        if let Some(node_type) = modifier.get_node_type() {
                            if annotation_usage::is_java_node_annotation(&node_type) {
                                match JavaAnnotationUsage::new_from_java_node_unchecked(
                                    modifier,
                                    file_imports,
                                    java_file_cache,
                                ) {
                                    Ok(annotation) => annotations.push(annotation),
                                    Err(err) => logger::log_warning(&err),
                                };
                            } else if visibility::is_visibility_node_type(&node_type) {
                                visibility = visibility::new(&node_type);
                            } else if JavaNodeType::Static == node_type {
                                is_static = true;
                            }
                        }
                    }
                } else if child_node.is_data_type_identifier() {
                    match JavaDataType::get_data_type(child_node, file_imports, java_file_cache) {
                        Ok(data_type) => {
                            return_type_opt = Some(data_type);
                            return_type_detected = true;
                        }
                        Err(err) => logger::log_warning(&err),
                    }
                } else if JavaNodeType::VoidType == node_type {
                    return_type_detected = true;
                } else if JavaNodeType::Id == node_type {
                    name_opt = Some(child_node.get_content_from_cache(java_file_cache));
                } else if JavaNodeType::FormalParams == node_type {
                    match JavaVariable::from_formal_params_node(
                        child_node,
                        file_imports,
                        java_file_cache,
                    ) {
                        Ok(result) => parameters = result,
                        Err(err) => return Err(format!("Invalid java method parameters, {}", err)),
                    }
                } else if JavaNodeType::Block == node_type {
                    // TODO: inspect method body
                }
            }
        }

        if !return_type_detected {
            return Err("Java method return type not detected".to_string());
        }

        Ok(JavaMethod {
            annotations,
            visibility,
            return_type: return_type_opt,
            is_static,
            name: name_opt.ok_or("Java method name not detected.")?,
            parameters,
            _statements: Vec::new(),
        })
    }

    pub(crate) fn write_to_string(&self, result: &mut String, indentation: &JavaIndentation) {
        self.write_annotations(result, indentation);
        self.write_visibility(result, indentation);
        self.write_return_type(result);
        *result += self.get_name();
        self.write_parameters(result);
        *result += " {\n";
        // TODO: write method body here
        *result += format!("{}}}\n", indentation.get_current_indentation()).as_str();
    }

    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        let mut imports = Vec::new();
        for import in self.get_annotation_imports() {
            imports.push(import.clone())
        }

        if let Some(import) = self
            .get_return_type()
            .as_ref()
            .and_then(|rt| rt.get_import_opt())
        {
            imports.push(import.clone())
        }

        for import in self.get_param_imports() {
            imports.push(import.clone());
        }

        imports
    }

    fn get_annotation_imports(&self) -> Vec<&JavaImport> {
        self.get_annotations()
            .iter()
            .flat_map(|annotation| annotation.get_imports())
            .collect()
    }

    fn get_param_imports(&self) -> Vec<JavaImport> {
        self.get_parameters()
            .iter()
            .filter_map(|param| param.get_import())
            .collect()
    }

    fn write_parameters(&self, result: &mut String) {
        let parameters = self.get_parameters();
        *result += "(";
        for (index, parameter) in parameters.iter().enumerate() {
            if index > 0 {
                *result += ", ";
            }

            *result += parameter.to_string().as_str();
        }
        *result += ")";
    }

    fn write_return_type(&self, result: &mut String) {
        if let Some(return_type) = self.get_return_type() {
            *result += format!("{} ", return_type).as_str();
        } else {
            *result += "void ";
        }
    }

    fn write_annotations(&self, result: &mut String, indentation: &JavaIndentation) {
        for annotation in self.get_annotations() {
            *result += annotation.to_file_string(indentation).as_str();
        }
    }

    fn write_visibility(&self, result: &mut String, indentation: &JavaIndentation) {
        *result += indentation.get_current_indentation().as_str();
        *result += &self.get_visibility().as_file_string();
    }
}

pub struct JavaMethodBuilder {
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    return_type: Option<JavaDataType>,
    name: Option<String>,
    parameters: Vec<JavaVariable>,
}

impl JavaMethodBuilder {
    fn new_builder() -> Self {
        Self {
            annotations: vec![],
            visibility: JavaVisibility::Package,
            is_static: false,
            return_type: None,
            name: None,
            parameters: vec![],
        }
    }
    pub fn annotations(&mut self, input: Vec<JavaAnnotationUsage>) -> &mut Self {
        self.annotations = input.clone();
        self
    }
    pub fn visibility(&mut self, input: JavaVisibility) -> &mut Self {
        self.visibility = input;
        self
    }
    pub fn is_static(&mut self, input: bool) -> &mut Self {
        self.is_static = input;
        self
    }
    pub fn return_type(&mut self, input: JavaDataType) -> &mut Self {
        self.return_type = Some(input.clone());
        self
    }
    pub fn name(&mut self, input: &str) -> &mut Self {
        self.name = Some(input.to_string());
        self
    }
    pub fn parameters(&mut self, input: Vec<JavaVariable>) -> &mut Self {
        self.parameters = input.clone();
        self
    }

    // TODO: possibility to add statements and expressions here

    pub fn build(&mut self) -> Result<JavaMethod, String> {
        Ok(JavaMethod {
            annotations: self.annotations.clone(),
            visibility: self.visibility,
            is_static: self.is_static,
            return_type: self.return_type.clone(),
            name: self
                .name
                .clone()
                .ok_or("Missing mandatory name to build JavaMethod")?,
            parameters: self.parameters.clone(),
            _statements: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_same_as_file;
    use crate::core::testing::test_path;
    use crate::java::dependency::org::junit::jupiter::junit_jupiter_api::java_junit_jupiter_api_factory;
    use crate::java::indentation_config::JavaIndentation;
    use crate::java::method::JavaMethod;
    use crate::java::variable::JavaVariable;
    use crate::java::visibility::JavaVisibility;

    #[test]
    fn generate_java_method() {
        let _folder_path = get_test_folder();
        let expected_file_content = get_test_file("ExpectedTestMethodWithParameters");

        let annotations = vec![java_junit_jupiter_api_factory::_create_test_annotation_usage()];
        let parameters = vec![
            JavaVariable::new_final_int("id"),
            JavaVariable::new_final_string("name"),
        ];
        let method = JavaMethod::builder()
            .annotations(annotations)
            .visibility(JavaVisibility::Public)
            .name("newMethodToGenerate")
            .parameters(parameters)
            .build()
            .expect("newMethodToGenerate is expected to be valid");

        let mut result = "".to_string();
        method.write_to_string(&mut result, &JavaIndentation::default());

        assert_same_as_file(&expected_file_content, &result);
    }

    #[test]
    fn get_method_imports_empty_method() {
        let method = JavaMethod::builder()
            .name("newMethodWithoutImports")
            .build()
            .expect("newMethodWithoutImports is expected to be valid");

        let imports = method.get_imports();

        assert!(imports.is_empty());
    }

    fn get_test_file(structure_name: &str) -> PathBuf {
        get_test_folder().join(format!("{}.java", structure_name).as_str())
    }

    fn get_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "method")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
