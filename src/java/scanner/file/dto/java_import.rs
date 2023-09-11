use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_node::JavaNode;

#[allow(unused)]
pub struct JavaImport {
    node: JavaNode,
}

impl JavaImport {
    pub(crate) fn is_explicit_import(&self) -> bool {
        todo!()
    }
    pub(crate) fn is_wildcard_import(&self) -> bool {
        todo!()
    }
}

impl<'a> JavaImport {
    pub fn new(node: &JavaNode) -> JavaImport {
        println!("{}", node.get_tree_str());
        panic!("finish");
        JavaImport { node: node.clone() }
    }
}
