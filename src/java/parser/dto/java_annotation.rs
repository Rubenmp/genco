use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;

#[derive(Debug)]
pub struct JavaAnnotation {
    node: JavaNode,
}

impl<'a> JavaAnnotation {
    pub fn new_from_modifiers(modifiers_node: &JavaNode) -> Vec<JavaAnnotation> {
        let mut result = Vec::new();
        for child in modifiers_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if is_java_node_annotation(&node_type) {
                    result.push(JavaAnnotation::new(&child));
                }
            }
        }

        result
    }

    pub fn new(node: &JavaNode) -> JavaAnnotation {
        JavaAnnotation { node: node.clone() }
    }
}

pub fn is_java_node_annotation(node_type: &JavaNodeType) -> bool {
    &JavaNodeType::Annotation == node_type || &JavaNodeType::MarkerAnnotation == node_type
}
