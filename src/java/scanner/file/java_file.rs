use std::path::{Path, PathBuf};

use crate::core::file_system::file_cache::FileCache;
use crate::core::file_system::file_edition::file_editor;
use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
use crate::core::file_system::path_helper;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::import::JavaImport;
use crate::java::indentation_config::JavaIndentation;
use crate::java::method::JavaMethod;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
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

// Public crate methods
impl JavaFile {
    /// This method returns a JavaFile where
    /// - Package declaration is not required
    pub(crate) fn from_user_input_path(java_file_path: &Path) -> Result<Self, String> {
        Self::from_user_input_path_internal(java_file_path)
    }

    pub(crate) fn write(file: &Path, input_structure: JavaStructure) -> Result<Self, String> {
        Self::write_internal(file, &input_structure)?
    }

    pub(crate) fn copy_to_output_folder(&self, output_java_folder: &Path) -> Result<Self, String> {
        self.copy_to_output_folder_internal(output_java_folder)
    }

    pub(crate) fn insert_method(&mut self, method: &JavaMethod) -> Result<JavaFile, String> {
        self.insert_method_internal(method)
    }

    pub(crate) fn get_file_path(&self) -> &PathBuf {
        &self.file
    }

    pub(crate) fn get_main_structure_type(&self) -> JavaStructureType {
        self.get_structure().get_type()
    }

    pub(crate) fn get_self_import(&self) -> JavaImport {
        JavaImport::new_explicit_import_from_file(self.get_file_path())
            .expect("Java structure must have a java import associated")
    }

    pub(crate) fn get_structure(&self) -> &JavaStructure {
        &self.structure
    }
}

// Private methods
impl JavaFile {
    fn write_internal(
        file: &Path,
        input_structure: &JavaStructure,
    ) -> Result<Result<JavaFile, String>, String> {
        validate_output_file(file)?;

        let imports = input_structure.get_imports_sorted_asc();
        Self::write_to_file_internal(file, &imports, input_structure)?;

        Ok(JavaFile::from_user_input_path(file))
    }

    fn from_user_input_path_internal(java_file_path: &Path) -> Result<JavaFile, String> {
        if let Some(base_java_project_dir) = java_package_scanner::get_base_package(java_file_path)
        {
            java_dependency_scanner::recursive_scan_dir_unchecked(&base_java_project_dir);
        } else {
            return Err(Self::get_invalid_java_project_file_error(java_file_path));
        }

        let file_cache = FileCache::from(java_file_path);
        let root_java_node = JavaNode::from_path(java_file_path)?;
        root_java_node.print_tree_and_panic();
        let mut imports = JavaFileImports::new();
        let mut structure_opt: Option<JavaStructure> = None;
        let java_file_import = JavaImport::new_explicit_import_from_file(java_file_path)?;

        for child in root_java_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if JavaNodeType::ImportDecl == node_type {
                    match JavaNode::get_import_decl_content(child, &file_cache) {
                        Ok(import_route) => imports.insert(
                            JavaImport::from_file_import_decl(import_route, &file_cache),
                            child.get_end_byte(),
                        ),
                        Err(err) => log_invalid_import(java_file_path, err),
                    };
                } else if JavaNodeType::PackageDecl == node_type {
                    Self::check_package_def(&java_file_import, child, &file_cache);
                } else if node_type.is_structure() {
                    let structure = JavaStructure::new(child, &imports, &file_cache)?;
                    structure_opt = Some(structure);
                }
            }
        }

        Self::check_existence(&structure_opt, java_file_path)?;
        let structure = structure_opt.ok_or(Self::get_structure_not_found_error(java_file_path))?;
        Self::log_java_file_package_mismatch_if_needed(
            &java_file_path,
            java_file_import,
            &structure,
        );

        Ok(JavaFile {
            file: java_file_path.to_path_buf(),
            imports,
            structure,
        })
    }

    fn log_java_file_package_mismatch_if_needed(
        java_file_path: &&Path,
        java_file_import: JavaImport,
        structure: &JavaStructure,
    ) {
        if !java_file_import.match_type_id(structure.get_name()) {
            logger::log_warning(&format!(
                "Mismatch between the identifier \"{}\" and its java file:\n\t\"{}\"\n",
                structure.get_name(),
                path_helper::try_to_absolute_path(java_file_path)
            ));
        }
    }

    fn get_structure_not_found_error(java_file_path: &Path) -> String {
        format!(
            "Java internal structure not found in file:\n\t\"{}\"\n",
            path_helper::try_to_absolute_path(java_file_path)
        )
    }

    fn get_invalid_java_project_file_error(java_file_path: &Path) -> String {
        format!(
            "Invalid java project file:\n\"{}\"\n",
            path_helper::try_to_absolute_path(java_file_path)
        )
    }

    fn copy_to_output_folder_internal(
        &self,
        output_java_folder: &Path,
    ) -> Result<JavaFile, String> {
        let input_java_file = self.get_file_path();
        if output_java_folder.exists() && !output_java_folder.is_dir() {
            return Err(format!("Not possible to copy java file:\n\"{}\"\ninto existing path that it is not a directory:\n\"{}\"\n", try_to_absolute_path(input_java_file), try_to_absolute_path(output_java_folder)));
        }

        let output_package_route_opt =
            java_package_scanner::get_package_route_opt_from_dir_no_check(output_java_folder);
        let output_package_route = output_package_route_opt.ok_or(format!(
            "Invalid output folder outside of java project:\n\"{}\"\n",
            try_to_absolute_path(output_java_folder)
        ))?;
        let file_name = input_java_file.iter().last().ok_or("Exists last item in input_java_file")?.to_str().ok_or(format!("Not possible to copy invalid java file\n\"{}\"\n, name does not represent a valid Unicode sequence", try_to_absolute_path(input_java_file)))?;
        let output_file = output_java_folder.join(file_name);
        let mut file_overwrite = FileOverwriting::from_unchecked_path(input_java_file);

        match java_package_scanner::find_package_start_end_bytes(input_java_file) {
            Ok(package_bytes) => {
                file_overwrite.replace(package_bytes.0, package_bytes.1, &output_package_route)
            }
            Err(_) => {
                let package_decl = format!("package {};\n\n", output_package_route);
                file_overwrite.insert_content_at(0, &package_decl)?;

                Ok(())
            }
        }?;

        file_overwrite.write_all_to_file(&output_file)?;

        Self::from_user_input_path(&output_file)
    }

    fn check_package_def(java_file_import: &JavaImport, child: &JavaNode, file_cache: &FileCache) {
        let expected_package = java_file_import.get_package_route();
        let expected_package_decl = format!("package {};", expected_package);
        let found_package_decl = child.get_content_from_cache(file_cache);
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

    fn insert_method_internal(&mut self, method: &JavaMethod) -> Result<JavaFile, String> {
        // TODO: validate if java_file changed before inserting method (still exist?)
        let mut to_overwrite = FileOverwriting::from_path(self.get_file_path())?;
        let mut byte_to_insert_first_import_opt = None;
        if self.get_file_imports().is_empty() {
            match self.get_byte_to_insert_first_import() {
                Ok(result_byte) => byte_to_insert_first_import_opt = Some(result_byte),
                Err(err) => {
                    return Err(err);
                }
            };
        }
        self.imports.add_missing_imports(
            &mut to_overwrite,
            method.get_imports(),
            byte_to_insert_first_import_opt,
        )?;

        let mut method_str = "\n".to_string();
        let mut initial_method_indentation = JavaIndentation::default();
        initial_method_indentation.increase_level();
        method.write_to_string(&mut method_str, &initial_method_indentation);
        to_overwrite.insert_content_with_previous_newline_at(
            self.get_structure().get_start_byte(),
            &method_str,
        )?;

        to_overwrite.write_all()?;

        let result_file = JavaFile::from_user_input_path(self.get_file_path())?;
        Ok(result_file)
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
        let mut result = write_package(file_path);
        result +=
            JavaFileImports::get_file_string_with_trailing_newline(java_file_imports).as_str();
        result += structure.get_skeleton_without_imports().as_str();
        structure.write_body(&mut result);

        file_editor::create_or_replace_file_with_bytes(file_path, result.as_bytes())
    }

    fn get_file_imports(&self) -> &JavaFileImports {
        &self.imports
    }

    fn check_existence(
        structure: &Option<JavaStructure>,
        java_file_path: &Path,
    ) -> Result<(), String> {
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
        let root_java_node = JavaNode::from_path(java_file_path)?;

        for child in root_java_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if JavaNodeType::PackageDecl == node_type {
                    return Ok(child.get_end_byte());
                } else if node_type.is_structure() {
                    return Ok(child.get_start_byte());
                }
            }
        }

        Err("It was not possible to determine the position of the new java import".to_string())
    }
}

fn log_invalid_import(java_file_path: &Path, err: String) {
    logger::log_warning(
        format!(
            "Invalid import ({}) in file:\n{}\n",
            err,
            try_to_absolute_path(java_file_path)
        )
        .as_str(),
    )
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
    let route_opt = java_package_scanner::get_package_route_from_file(file_path);
    let route = route_opt.unwrap_or("".to_string());

    let mut result = "package ".to_string();
    result += route.as_str();
    result += ";\n\n";
    result
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::{assert_fail, assert_same_file};
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

    fn check_basic_enum_java_file(java_file: JavaFile) {
        //assert_eq!("org.test", java_file.get_package().to_string());
        assert_eq!(0, java_file.get_file_imports().count());
        let structure = java_file.get_structure();
        assert_eq!(0, structure.get_annotations().len());
        assert_eq!(JavaStructureType::Enum, structure.get_type());
    }

    #[test]
    fn scan_invalid() {
        let invalid_java_file = get_test_folder().join("Invalid.java");

        match JavaFile::from_user_input_path(&invalid_java_file) {
            Ok(_) => assert_fail("It should not return a valid java file struct"),
            Err(e) => assert!(e.contains("Java structure not found in file")),
        }
    }

    #[test]
    fn copy_to_output_folder_succeed() {
        let structure_name = "CopyToOutputFolder";
        let expected_file_content =
            get_expected_test_folder().join("ExpectedCopyToOutputFolder.java");

        test_copy_to_output_folder(structure_name, expected_file_content);
    }

    #[test]
    fn copy_to_output_folder_file_without_package_succeed() {
        let structure_name = "CopyToOutputFolderWithoutPackage";
        let expected_file_content =
            get_expected_test_folder().join("ExpectedCopyToOutputFolderWithoutPackage.java");

        test_copy_to_output_folder(structure_name, expected_file_content);
    }

    fn test_copy_to_output_folder(structure_name: &str, expected_file_content: PathBuf) {
        let initial_file_path = get_test_folder().join(format!("{}.java", structure_name));
        let output_folder = get_test_folder().join("copy_to_output_folder_test");
        let input_java_file = JavaFile::from_user_input_path(&initial_file_path)
            .expect("Java file scan must succeed");

        match input_java_file.copy_to_output_folder(&output_folder) {
            Ok(copied_java_file) => {
                assert_eq!(
                    structure_name,
                    copied_java_file.get_self_import().get_last_node()
                );
                assert_same_file(&expected_file_content, copied_java_file.get_file_path());
                //remove_dir_all(output_folder).expect("output_folder should have been created and removed");
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_expected_test_folder() -> PathBuf {
        get_test_folder().join("expected")
    }

    fn get_test_folder() -> PathBuf {
        get_java_project_test_folder(get_current_file_path(), "java_file")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
