use std::fmt;
use std::path::{Component, Path, PathBuf};

use crate::core::file_system::file_browser::file_browser;
use crate::core::file_system::path_helper;
use crate::core::observability::logger::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct JavaImport {
    // Case 1: hard coded imports without explicit file references
    fake_non_checked_route: String,

    // Case 2: imports from specific file/folder with verified java project
    // If the folder_path is None it represents a hardcoded import without dependency tree analysis
    folder_path: Option<PathBuf>,
    nodes: Vec<String>, // ["JavaClass", "JavaSubClass", "JavaSubClassMethod", ...]
}

impl JavaImport {
    pub(crate) fn new_explicit_import_from_file(file_path: &Path) -> Result<JavaImport, String> {
        check_file_for_new_explicit_import(file_path)?;

        let mut dir_path = file_path.to_owned().to_path_buf();
        dir_path.pop();
        if get_package_nodes_vec_from_dir(&dir_path).is_empty() {
            return Err(invalid_explicit_import_msg(&file_path));
        }
        let last_node = get_last_item_string(file_path);

        Ok(JavaImport {
            fake_non_checked_route: "".to_string(),
            folder_path: Some(dir_path.to_owned()),
            nodes: vec![remove_java_extension(last_node)],
        })
    }

    /// TODO: integrate with JavaDependencyTree
    pub(crate) fn from_file_import_decl(
        import_decl_node: &JavaNode,
        _java_file_path: &Path,
    ) -> JavaImport {
        let content = get_nodes_content(import_decl_node.to_owned());
        Self::new_from_route(&content)
    }

    pub(crate) fn new_wildcard_import_from_dir(dir_path: &Path) -> Result<JavaImport, String> {
        check_dir_for_new_wildcard_import(dir_path)?;
        if get_package_nodes_vec_from_dir(dir_path).is_empty() {
            return Err(format!(
                "Invalid attempt to create a wildcard java import using folder:\n\t\"{:?}\"",
                path_helper::to_absolute_path_str(dir_path)
            ));
        }

        Ok(JavaImport {
            fake_non_checked_route: "".to_string(),
            folder_path: Some(dir_path.to_owned()),
            nodes: vec![],
        })
    }

    // This method lacks contest from the definition route of the class/interface/enum
    // i.e. which submodule is this route coming from? -> Not possible to detect with this header
    // It is used to create well-known imports like "org.springframework.stereotype.Service"
    pub(crate) fn new_explicit_import_requiring_m2_repo_scan(
        route: &str,
    ) -> Result<JavaImport, String> {
        // TODO: scan m2 repository after parse
        let import = JavaImport::new_from_route(route);
        if !import.is_explicit_import() {
            return Err(format!(
                "Invalid attempt to create an explicit java import:\n\t\"{:?}\"",
                import.fake_non_checked_route
            ));
        }
        Ok(import)
    }
}

impl JavaImport {
    pub(crate) fn get_specific_file(&self) -> Result<PathBuf, String> {
        if !self.is_explicit_import() {
            return Err(format!(
                "Java import \"{}\" must be explicit to the its specific file.",
                self.to_string()
            ));
        }

        if let Some(folder) = self.folder_path.to_owned() {
            dbg!(format!(
                "Java import folder: {}",
                path_helper::to_absolute_path_str(&folder)
            ));
            if let Some(first_node) = self.nodes.get(0) {
                return Ok(folder.join(format!("{}.java", first_node)));
            }
        }
        Err(format!(
            "Specific file not found for import \"{}\"",
            self.to_string()
        ))
    }
    pub(crate) fn is_explicit_import(&self) -> bool {
        if self.folder_path.is_some() {
            return !self.nodes.is_empty();
        }

        if let Some(last_node) = self.get_nodes().last() {
            return !last_node.eq("*");
        }
        return false;
    }

    fn get_all_nodes(&self) -> Vec<String> {
        if self.folder_path.is_some() {
            let mut all_nodes = self.get_package_nodes_vec();
            for node in self.nodes.to_owned() {
                all_nodes.push(node);
            }

            return all_nodes;
        }

        return self.get_all_fake_nodes();
    }

    fn get_nodes(&self) -> Vec<String> {
        if self.folder_path.is_some() {
            return self.nodes.to_owned();
        }

        return self.get_all_fake_nodes();
    }

    pub(crate) fn is_wildcard_import(&self) -> bool {
        if self.folder_path.is_some() {
            return self.nodes.is_empty();
        }

        if let Some(last_node) = self.get_nodes().last() {
            return last_node.eq("*");
        }
        return false;
    }

    fn new_from_route(route: &str) -> JavaImport {
        if split_to_nodes(&route).is_empty() {
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

    /// The signature of this method prevents its validity
    /// It is not possible to fully recognize an import with type_id
    /// since the type could be a substructure JavaClass.JavaSubclass, JavaClass.method...
    /// The only valid way to validate this match is to check all self.nodes with the type_id(s)
    pub(crate) fn match_type_id(&self, type_id: &str) -> bool {
        let all_nodes = self.get_all_nodes().to_owned();
        if let Some(last_node) = all_nodes.last() {
            return &last_node == &type_id;
        }
        return false;
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
        path_helper::to_absolute_path_str(&file_path)
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
            path_helper::to_absolute_path_str(file_path)
        ));
    }

    if !file_path.is_file() {
        return Err(format!(
            "Can not create an explicit java import using non file input:\n\t\"{:?}\"\n",
            path_helper::to_absolute_path_str(file_path)
        ));
    }

    if !file_browser::do_last_element_in_path_ends_with(file_path, ".java") {
        return Err(format!(
            "Can not create an explicit java import using non java file:\n\t\"{:?}\"\n",
            path_helper::to_absolute_path_str(file_path)
        ));
    }
    Ok(())
}

fn check_dir_for_new_wildcard_import(dir_path: &Path) -> Result<(), String> {
    if !dir_path.is_dir() {
        return Err(format!(
            "Can not create a wildcard java import using non-dir input:\n\t\"{:?}\"\n",
            path_helper::to_absolute_path_str(dir_path)
        ));
    }
    if !dir_path.exists() {
        return Err(format!(
            "Can not create a wildcard java import using a folder that does not exist:\n\t\"{:?}\"\n",
            path_helper::to_absolute_path_str(dir_path)
        ));
    }
    Ok(())
}

fn remove_java_extension(java_file_name: String) -> String {
    let until = java_file_name.len() - 5;
    let node_copy = java_file_name.to_owned();
    node_copy.to_owned().drain(0..until).as_str().to_string()
}

pub fn get_package_nodes_vec_from_dir(dir_path: &Path) -> Vec<String> {
    if !dir_path.exists() || !dir_path.is_dir() {
        logger::log_unrecoverable_error(
            format!(
                "Java import related to an invalid folder:\n\t\"{}\"\n",
                path_helper::to_absolute_path_str(dir_path)
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
            path_helper::to_absolute_path_str(dir_path)
        )
        .as_str(),
    );
    Vec::new() // Fake return, it will stop in log_unrecoverable_error
}

// Relates to fake_non_checked_route
fn split_to_nodes(content: &str) -> Vec<String> {
    content.split(".").map(|str| str.to_string()).collect()
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

    return "".to_string();
}

impl fmt::Display for JavaImport {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("import ")?;

        for (idx, children) in self.get_all_nodes().iter().enumerate() {
            if idx != 0 {
                fmt.write_str(".")?;
            }
            fmt.write_str(&children)?;
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

    fn get_test_folder() -> PathBuf {
        get_java_project_test_folder(get_current_file_path(), "java_import")
    }

    #[test]
    fn new_wildcard_import_from_dir_test() {
        let dir_path = get_test_folder();

        match JavaImport::new_wildcard_import_from_dir(&dir_path) {
            Ok(import) => {
                assert!(import.is_wildcard_import());
                assert!(!import.is_explicit_import());
                assert_eq!("org.test.*", import.get_route());
                assert_eq!("import org.test.*;", import.to_string());
            }
            Err(err) => assert_fail(&err),
        };
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
