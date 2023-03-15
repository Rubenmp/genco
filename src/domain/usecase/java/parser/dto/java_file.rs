use crate::domain::usecase::java::parser::dto::java_annotation::JavaAnnotation;
use crate::domain::usecase::java::parser::dto::java_import::JavaImport;
use crate::domain::usecase::java::parser::dto::java_node::JavaNode;
use crate::domain::usecase::java::parser::dto::java_node_type::JavaNodeType;

#[allow(unused)]
pub struct JavaFile {
    imports: Vec<JavaImport>,
    annotations: Vec<JavaAnnotation>,
}

impl JavaFile {
    pub fn new(root_node: JavaNode) -> Self {
        let mut java_file = JavaFile {
            imports: Vec::new(),
            annotations: Vec::new()
        };

        fill(&mut java_file, &root_node);
        java_file
    }

}
fn fill(java_file: &mut JavaFile, current_node: & JavaNode) {
    if let Some(node_type) = current_node.get_node_type() {
        if JavaNodeType::ImportDecl == node_type {
            let import = JavaImport::new(current_node);
            java_file.imports.push(import);
        }
    }

    for child in current_node.clone().get_children() {
        fill(java_file, &child);
    }
}