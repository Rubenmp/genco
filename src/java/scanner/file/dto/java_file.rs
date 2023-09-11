use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::scanner::file::dto::java_import::JavaImport;
use crate::java::scanner::file::dto::java_package::JavaPackage;
use crate::java::scanner::file::dto::java_structure::JavaStructure;

#[allow(unused)]
pub struct JavaFile {
    package: JavaPackage,
    imports: Vec<JavaImport>,
    structure: JavaStructure,
}

impl JavaFile {
    pub fn new(root_node: JavaNode) -> Result<Self, String> {
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

        Self::check_existence(&package, &structure)?;

        Ok(JavaFile {
            package: package.expect("Java package not found."),
            imports,
            structure: structure.expect("Java structure not found."),
        })
    }

    pub fn get_package(&self) -> &JavaPackage {
        &self.package
    }

    fn check_existence(
        package: &Option<JavaPackage>,
        structure: &Option<JavaStructure>,
    ) -> Result<(), String> {
        if package.is_none() {
            return Err("Java package not found.".to_string());
        } else if structure.is_none() {
            return Err("Java structure not found.".to_string());
        }

        Ok(())
    }
}
