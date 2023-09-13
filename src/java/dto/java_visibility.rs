use std::fmt;

use crate::java::parser::dto::java_node_type::JavaNodeType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JavaVisibility {
    Public,
    Private,
    Package,
    Protected,
}

impl JavaVisibility {
    pub(crate) fn as_file_string(&self) -> String {
        let visibility = self.to_string().to_lowercase();
        if visibility == "package" {
            return "".to_string();
        }
        format!("{} ", visibility)
    }
}

impl fmt::Display for JavaVisibility {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;

        Ok(())
    }
}

pub(crate) fn is_visibility_node_type(node_type: &JavaNodeType) -> bool {
    &JavaNodeType::Private == node_type
        || &JavaNodeType::Public == node_type
        || &JavaNodeType::Protected == node_type
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
