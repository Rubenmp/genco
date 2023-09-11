use crate::java::parser::dto::java_node::JavaNode;

#[derive(Debug, Copy, Clone)]
pub struct JavaExpression {}

impl JavaExpression {
    pub(crate) fn new(_node: &JavaNode) -> Self {
        JavaExpression {}
    }
}
