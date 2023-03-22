use crate::java::parser::dto::java_node::JavaNode;

#[derive(Debug)]
pub struct JavaExpression {
    node: JavaNode,
}

impl<'a> JavaExpression {
    pub fn new(node: &JavaNode) -> Self {
        JavaExpression { node: node.clone() }
    }
}
