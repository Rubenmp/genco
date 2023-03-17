use crate::domain::usecase::java::parser::dto::java_node::JavaNode;

#[allow(unused)]
#[derive(Debug)]
pub struct JavaMethod {
    node: JavaNode,
}

impl<'a> JavaMethod {
    pub fn new(node: &JavaNode) -> JavaMethod {
        JavaMethod { node: node.clone() }
    }
}
