use crate::java::parser::dto::java_node::JavaNode;

#[derive(Debug, Copy, Clone)]
pub struct JavaExpression {}

impl<'a> JavaExpression {
    pub(crate) fn new(_node: &JavaNode) -> Self {
        JavaExpression {}
    }
}
