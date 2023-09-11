use std::fmt;

use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;

#[allow(unused)]
pub struct JavaPackage {
    nodes: Vec<String>,
}

impl<'a> JavaPackage {
    pub fn new(node: &JavaNode) -> JavaPackage {
        JavaPackage {
            nodes: get_nodes(node.clone()),
        }
    }
}

fn get_nodes(java_package_decl_node: JavaNode) -> Vec<String> {
    if Some(JavaNodeType::PackageDecl) != java_package_decl_node.get_node_type() {
        return Vec::new();
    }

    let mut nodes = Vec::new();
    for children_level_one in java_package_decl_node.get_children() {
        if Some(JavaNodeType::ScopedIdentifier) == children_level_one.get_node_type() {
            for children_level_two in children_level_one.get_children() {
                if Some(JavaNodeType::ScopedIdentifier) == children_level_two.get_node_type() {
                    for children_level_three in children_level_two.get_children() {
                        if Some(JavaNodeType::Id) == children_level_three.get_node_type() {
                            nodes.push(children_level_three.get_content());
                        }
                    }
                } else if Some(JavaNodeType::Id) == children_level_two.get_node_type() {
                    nodes.push(children_level_two.get_content());
                }
            }
        }
    }

    nodes
}

impl fmt::Display for JavaPackage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (idx, children) in self.nodes.iter().enumerate() {
            if idx != 0 {
                fmt.write_str(".")?;
            }
            fmt.write_str(&children)?;
        }
        Ok(())
    }
}
