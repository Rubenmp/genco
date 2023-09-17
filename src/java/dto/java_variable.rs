use std::fmt;
use std::path::Path;

use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_data_type::JavaDataType::Basic;
use crate::java::dto::java_data_type::{JavaBasicDataType, JavaDataType};
use crate::java::dto::java_import::JavaImport;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file_imports::JavaFileImports;

#[derive(Debug, Clone)]
pub struct JavaVariable {
    is_final: bool,
    data_type: JavaDataType,
    name: String,
}

impl JavaVariable {
    // Public methods
    pub fn builder() -> JavaVariableBuilder {
        JavaVariableBuilder::new_builder()
    }

    pub fn new_final_int(var_name: &str) -> Self {
        Self::builder()
            .is_final(true)
            .data_type(Basic(JavaBasicDataType::Int))
            .name(var_name)
            .build()
            .unwrap()
    }
    pub fn new_final_string(var_name: &str) -> Self {
        Self::builder()
            .is_final(true)
            .data_type(Basic(JavaBasicDataType::String))
            .name(var_name)
            .build()
            .unwrap()
    }
}

impl JavaVariable {
    // Crate or private methods
    pub(crate) fn from_formal_params_node(
        root_node: &JavaNode,
        file_imports: &JavaFileImports,
        input_java_file: &Path,
    ) -> Result<Vec<Self>, String> {
        let mut params = Vec::new();
        for param_node in root_node.get_children() {
            if Some(JavaNodeType::FormalParam) == param_node.get_node_type_opt() {
                params.push(Self::from_formal_param_node(
                    param_node,
                    file_imports,
                    input_java_file,
                )?);
            }
        }

        Ok(params)
    }
    pub(crate) fn from_formal_param_node(
        root_node: &JavaNode,
        file_imports: &JavaFileImports,
        input_java_file: &Path,
    ) -> Result<Self, String> {
        let mut is_final = false;
        let mut data_type_opt = None;
        let mut name_opt = None;
        for child_node in root_node.get_children() {
            if Some(JavaNodeType::Modifiers) == child_node.get_node_type_opt() {
                for modifier_node in root_node.get_children() {
                    if Some(JavaNodeType::Final) == modifier_node.get_node_type_opt() {
                        is_final = true;
                    }
                }
            }
            if Some(JavaNodeType::TypeIdentifier) == child_node.get_node_type_opt() {
                match JavaDataType::from_generic_type_id(
                    &child_node.get_content(),
                    file_imports,
                    input_java_file,
                ) {
                    Ok(data_type) => data_type_opt = Some(data_type),
                    Err(err) => {
                        logger::log_warning(&err);
                        return Err(err);
                    }
                }
            }
            if Some(JavaNodeType::Id) == child_node.get_node_type_opt() {
                name_opt = Some(child_node.get_content());
            }
        }

        Self::builder()
            .is_final(is_final)
            .data_type(data_type_opt.ok_or("Java data type is mandatory to build variable")?)
            .name(&name_opt.ok_or("Java variable name is mandatory")?)
            .build()
    }

    pub(crate) fn get_import(&self) -> Option<JavaImport> {
        self.data_type.get_import()
    }
}

impl fmt::Display for JavaVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        if self.is_final {
            string += "final ";
        }
        string += format!("{} {}", self.data_type, self.name).as_str();
        write!(f, "{}", string)
    }
}

pub struct JavaVariableBuilder {
    is_final: bool,
    data_type: Option<JavaDataType>,
    name: Option<String>,
}

impl JavaVariableBuilder {
    fn new_builder() -> Self {
        Self {
            is_final: false,
            data_type: None,
            name: None,
        }
    }
    pub fn is_final(&mut self, input: bool) -> &mut Self {
        self.is_final = input;
        self
    }
    pub fn data_type(&mut self, input: JavaDataType) -> &mut Self {
        self.data_type = Some(input.to_owned());
        self
    }
    pub fn name(&mut self, input: &str) -> &mut Self {
        self.name = Some(input.to_string());
        self
    }
    pub fn build(&mut self) -> Result<JavaVariable, String> {
        Ok(JavaVariable {
            is_final: self.is_final,
            data_type: self
                .data_type
                .to_owned()
                .ok_or("Java data type is mandatory to create a JavaVariable")?,
            name: self
                .name
                .to_owned()
                .ok_or("Java variable name is mandatory to create a JavaVariable")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::core::testing::test_assert::assert_fail;
    use crate::java::dto::java_data_type::{JavaBasicDataType, JavaDataType};
    use crate::java::dto::java_variable::JavaVariable;

    #[test]
    fn builder() {
        match JavaVariable::builder()
            .is_final(true)
            .data_type(JavaDataType::Basic(JavaBasicDataType::Int))
            .name("id")
            .build()
        {
            Ok(int) => {
                assert_eq!("final int id", int.to_string());
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn to_string_int() {
        let int = JavaVariable::new_final_int("id");

        assert_eq!("final int id", int.to_string());
    }

    #[test]
    fn get_import_basic_type() {
        let int = JavaVariable::new_final_int("id");

        assert!(int.get_import().is_none());
    }
}
