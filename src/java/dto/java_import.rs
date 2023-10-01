use std::fmt;
use std::path::{Component, Path, PathBuf};

use crate::core::file_system::file_browsing::file_browser;
use crate::core::file_system::path_helper;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::java_node::JavaNode;
use crate::java::scanner::package::java_dependency_scanner;

/// At the moment JavaImport only supports explicit references to files (i.e. classes, interfaces, enums).
/// Class methods and substructures are not supported yet.
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct JavaImport {
    /// Case 1: hard coded imports without explicit file references
    /// (i.e. "org.test.JavaClassFrom")
    ///
    /// TODO: try to store origin file, a route is not unique, it depends
    /// on the file where it is declared because not all packages are available from all files
    /// -> Still need to implement multi-module support for inconsistencies.
    fake_non_checked_route: String,

    // Case 2: imports from specific file/folder with verified java project
    // If the folder_path is None it represents a hardcoded import without dependency tree analysis
    folder_path: Option<PathBuf>,
    nodes: Vec<String>, // ["JavaClass", "JavaSubClass", "JavaSubClassMethod", ...]
}

// Public crate methods
impl JavaImport {
    pub(crate) fn new_explicit_import_from_file(file_path: &Path) -> Result<JavaImport, String> {
        Self::new_explicit_import_from_file_internal(&file_path)
    }

    /// Warning: this method java_dependency_scanner::search_imports can handle only imports
    /// in the same java project than java_file_path (multi-module not supported)
    pub(crate) fn from_file_import_decl(import_route: String, java_file_path: &Path) -> JavaImport {
        // Warning: this method java_dependency_scanner::search_imports can handle only imports
        // in the same java project than java_file_path (multi-module not supported)
        Self::from_file_import_decl_internal(&import_route, java_file_path)
            .expect("Java import must be returned")
    }

    pub(crate) fn new_explicit_import_from_scoped_identifier(
        scoped_identifier: &JavaNode,
        input_java_file: &Path,
    ) -> JavaImport {
        let route = scoped_identifier.get_content();
        Self::from_file_import_decl(route, input_java_file)
    }

    /// TODO: scan m2 repository after parse
    /// This method lacks contest from the definition route of the class/interface/enum
    /// i.e. which submodule is this route coming from? -> Not possible to detect with this header
    /// It is used to create well-known imports like "org.springframework.stereotype.Service"
    /// but should not be used in real code.
    #[allow(dead_code)]
    pub(crate) fn new_explicit_import_requiring_m2_repo_scan(
        route: &str,
    ) -> Result<JavaImport, String> {
        Self::new_explicit_import_without_m2_repo_scan(route)
    }

    #[allow(dead_code)]
    pub(crate) fn new_explicit_import_without_m2_repo_scan(
        route: &str,
    ) -> Result<JavaImport, String> {
        let import = JavaImport::new_from_route(route);
        Self::check_if_explicit_import(&import)?;
        Ok(import)
    }
}

impl JavaImport {
    fn new_explicit_import_from_file_internal(file_path: &&Path) -> Result<JavaImport, String> {
        check_file_for_new_explicit_import(file_path)?;

        let mut dir_path = file_path.to_owned().to_path_buf();
        dir_path.pop();
        if get_package_nodes_vec_from_dir(&dir_path).is_empty() {
            return Err(invalid_explicit_import_msg(file_path));
        }
        let last_node = get_last_item_string(file_path);

        Ok(JavaImport {
            fake_non_checked_route: "".to_string(),
            folder_path: Some(dir_path.to_owned()),
            nodes: vec![file_browser::remove_java_extension(last_node)],
        })
    }

    fn from_file_import_decl_internal(
        import_route: &String,
        java_file_path: &Path,
    ) -> Result<JavaImport, String> {
        let imports = java_dependency_scanner::search_imports(import_route, java_file_path);
        if imports.len() > 1 {
            logger::log_warning(
                format!(
                    "Several import possibilities found for import \"{}\" in file:\n\"{}\"\n",
                    import_route,
                    try_to_absolute_path(java_file_path)
                )
                    .as_str(),
            );
        } else if let Some(java_import_route) = imports.get(0) {
            let java_import_route_path = java_import_route.to_file_path();

            return Self::new_explicit_import_from_file(&java_import_route_path);
        }

        Ok(Self::new_from_route(import_route))
    }

    fn check_if_explicit_import(import: &JavaImport) -> Result<(), String> {
        if !import.is_explicit_import() {
            return Err(format!(
                "Invalid attempt to create an explicit java import:\n\t\"{:?}\"",
                import.fake_non_checked_route
            ));
        }
        Ok(())
    }

    pub(crate) fn get_specific_file(&self) -> Result<PathBuf, String> {
        if !self.is_explicit_import() {
            return Err(format!(
                "Java import \"{}\" must be explicit to the its specific file.",
                self
            ));
        }

        if let Some(folder) = self.folder_path.to_owned() {
            if let Some(first_node) = self.nodes.get(0) {
                return Ok(folder.join(format!("{}.java", first_node)));
            }
        }

        Err(format!("Specific file not found for import \"{}\"", self))
    }

    pub(crate) fn is_explicit_import(&self) -> bool {
        if self.folder_path.is_some() {
            return !self.nodes.is_empty();
        }

        if let Some(last_node) = self.get_nodes().last() {
            return !last_node.eq("*");
        }
        false
    }

    fn get_all_nodes(&self) -> Vec<String> {
        if self.folder_path.is_some() {
            let mut all_nodes = self.get_package_nodes_vec();
            for node in self.nodes.iter().clone() {
                all_nodes.push(node.to_string());
            }

            return all_nodes;
        }

        self.get_all_fake_nodes()
    }

    fn get_nodes(&self) -> Vec<String> {
        if self.folder_path.is_some() {
            return self.nodes.to_owned();
        }

        self.get_all_fake_nodes()
    }

    pub(crate) fn is_wildcard_import(&self) -> bool {
        if self.folder_path.is_some() {
            return self.nodes.is_empty();
        }

        if let Some(last_node) = self.get_nodes().last() {
            return last_node.eq("*");
        }
        false
    }

    fn new_from_route(route: &str) -> JavaImport {
        if split_to_nodes(route).is_empty() {
            logger::log_unrecoverable_error(&format!(
                "Invalid java import found:\n\t\"{:?}\"",
                route
            ));
        }
        JavaImport {
            fake_non_checked_route: route.to_string(),
            folder_path: None,
            nodes: vec![],
        }
    }

    pub(crate) fn get_package_nodes_vec(&self) -> Vec<String> {
        get_package_nodes_vec_from_dir(&self.folder_path.to_owned().unwrap())
    }

    /// TODO: The signature of this method prevents its validity
    /// It is not possible to fully recognize an import with type_id
    /// since the type could be a substructure JavaClass.JavaSubclass, JavaClass.method...
    /// The only valid way to validate this match is to check all self.nodes with the type_id(s)
    pub(crate) fn match_type_id(&self, type_id: &str) -> bool {
        let all_nodes = self.get_all_nodes().to_owned();
        if let Some(last_node) = all_nodes.last() {
            return last_node == type_id;
        }
        false
    }

    // Relates to fake_non_checked_route
    pub(crate) fn get_last_node(&self) -> String {
        self.get_all_nodes()
            .last()
            .expect("Last node must exist in java import")
            .to_string()
    }

    fn get_all_fake_nodes(&self) -> Vec<String> {
        split_to_nodes(&self.fake_non_checked_route)
    }

    pub(crate) fn get_package_route(&self) -> String {
        self.get_route_internal(true)
    }

    pub(crate) fn get_route(&self) -> String {
        self.get_route_internal(false)
    }

    pub(crate) fn get_route_internal(&self, just_package_route: bool) -> String {
        if self.folder_path.is_none() {
            // TODO: fix this when just_package_route=true (it does not recognize folder+typeIds division)
            return self.fake_non_checked_route.to_string();
        }

        let package_nodes = self.get_package_nodes_vec();
        let mut package_nodes_str = package_nodes.join(".");

        if !just_package_route {
            if !self.nodes.is_empty() {
                package_nodes_str += ".";
                package_nodes_str += self.nodes.join(".").as_str();
            } else {
                package_nodes_str += ".*";
            }
        }

        package_nodes_str
    }
}

fn invalid_explicit_import_msg(file_path: &&Path) -> String {
    format!(
        "Invalid attempt to create an explicit java import using a file not associated to a java project:\n\"{:?}\"",
        path_helper::try_to_absolute_path(file_path)
    )
}

fn get_last_item_string(file_path: &Path) -> String {
    file_path
        .iter()
        .last()
        .expect("There must be a last file element")
        .to_string_lossy()
        .to_string()
}

fn check_file_for_new_explicit_import(file_path: &Path) -> Result<(), String> {
    if !file_path.exists() {
        return Err(format!(
            "Can not create an explicit java import using a file that does not exist:\n\t\"{:?}\"\n",
            path_helper::try_to_absolute_path(file_path)
        ));
    }

    if !file_path.is_file() {
        return Err(format!(
            "Can not create an explicit java import using non file input:\n\t\"{:?}\"\n",
            path_helper::try_to_absolute_path(file_path)
        ));
    }

    if !file_browser::do_last_element_in_path_ends_with(file_path, ".java") {
        return Err(format!(
            "Can not create an explicit java import using non java file:\n\t\"{:?}\"\n",
            path_helper::try_to_absolute_path(file_path)
        ));
    }
    Ok(())
}

pub fn get_package_nodes_vec_from_dir(dir_path: &Path) -> Vec<String> {
    if !dir_path.exists() || !dir_path.is_dir() {
        logger::log_unrecoverable_error(
            format!(
                "Java import related to an invalid folder:\n\t\"{}\"\n",
                path_helper::try_to_absolute_path(dir_path)
            )
                .as_str(),
        );
    }

    for ancestor in dir_path.ancestors() {
        if ancestor.ends_with("java") {
            if let Some(second_ancestor) = ancestor.parent() {
                if second_ancestor.ends_with("main") {
                    if let Some(third_ancestor) = second_ancestor.parent() {
                        if third_ancestor.ends_with("src") {
                            let ancestor_vec: Vec<_> = ancestor.components().collect();
                            let dir_path_vec: Vec<_> = dir_path.components().collect();

                            let difference_component_vec: Vec<Component> =
                                dir_path_vec[ancestor_vec.len()..].to_vec();
                            let mut package_nodes = Vec::new();
                            for difference_item in difference_component_vec {
                                package_nodes.push(
                                    difference_item.as_os_str().to_string_lossy().to_string(),
                                );
                            }
                            return package_nodes;
                        }
                    }
                }
            }
        }
    }

    logger::log_unrecoverable_error(
        format!(
            "Trying to create a java import that does not belong to any java project:\n\t\"{}\"\n",
            path_helper::try_to_absolute_path(dir_path)
        )
            .as_str(),
    );
    Vec::new() // Fake return, it will stop in log_unrecoverable_error
}

// Relates to fake_non_checked_route
fn split_to_nodes(content: &str) -> Vec<String> {
    content.split('.').map(|str| str.to_string()).collect()
}

impl fmt::Display for JavaImport {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("import ")?;

        for (idx, children) in self.get_all_nodes().iter().enumerate() {
            if idx != 0 {
                fmt.write_str(".")?;
            }
            fmt.write_str(children)?;
        }
        if self.folder_path.is_some() && self.nodes.is_empty() {
            fmt.write_str(".*")?;
        }
        fmt.write_str(";")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_java_project_test_folder;
    use crate::java::dto::java_import::JavaImport;
    use crate::java::scanner::file::java_file::JavaFile;

    /// Tests with hard coded imports
    #[test]
    fn new_explicit_import() {
        let import = JavaImport::new_explicit_import_requiring_m2_repo_scan("org.test")
            .expect("Java explicit import is valid");

        assert!(import.is_explicit_import());
        assert!(!import.is_wildcard_import());
        assert_eq!("org.test", import.get_route());
    }

    #[test]
    fn new_wildcard_import() {
        let import = JavaImport::new_from_route("org.test.*");

        assert!(import.is_wildcard_import());
        assert!(!import.is_explicit_import());
        assert_eq!("org.test.*", import.get_route());
    }

    #[test]
    fn get_last_node() {
        let import =
            JavaImport::new_explicit_import_requiring_m2_repo_scan("org.test.LastNodeClass")
                .expect("Java explicit import is valid");

        assert!(import.is_explicit_import());
        assert!(!import.is_wildcard_import());
        assert_eq!("org.test.LastNodeClass", import.get_route());
        assert_eq!("LastNodeClass", import.get_last_node());
    }

    #[test]
    fn to_string_hardcoded_import() {
        let import = JavaImport::new_explicit_import_requiring_m2_repo_scan("org.test.Class")
            .expect("Java explicit import is valid");

        assert_eq!("import org.test.Class;", import.to_string());
    }

    ///// Other tests:

    #[test]
    fn new_explicit_import_from_file_test() {
        let file_path = get_test_folder().join("JavaImportClass.java");

        match JavaImport::new_explicit_import_from_file(&file_path) {
            Ok(import) => {
                assert!(import.is_explicit_import());
                assert!(!import.is_wildcard_import());
                assert_eq!("org.test.JavaImportClass", import.get_route());
                assert_eq!("org.test", import.get_package_route());
                assert_eq!("import org.test.JavaImportClass;", import.to_string());
                assert!(import.match_type_id("JavaImportClass"));
                assert!(!import.match_type_id("JavaImportClassFake"));
            }
            Err(err) => assert_fail(&err),
        };
    }

    #[test]
    fn java_entity_full_scoped_identifier() {
        let file_path = get_test_folder().join("JavaEntityFullScopedIdentifier.java");

        let java_file = JavaFile::from_user_input_path(&file_path).expect("Valid file scan");

        let annotations = java_file.get_structure().get_annotations();
        assert_eq!(1, annotations.len());
        match annotations.get(0) {
            None => assert_fail("Annotation expected"),
            Some(annotation) => {
                assert_eq!(
                    "jakarta.persistence.Entity",
                    annotation.get_self_import().get_package_route()
                )
            }
        }
    }

    #[test]
    fn java_method_with_parameter_scoped_identifier() {
        let file_path = get_test_folder().join("JavaMethodWithParameterScopedIdentifier.java");

        let java_file = JavaFile::from_user_input_path(&file_path).expect("Valid file scan");

        let methods = java_file.get_structure().get_methods();
        assert_eq!(1, methods.len());
        match methods.get(0) {
            None => assert_fail("Method expected"),
            Some(method) => {
                let parameters = method.get_parameters();
                assert_eq!(1, parameters.len());
                let parameter = parameters.get(0).expect("Parameter expected");
                assert_eq!(
                    "jakarta.persistence.Entity",
                    parameter
                        .get_import()
                        .expect("Parameter import")
                        .get_route()
                )
            }
        }
    }

    #[test]
    fn java_method_with_return_type_scoped_identifier() {
        let file_path = get_test_folder().join("JavaMethodWithReturnTypeScopedIdentifier.java");

        let java_file = JavaFile::from_user_input_path(&file_path).expect("Valid file scan");

        let methods = java_file.get_structure().get_methods();
        assert_eq!(1, methods.len());
        let method = methods.get(0).expect("Method expected");

        let return_type = &method.get_return_type().to_owned().expect("Expected return type");
        let route = return_type.get_import().expect("Return type import expected").get_route();
        assert_eq!("jakarta.persistence.Entity", route);

        let parameters = method.get_parameters();
        assert_eq!(1, parameters.len());
        let parameter = parameters.get(0).expect("First parameter expected");
        let route = parameter.get_import().expect("Parameter import expected").get_route();
        assert_eq!("jakarta.persistence.Entity", route);
    }

    fn get_test_folder() -> PathBuf {
        get_java_project_test_folder(get_current_file_path(), "java_import")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
