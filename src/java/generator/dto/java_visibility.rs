use std::fmt;

use crate::java::parser::dto::java_node_type::JavaNodeType;

#[derive(Debug)]
pub enum JavaVisibility {
    Public,
    Private,
    Package,
    Protected,
}

impl fmt::Display for JavaVisibility {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;

        Ok(())
    }
}

pub(crate) fn new(node_type: &JavaNodeType) -> JavaVisibility {
    if &JavaNodeType::Private == node_type {
        return JavaVisibility::Private;
    } else if &JavaNodeType::Public == node_type {
        return JavaVisibility::Public;
    } else if &JavaNodeType::Protected == node_type {
        return JavaVisibility::Protected;
    }

    panic!(
        "Invalid node_type \"{}\" to to create JavaVisibility.",
        node_type
    );
}
