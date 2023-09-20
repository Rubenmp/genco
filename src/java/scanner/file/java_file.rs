use std::path::{Path, PathBuf};

use crate::core::file_system::file_creation::file_creator;
use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
use crate::core::file_system::path_helper;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_method::JavaMethod;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::parser::java_parser;
use crate::java::scanner::file::java_file_imports::JavaFileImports;
use crate::java::scanner::file::java_structure::JavaStructure;
use crate::java::scanner::file::java_structure_type::JavaStructureType;
use crate::java::scanner::package::{java_dependency_scanner, java_package_scanner};

#[derive(Debug)]
pub(crate) struct JavaFile {
    file: PathBuf,
    imports: JavaFileImports,
    structure: JavaStructure,
}

impl JavaFile {
    pub(crate) fn write(file: &Path, input_structure: JavaStructure) -> Result<Self, String> {
        let imports = input_structure.get_imports_sorted_asc();
        Self::write_to_file_internal(&file, &imports, &input_structure)?;

        JavaFile::from_user_input_path(file)
    }

    pub(crate) fn from_user_input_path(java_file_path: &Path) -> Result<Self, String> {
        let base_java_project_dir_opt = java_package_scanner::get_base_package(java_file_path);
        if let Some(base_java_project_dir) = base_java_project_dir_opt {
            java_dependency_scanner::recursive_scan_dir_unchecked(&base_java_project_dir);
        } else {
            return Err(format!(
                "Invalid java project file:\n\"{}\"\n",
                path_helper::try_to_absolute_path(java_file_path)
            ));
        }

        let root_java_node = java_parser::parse(java_file_path)?;
        let mut package_found = false;
        let mut imports = JavaFileImports::new();
        let mut structure_opt: Option<JavaStructure> = None;
        let java_file_import = JavaImport::new_explicit_import_from_file(java_file_path)?;

        for child in root_java_node.get_children() {
            if let Some(node_type) = child.get_node_type_opt() {
                if JavaNodeType::ImportDecl == node_type {
                    let import_route = get_nodes_content(child.to_owned());

                    imports.insert(
                        JavaImport::from_file_import_decl(import_route, java_file_path),
                        child.get_end_byte(),
                    );
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
            file: java_file_path.to_path_buf(),
            imports,
            structure,
        })
    }

    /// # write_to_file
    /// Export java structure into a specific directory "export_directory"
    /// that must be inside a java project, creating a java file with
    /// the name of the structure.
    fn write_to_file_internal(
        file_path: &Path,
        java_file_imports: &Vec<JavaImport>,
        structure: &JavaStructure,
    ) -> Result<(), String> {
        validate_output_file(file_path)?;
        let mut result = write_package(file_path);
        write_imports(&mut result, java_file_imports);

        result += structure.get_skeleton_without_imports().as_str();
        structure.write_body(&mut result);
        write_to_file_internal(file_path, &result);
        Ok(())
    }

    pub(crate) fn get_file_path(&self) -> &PathBuf {
        &self.file
    }

    pub(crate) fn insert_method(&mut self, method: &JavaMethod) -> Result<JavaFile, String> {
        // TODO: validate if java_file changed before inserting method (still exist?)
        let mut to_overwrite = FileOverwriting::new(self.get_file_path());
        let method_imports = method.get_imports();
        let mut byte_to_insert_first_import_opt = None;
        if method_imports.is_empty() {
            match self.get_byte_to_insert_first_import() {
                Ok(result_byte) => byte_to_insert_first_import_opt = Some(result_byte),
                Err(err) => {
                    return Err(err);
                }
            };
        }
        self.imports.add_missing_imports(
            &mut to_overwrite,
            method_imports,
            byte_to_insert_first_import_opt,
        );
        //todo!();
        // Add java_method to to_overwrite variable;

        to_overwrite.write_all(); // TODO: return possible error from this
        let result_file = JavaFile::from_user_input_path(self.get_file_path())?;
        Ok(result_file)
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

    pub(crate) fn get_self_import(&self) -> JavaImport {
        JavaImport::new_explicit_import_from_file(self.get_file_path())
            .expect("Java structure must have a java import associated")
    }

    fn get_file_imports(&self) -> &JavaFileImports {
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
    fn get_byte_to_insert_first_import(&self) -> Result<usize, String> {
        let java_file_path = self.get_file_path();
        let root_java_node = java_parser::parse(java_file_path)?;

        for child in root_java_node.get_children() {
            if let Some(node_type) = child.get_node_type_opt() {
                if JavaNodeType::PackageDecl == node_type {
                    return Ok(child.get_end_byte());
                } else if Self::is_structure(node_type) {
                    return Ok(child.get_start_byte());
                }
            }
        }

        Err("It was not possible to determine the position of the new java import".to_string())
    }
}

fn validate_output_file(file: &Path) -> Result<(), String> {
    if file.exists() && file.is_dir() {
        return Err(format!(
            "expecting an output file but a dir was found:\n{}\n",
            path_helper::try_to_absolute_path(file)
        ));
    }

    Ok(())
}

fn write_package(file_path: &Path) -> String {
    let mut result = "package ".to_string();
    result += java_package_scanner::get_package_from_dir(&get_dir(file_path)).as_str();
    result += ";\n\n";
    result
}

fn write_imports(result: &mut String, imports: &Vec<JavaImport>) {
    for import in imports {
        *result += import.to_string().as_str();
        *result += "\n";
    }
    if !imports.is_empty() {
        *result += "\n";
    }
}

fn write_to_file_internal(file_path: &Path, result: &str) {
    if file_path.exists() && file_path.is_file() {
        file_creator::remove_file_if_exists(file_path);
    }

    file_creator::create_file_if_not_exist(file_path);
    let mut overwriting = FileOverwriting::new(file_path);
    overwriting.append(result);
    overwriting.write_all();
}

fn get_dir(input_path: &Path) -> PathBuf {
    let mut path = input_path.to_path_buf();
    path.pop();
    path.to_owned()
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
        assert_eq!(2, java_file.get_file_imports().count());
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
        assert_eq!(0, java_file.get_file_imports().count());
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
