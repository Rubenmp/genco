use std::path::Path;

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_data_type::JavaDataType;
use crate::java::dto::java_expression::JavaExpression;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_indentation_config::JavaIndentation;
use crate::java::dto::java_visibility::JavaVisibility;
use crate::java::dto::{java_annotation_usage, java_visibility};
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file_imports::JavaFileImports;

#[derive(Debug, Clone)]
pub struct JavaField {
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_final: bool,
    data_type: JavaDataType,
    name: String,
    value: Option<JavaExpression>,
}

impl JavaField {
    pub fn builder() -> JavaFieldBuilder {
        JavaFieldBuilder::new_builder()
    }
    pub fn get_annotations(&self) -> Vec<JavaAnnotationUsage> {
        self.annotations.to_owned()
    }
    pub fn get_visibility(&self) -> JavaVisibility {
        self.visibility.to_owned()
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    pub fn get_data_type(&self) -> JavaDataType {
        self.data_type.to_owned()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &Option<JavaExpression> {
        &self.value
    }
}

impl JavaField {
    pub(crate) fn new(
        root_node: &JavaNode,
        file_imports: &JavaFileImports,
        input_java_file: &Path,
    ) -> Result<JavaField, String> {
        let mut annotations = Vec::new();
        let mut is_static = false;
        let mut is_final = false;
        let mut visibility = JavaVisibility::Package;
        let mut data_type_opt: Option<JavaDataType> = None;
        let mut name = "".to_string();
        let mut value = None;

        for child in root_node.get_children() {
            let node_type_opt = child.get_node_type_opt();
            if Some(JavaNodeType::Modifiers) == node_type_opt {
                for modifiers_child in child.get_children() {
                    let modifier_type_opt = modifiers_child.get_node_type_opt();
                    if java_annotation_usage::is_java_node_annotation_opt(&modifier_type_opt) {
                        match JavaAnnotationUsage::new_from_java_node(
                            modifiers_child,
                            file_imports,
                            input_java_file,
                        ) {
                            Ok(annotation) => annotations.push(annotation),
                            Err(err) => logger::log_warning(&err),
                        };
                    } else if java_node_type::is_visibility(&modifier_type_opt) {
                        visibility = java_visibility::new(&modifier_type_opt.unwrap());
                    } else if let Some(JavaNodeType::Static) = modifier_type_opt {
                        is_static = true;
                    } else if let Some(JavaNodeType::Final) = modifier_type_opt {
                        is_final = true;
                    }
                }
            } else if JavaDataType::is_data_type_node_opt(&node_type_opt) {
                match JavaDataType::get_data_type(child, file_imports, input_java_file) {
                    Ok(data_type) => data_type_opt = Some(data_type),
                    Err(err) => logger::log_warning(&err),
                }
            } else if Some(JavaNodeType::VariableDeclarator) == node_type_opt {
                let mut next_child_is_expression = false;
                for var_decl_child in child.get_children() {
                    if let Some(var_node_type) = var_decl_child.get_node_type_opt() {
                        if JavaNodeType::Id == var_node_type {
                            name = child.get_content();
                        } else if JavaNodeType::Equals == var_node_type {
                            next_child_is_expression = true
                        } else if next_child_is_expression {
                            next_child_is_expression = false;
                            value = Some(JavaExpression::new(var_decl_child));
                        }
                    }
                }
            }
        }

        if name.is_empty() {
            return Err(format!(
                "Invalid java field declaration (name is empty) \"{:?}\" in file\n\"{}\"\n",
                root_node.get_content(),
                try_to_absolute_path(input_java_file)
            ));
        }
        let data_type = data_type_opt.ok_or(&format!(
            "Invalid java field declaration (data_type is empty) \"{:?}\" in file:\n\"{}\"\n",
            root_node.get_content(),
            try_to_absolute_path(input_java_file)
        ))?;

        Ok(JavaField {
            annotations,
            visibility,
            is_static,
            is_final,
            data_type,
            name,
            value,
        })
    }

    pub(crate) fn get_str(&self, initial_indentation: &JavaIndentation) -> String {
        let mut result = "".to_string();
        for annotation in self.get_annotations() {
            result += initial_indentation.get_current_indentation().as_str();
            result += annotation.to_string().as_str();
            result += "\n"
        }

        result += initial_indentation.get_current_indentation().as_str();
        result += self.get_visibility().as_file_string().as_str();
        if self.is_static {
            result += "static ";
        }
        if self.is_final {
            result += "final ";
        }
        result += format!("{} ", self.data_type).as_str();
        result += format!("{};\n", self.get_name()).as_str();
        result
    }

    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        let mut imports = Vec::new();

        for import in self.get_annotation_imports() {
            imports.push(import);
        }

        if let Some(type_import) = self.get_data_type().get_import() {
            imports.push(type_import.to_owned());
        }

        imports
    }

    fn get_annotation_imports(&self) -> Vec<JavaImport> {
        self.get_annotations()
            .iter()
            .flat_map(|annotation| annotation.get_imports())
            .collect()
    }
}

pub struct JavaFieldBuilder {
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_final: bool,
    data_type: Option<JavaDataType>,
    name: Option<String>,
    value: Option<JavaExpression>,
}

impl JavaFieldBuilder {
    fn new_builder() -> Self {
        Self {
            annotations: vec![],
            visibility: JavaVisibility::Package,
            is_static: false,
            is_final: false,
            data_type: None,
            name: None,
            value: None,
        }
    }
    pub fn annotations(&mut self, input: Vec<JavaAnnotationUsage>) -> &mut Self {
        self.annotations = input;
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
    pub fn is_final(&mut self, input: bool) -> &mut Self {
        self.is_final = input;
        self
    }
    pub fn data_type(&mut self, input: JavaDataType) -> &mut Self {
        self.data_type = Some(input);
        self
    }
    pub fn name(&mut self, input: &str) -> &mut Self {
        self.name = Some(input.to_string());
        self
    }
    pub fn value(&mut self, input: JavaExpression) -> &mut Self {
        self.value = Some(input);
        self
    }

    pub fn build(&mut self) -> Result<JavaField, String> {
        Ok(JavaField {
            annotations: self.annotations.to_owned(),
            visibility: self.visibility,
            is_static: self.is_static,
            is_final: self.is_final,
            data_type: self
                .data_type
                .to_owned()
                .ok_or("Missing mandatory \"data_type\" to build java field")?,
            name: self
                .name
                .to_owned()
                .ok_or("Missing mandatory \"name\" to build java field")?,
            value: self.value,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::{assert_fail, assert_same_as_file};
    use crate::core::testing::test_path::get_test_dir;
    use crate::java::dependency::org::springframework::spring_beans::java_spring_beans_factory;
    use crate::java::dto::java_data_type::{JavaBasicDataType, JavaDataType};
    use crate::java::dto::java_field::JavaField;
    use crate::java::dto::java_indentation_config::JavaIndentation;
    use crate::java::dto::java_visibility::JavaVisibility;

    #[test]
    fn get_str_autowired_private_static_final() {
        let expected_str_file_path = get_test_dir(get_current_file_path(), "java_field")
            .join("ExpectedAutowiredPrivateStaticField.java");
        match JavaField::builder()
            .annotations(vec![
                java_spring_beans_factory::_create_autowired_annotation_usage(),
            ])
            .visibility(JavaVisibility::Private)
            .is_static(true)
            .is_final(true)
            .data_type(JavaDataType::Basic(JavaBasicDataType::String))
            .name("field")
            .build()
        {
            Ok(field) => {
                let field_str = field.get_str(&JavaIndentation::default());
                assert_same_as_file(&expected_str_file_path, &field_str);
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn get_imports_empty() {
        match JavaField::builder()
            .data_type(JavaDataType::Basic(JavaBasicDataType::String))
            .name("field")
            .build()
        {
            Ok(field) => {
                assert!(field.get_imports().is_empty());
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
