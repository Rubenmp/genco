use crate::domain::usecase::java::parser::dto::java_node::JavaNode;

#[allow(unused)]
pub struct JavaImport {
    node: JavaNode,
}

impl<'a> JavaImport {
    pub fn new(node: &JavaNode) -> JavaImport {
        JavaImport {
            node: node.clone()
        }
    }
}