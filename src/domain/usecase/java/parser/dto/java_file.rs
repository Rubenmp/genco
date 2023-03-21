use crate::domain::usecase::java::parser::dto::java_import::JavaImport;
use crate::domain::usecase::java::parser::dto::java_node::JavaNode;
use crate::domain::usecase::java::parser::dto::java_node_type::JavaNodeType;
use crate::domain::usecase::java::parser::dto::java_package::JavaPackage;
use crate::domain::usecase::java::parser::dto::java_structure::JavaStructure;

#[allow(unused)]
pub struct JavaFile {
    package: JavaPackage,
    imports: Vec<JavaImport>,
    structure: JavaStructure,
}

impl JavaFile {
    pub fn new(root_node: JavaNode) -> Self {
        let mut package: Option<JavaPackage> = None;
        let mut imports = Vec::new();
        let mut structure: Option<JavaStructure> = None;

        for child in root_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if JavaNodeType::ImportDecl == node_type {
                    imports.push(JavaImport::new(&child));
                } else if JavaNodeType::PackageDecl == node_type {
                    package = Some(JavaPackage::new(&child));
                } else if JavaNodeType::ClassDecl == node_type
                    || JavaNodeType::InterfaceDeclaration == node_type
                    || JavaNodeType::EnumDeclaration == node_type
                {
                    structure = Some(JavaStructure::new(&child));
                }
            }
        }

        JavaFile {
            package: package.expect("Java package not found."),
            imports,
            structure: structure.expect("Java structure not found."),
        }
    }
}
