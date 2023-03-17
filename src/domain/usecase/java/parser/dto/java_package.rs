use crate::domain::usecase::java::parser::dto::java_node::JavaNode;

#[allow(unused)]
pub struct JavaPackage {
    node: JavaNode,
}

impl<'a> JavaPackage {
    pub fn new(node: &JavaNode) -> JavaPackage {
        JavaPackage { node: node.clone() }
    }
}
