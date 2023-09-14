use std::path::{Path, PathBuf};

use crate::core::file_system::path_helper;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_import::JavaImport;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::parser::java_parser;
use crate::java::scanner::file::java_imports_scan::JavaImportsScan;
use crate::java::scanner::file::java_structure::JavaStructure;
use crate::java::scanner::file::java_structure_type::JavaStructureType;
use crate::java::scanner::package::{java_dependency_scanner, java_package_scanner};

#[derive(Debug)]
pub(crate) struct JavaFile {
    self_import: JavaImport,
    #[cfg(test)]
    imports: JavaImportsScan,
    structure: JavaStructure,
}

impl JavaFile {
    pub(crate) fn from_user_input_path(java_file_path: &Path) -> Result<Self, String> {
        let base_java_project_dir_opt = java_package_scanner::get_base_package(java_file_path);
        if let Some(base_java_project_dir) = base_java_project_dir_opt {
            java_dependency_scanner::recursive_scan_dir_unchecked(&base_java_project_dir);
        } else {
            return Err(format!(
                "Invalid java project file:\n\"{}\"\n",
                try_to_absolute_path(java_file_path)
            ));
        }

        let root_java_node = java_parser::parse(java_file_path)?;
        let mut package_found = false;
        let mut imports = JavaImportsScan::new();
        let mut structure_opt: Option<JavaStructure> = None;
        let java_file_import = JavaImport::new_explicit_import_from_file(java_file_path)?;

        for child in root_java_node.get_children() {
            if let Some(node_type) = child.get_node_type_opt() {
                if JavaNodeType::ImportDecl == node_type {
                    let import_route = get_nodes_content(child.to_owned());

                    imports.insert(JavaImport::from_file_import_decl(
                        import_route,
                        java_file_path,
                    ));
                } else if JavaNodeType::PackageDecl == node_type {
                    Self::check_package_def(&java_file_import, child);
                    package_found = true;
                } else if Self::is_structure(node_type) {
                    let structure = JavaStructure::new(child, &imports, java_file_path)?;
                    structure_opt = Some(structure);
                }
            }
        }

        Self::check_existence(package_found, &structure_opt, java_file_path)?;
        let structure = structure_opt.ok_or(
            format!(
                "Java internal structure not found in file:\n\t\"{}\"\n",
                path_helper::try_to_absolute_path(java_file_path)
            )
            .as_str(),
        )?;

        if !java_file_import.match_type_id(structure.get_name()) {
            logger::log_warning(&format!(
                "Mismatch between the identifier \"{}\" and its java file:\n\t\"{}\"\n",
                structure.get_name(),
                path_helper::try_to_absolute_path(java_file_path)
            ));
        }

        Ok(JavaFile {
            self_import: java_file_import,
            #[cfg(test)]
            imports,
            structure,
        })
    }

    pub(crate) fn get_file_path(&self) -> PathBuf {
        self.get_import()
            .get_specific_file()
            .expect("Explicit self java file import always exists")
    }

    fn check_package_def(java_file_import: &JavaImport, child: &JavaNode) {
        let expected_package = java_file_import.get_package_route();
        let expected_package_decl = format!("package {};", expected_package);
        let found_package_decl = child.get_content();
        // TODO: improve this comparison to be spaces independent (using new method in string_helper)
        if found_package_decl != expected_package_decl {
            logger::log_warning(
                format!(
                    "Unrecognized package in file:\n\t\"{}\"\n\nExpected: \"{}\"\nFound: \"{}\"\n",
                    path_helper::try_to_absolute_path(
                        &java_file_import
                            .get_specific_file()
                            .expect("Java file import is associated to a path")
                    ),
                    expected_package_decl,
                    found_package_decl
                )
                .as_str(),
            )
        }
    }

    fn is_structure(node_type: JavaNodeType) -> bool {
        JavaNodeType::ClassDecl == node_type
            || JavaNodeType::InterfaceDeclaration == node_type
            || JavaNodeType::EnumDeclaration == node_type
    }

    pub(crate) fn get_main_structure_type(&self) -> JavaStructureType {
        self.get_structure().get_type().to_owned()
    }

    pub(crate) fn get_import(&self) -> JavaImport {
        self.self_import.to_owned()
    }

    #[cfg(test)]
    fn get_imports(&self) -> &JavaImportsScan {
        &self.imports
    }

    pub(crate) fn get_structure(&self) -> &JavaStructure {
        &self.structure
    }

    fn check_existence(
        package_found: bool,
        structure: &Option<JavaStructure>,
        java_file_path: &Path,
    ) -> Result<(), String> {
        if !package_found {
            return Err(format!(
                "Java package not found in file:\n\t\"{}\"\n",
                path_helper::try_to_absolute_path(java_file_path)
            ));
        }
        if structure.is_none() {
            return Err(format!(
                "Java structure not found in file:\n\t\"{}\"\n",
                path_helper::try_to_absolute_path(java_file_path)
            ));
        }

        Ok(())
    }
}

fn get_nodes_content(import_decl_node: JavaNode) -> String {
    if Some(JavaNodeType::ImportDecl) != import_decl_node.get_node_type_opt() {
        panic!("Java import declaration node required")
    }

    for children_level_one in import_decl_node.get_children() {
        if Some(JavaNodeType::ScopedIdentifier) == children_level_one.get_node_type_opt() {
            return children_level_one.get_content();
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_java_project_test_folder;
    use crate::java::scanner::file::java_file::JavaFile;
    use crate::java::scanner::file::java_structure_type::JavaStructureType;

    #[test]
    fn scan_basic_application() {
        let dir_path = get_test_folder().join("BasicApplication.java");

        match JavaFile::from_user_input_path(&dir_path) {
            Ok(java_file) => {
                check_basic_application_java_file(java_file);
            }
            Err(e) => assert_fail(&e),
        }
    }

    fn check_basic_application_java_file(java_file: JavaFile) {
        //assert_eq!("org.test", java_file.get_package().to_string());
        assert_eq!(2, java_file.get_imports().count());
        let structure = java_file.get_structure();
        assert_eq!(JavaStructureType::Class, structure.get_type());

        let annotations = structure.get_annotations();
        assert_eq!(1, annotations.len());
        if let Some(annotation) = annotations.first() {
            assert_eq!("SpringBootApplication", annotation.get_name())
        }
    }

    #[test]
    fn scan_basic_enum() {
        let dir_path = get_test_folder().join("BasicEnumName.java");

        match JavaFile::from_user_input_path(&dir_path) {
            Ok(java_file) => {
                check_basic_enum_java_file(java_file);
            }
            Err(e) => assert_fail(&e),
        }
    }

    fn get_test_folder() -> PathBuf {
        get_java_project_test_folder(get_current_file_path(), "java_file")
    }

    fn check_basic_enum_java_file(java_file: JavaFile) {
        //assert_eq!("org.test", java_file.get_package().to_string());
        assert_eq!(0, java_file.get_imports().count());
        let structure = java_file.get_structure();
        assert_eq!(0, structure.get_annotations().len());
        assert_eq!(JavaStructureType::Enum, structure.get_type());
    }

    #[test]
    fn scan_invalid() {
        let dir_path = get_test_folder().join("Invalid.java");

        match JavaFile::from_user_input_path(&dir_path) {
            Ok(_) => assert_fail("It should not return a valid java file struct"),
            Err(e) => assert!(e.contains("Java package not found in file")),
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
