use crate::domain::core::parser::parser_node_trait::ParserNode;
use crate::domain::usecase::java::generator::dto::java_data_type::JavaDataType;
use crate::domain::usecase::java::generator::dto::java_visibility::JavaVisibility;
use crate::domain::usecase::java::generator::dto::{java_data_type, java_visibility};
use crate::domain::usecase::java::parser::dto::java_annotation::JavaAnnotation;
use crate::domain::usecase::java::parser::dto::java_expression::JavaExpression;
use crate::domain::usecase::java::parser::dto::java_node::JavaNode;
use crate::domain::usecase::java::parser::dto::java_node_type::JavaNodeType;
use crate::domain::usecase::java::parser::dto::{java_annotation, java_node_type};

#[allow(unused)]
#[derive(Debug)]
pub struct JavaFieldDeclaration {
    node: JavaNode,
    annotations: Vec<JavaAnnotation>,
    visibility: JavaVisibility,
    data_type: JavaDataType,
    name: String,
    value: Option<JavaExpression>,
}

impl<'a> JavaFieldDeclaration {
    pub fn new(root_node: &JavaNode) -> JavaFieldDeclaration {
        let mut annotations = Vec::new();
        let mut visibility = JavaVisibility::Package;
        let mut data_type_opt: Option<JavaDataType> = None;
        let mut name = "".to_string();
        let mut value = None;

        for child in root_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if JavaNodeType::Modifiers == node_type {
                    for modifiers_child in child.get_children() {
                        if let Some(modifiers_node_type) = modifiers_child.get_node_type() {
                            if java_annotation::is_java_node_annotation(&modifiers_node_type) {
                                annotations.push(JavaAnnotation::new(&modifiers_child));
                            } else if java_node_type::is_visibility(&modifiers_node_type) {
                                visibility = java_visibility::new(&modifiers_node_type);
                            }
                        }
                    }
                } else if JavaNodeType::TypeIdentifier == node_type {
                    data_type_opt = Some(java_data_type::new(&child.get_content()));
                } else if JavaNodeType::VariableDeclarator == node_type {
                    let mut next_child_is_expression = false;
                    for var_decl_child in child.get_children() {
                        if let Some(var_node_type) = var_decl_child.get_node_type() {
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
        }

        JavaFieldDeclaration {
            node: root_node.clone(),
            annotations,
            visibility,
            data_type: data_type_opt.unwrap(),
            name,
            value,
        }
    }
}
