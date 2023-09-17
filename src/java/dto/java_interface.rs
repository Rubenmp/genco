use std::path::{Path, PathBuf};

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_field::JavaField;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_method::JavaMethod;
use crate::java::dto::java_visibility::JavaVisibility;
use crate::java::scanner::file::java_file::JavaFile;
use crate::java::scanner::file::java_structure::JavaStructure;
use crate::java::scanner::file::java_structure_type::JavaStructureType;

#[derive(Debug, Clone)]
pub struct JavaInterface {
    structure: JavaStructure,
}

impl JavaInterface {
    // Public methods

    /// # Builder pattern
    /// This method allows to create a new Java Interface
    /// and export it to a file. The "name" parameter is mandatory.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::dto::java_interface::JavaInterface;
    ///
    /// let dir = &env::current_dir().unwrap().join("doc").join("test");
    /// let java_interface = JavaInterface::builder().folder(dir).name("InterfaceName").build();
    /// ```
    pub fn builder() -> JavaInterfaceBuilder {
        JavaInterfaceBuilder::new_builder()
    }

    /// # from
    /// Creates a reference to a java interface from a given "file_path".
    /// If the provided file does not exist or it is not a valid java interface
    /// an error is returned.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::dto::java_interface::JavaInterface;
    ///
    /// let existing_file = env::current_dir().unwrap().join("AnyInterface.java");
    /// // let java_interface = JavaInterface::from(&existing_file);
    /// ```
    pub fn from(file_path: &Path) -> Result<Self, String> {
        let java_file = JavaFile::from_user_input_path(file_path)?;
        let structure_type = java_file.get_main_structure_type();
        if structure_type != JavaStructureType::Interface {
            return Err(format!(
                "Expected java interface, found java {:?}",
                structure_type
            ));
        }

        Ok(Self::from_structure(java_file.get_structure().to_owned()))
    }

    /// # get_annotations
    /// Get the java annotations of the JavaInterface
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        self.get_structure().get_annotations()
    }

    /// # get_visibility
    /// Get the java visibility of the JavaInterface
    pub fn get_visibility(&self) -> JavaVisibility {
        self.get_structure().get_visibility()
    }

    /// # get_name
    /// It returns the current JavaInterface name.
    pub fn get_name(&self) -> &str {
        self.get_structure().get_name()
    }

    /// # get_methods
    /// Get the methods of the current JavaInterface.
    pub fn get_methods(&self) -> &Vec<JavaMethod> {
        self.get_structure().get_methods()
    }

    /// # get_fields
    /// Get the fields of the current JavaInterface.
    pub fn get_fields(&self) -> &Vec<JavaField> {
        self.get_structure().get_fields()
    }
}

impl JavaInterface {
    // Crate or private methods
    pub(crate) fn from_import(import: &JavaImport) -> Result<Self, String> {
        let file_path = import.get_specific_file()?;
        let java_file = JavaFile::from_user_input_path(&file_path)?;
        Self::from_java_file(java_file)
    }

    pub(crate) fn from_structure(structure: JavaStructure) -> Self {
        Self { structure }
    }

    pub(crate) fn get_structure(&self) -> &JavaStructure {
        &self.structure
    }

    #[cfg(test)]
    pub(crate) fn get_file(&self) -> &PathBuf {
        self.get_structure().get_file()
    }

    #[cfg(test)]
    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        self.get_structure().get_imports()
    }

    pub(crate) fn get_self_import(&self) -> JavaImport {
        self.get_structure().get_self_import() // TODO: use java file instead of structure
    }

    fn from_java_file(java_file: JavaFile) -> Result<Self, String> {
        let structure_type = java_file.get_main_structure_type();
        if structure_type != JavaStructureType::Interface {
            return Err(format!(
                "Expected java interface, found java {:?} in file:\n{}\n",
                structure_type,
                try_to_absolute_path(&java_file.get_file_path())
            ));
        }

        Ok(Self::from_structure(java_file.get_structure().to_owned()))
    }
}

pub struct JavaInterfaceBuilder {
    folder: Option<PathBuf>,

    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    //is_static: bool,
    //is_abstract: bool,

    //extended_interfaces: Vec<JavaInterface>,
    name: Option<String>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
}

impl JavaInterfaceBuilder {
    fn new_builder() -> Self {
        Self {
            folder: None,
            annotations: vec![],
            visibility: JavaVisibility::Package,
            name: None,
            fields: vec![],
            methods: vec![],
        }
    }

    pub fn folder(&mut self, input: &Path) -> &mut Self {
        self.folder = Some(input.to_owned());
        self
    }

    pub fn annotations(&mut self, input: Vec<JavaAnnotationUsage>) -> &mut Self {
        self.annotations = input;
        self
    }

    pub fn visibility(&mut self, input: JavaVisibility) -> &mut Self {
        self.visibility = input;
        self
    }

    pub fn name(&mut self, input: &str) -> &mut Self {
        self.name = Some(input.to_string());
        self
    }

    pub fn fields(&mut self, input: Vec<JavaField>) -> &mut Self {
        self.fields = input;
        self
    }
    pub fn methods(&mut self, input: Vec<JavaMethod>) -> &mut Self {
        self.methods = input;
        self
    }

    pub fn build(&mut self) -> Result<JavaInterface, String> {
        let minimal_build_usage =
            "JavaInterface::builder()\n\t.folder(/* Mandatory folder */)\n\t.name(/* Interface name */)\n\t.build()";
        if self.name.is_none() {
            return Err(format!(
                "Invalid java interface build, name is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }

        let name = self.name.to_owned().expect("Class name is mandatory");
        if self.folder.is_none() {
            return Err(format!(
                "Invalid java interface build, folder is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }
        let folder = self.folder.as_ref().unwrap();
        if !folder.is_dir() {
            return Err(format!(
                "Invalid java interface \"{}\" build, expected dir:\n{}\n",
                name,
                try_to_absolute_path(folder)
            ));
        }

        return match JavaStructure::builder()
            .file(&folder.join(format!("{}.java", name)))
            .structure_type(JavaStructureType::Interface)
            .annotations(self.annotations.to_owned())
            .visibility(self.visibility)
            //.is_static(self.is_static)
            //.is_abstract(self.is_abstract)
            //.extended_interfaces(self.extended_interfaces.to_owned())
            .name(&name)
            .fields(self.fields.to_owned())
            .methods(self.methods.to_owned())
            .write()
        {
            Ok(structure) => Ok(JavaInterface::from_structure(structure)),
            Err(err) => Err(format!(
                "Invalid java interface \"{}\" build, {}",
                name, err
            )),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_creation::file_creator::remove_file_if_exists;
    use crate::core::testing::test_assert::{assert_fail, assert_same_file};
    use crate::core::testing::test_path;
    use crate::java::dto::java_interface::JavaInterface;
    use crate::java::dto::java_visibility::JavaVisibility::Package;

    #[test]
    fn builder() {
        let folder = get_test_folder();
        let file_path = folder.join("NewJavaInterface.java");
        let expected_file_content = get_test_file("ExpectedNewJavaInterface");
        let name = "NewJavaInterface";

        match JavaInterface::builder().folder(&folder).name(name).build() {
            Ok(interface) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(name, interface.get_name());
                assert_eq!(&file_path, interface.get_file());
                assert_eq!(0, interface.get_annotations().len());
                assert_eq!(Package, interface.get_visibility());
                assert_eq!(name, interface.get_name());
                assert_eq!(0, interface.get_fields().len());
                assert_eq!(0, interface.get_methods().len());
                assert_eq!(0, interface.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        };
    }

    #[test]
    fn new_from_path_interface() {
        let file_path = get_test_file("JavaInterfaceFrom");

        match JavaInterface::from(&file_path) {
            Ok(java_class) => {
                assert_eq!("JavaInterfaceFrom", java_class.get_name())
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_test_file(structure_name: &str) -> PathBuf {
        get_test_folder().join(format!("{}.java", structure_name).as_str())
    }

    fn get_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "java_interface")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
