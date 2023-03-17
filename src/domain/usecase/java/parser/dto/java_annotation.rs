use crate::domain::usecase::java::parser::dto::java_node::JavaNode;

#[derive(Debug)]
pub struct JavaAnnotation {
    node: JavaNode,
}

impl<'a> JavaAnnotation {
    pub fn new(node: &JavaNode) -> JavaAnnotation {
        JavaAnnotation { node: node.clone() }
    }
}
