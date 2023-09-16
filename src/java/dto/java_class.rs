#[allow(unused)]
use std::path::{Path, PathBuf};

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_field::JavaField;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_interface::JavaInterface;
use crate::java::dto::java_method::JavaMethod;
use crate::java::dto::java_visibility::JavaVisibility;
use crate::java::scanner::file::java_file::JavaFile;
use crate::java::scanner::file::java_structure::JavaStructure;
use crate::java::scanner::file::java_structure_type::JavaStructureType;

/// # JavaClass
/// A Java Class can be used to write it into a file
/// or as a reference for other methods.
#[derive(Debug, Clone)]
pub struct JavaClass {
    scanned_structure: JavaStructure,
}

impl JavaClass {
    // Public methods
    /// # Builder pattern
    /// This method allows to create a new Java Class
    /// and export it to a file. The "name" parameter is mandatory.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::dto::java_class::JavaClass;
    ///
    /// let dir = &env::current_dir().unwrap().join("doctest");
    /// let java_class = JavaClass::builder().folder(dir).name("Service").build();
    /// ```
    pub fn builder() -> JavaClassBuilder {
        JavaClassBuilder::new_builder()
    }

    /// # from
    /// Creates a reference to a java class from a given "file_path".
    /// If the provided file does not exist or it is not a valid java class
    /// an error is returned.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::dto::java_class::JavaClass;
    ///
    /// let existing_file = env::current_dir().unwrap().join("AnyClass.java");
    /// // let java_class = JavaClass::from(&existing_file);
    /// ```
    pub fn from(file_path: &Path) -> Result<Self, String> {
        let java_file = JavaFile::from_user_input_path(file_path)?;
        Self::from_java_file(java_file)
    }

    /// # get_annotations
    /// Get the java annotations of the class
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        self.get_structure().get_annotations()
    }

    /// # get_visibility
    /// Get the java visibility of the class
    pub fn get_visibility(&self) -> JavaVisibility {
        self.get_structure().get_visibility()
    }

    /// # is_static
    /// It returns if the current JavaClass is static.
    pub fn is_static(&self) -> bool {
        self.get_structure().is_static()
    }

    /// # is_abstract
    /// It returns if the current JavaClass is abstract.
    pub fn is_abstract(&self) -> bool {
        self.get_structure().is_abstract()
    }

    /// # is_final
    /// It returns if the current JavaClass is final.
    pub fn is_final(&self) -> bool {
        self.get_structure().is_final()
    }

    /// # get_name
    /// It returns the current JavaClass name.
    pub fn get_name(&self) -> &str {
        self.get_structure().get_name()
    }

    /// # get_extended_class
    /// Get the class from what the current JavaClass extends.
    /// Under the hood, JavaClass stores the reference to its extended class.
    /// Therefore, this method will scan (if exists) the extended class file,
    /// and return the result.
    pub fn get_extended_class(&self) -> Option<JavaClass> {
        self.get_structure().get_extended_class().to_owned()
    }

    /// # get_implemented_interfaces
    /// Get the interfaces that current JavaClass implement.
    /// Under the hood, JavaClass stores the reference to its implemented interfaces.
    /// Therefore, this method will scan (if exists) the implemented interface file(s),
    /// and return the result.
    pub fn get_implemented_interfaces(&self) -> Vec<JavaInterface> {
        self.get_structure().get_implemented_interfaces()
    }

    /// # get_methods
    /// Get the methods of the current JavaClass.
    pub fn get_methods(&self) -> &Vec<JavaMethod> {
        self.get_structure().get_methods()
    }

    /// # get_fields
    /// Get the fields of the current JavaClass.
    pub fn get_fields(&self) -> &Vec<JavaField> {
        self.get_structure().get_fields()
    }

    /// # insert_method
    /// Insert a new method into the class and write it to the file.
    pub fn insert_method(&mut self, method: &JavaMethod) -> Result<(), String> {
        self.get_structure_mut().insert_method(method)
    }
}

impl JavaClass {
    // Crate or private methods

    pub(crate) fn from_structure(structure: JavaStructure) -> Self {
        Self {
            scanned_structure: structure,
        }
    }

    pub(crate) fn from_import(import: &JavaImport) -> Result<Self, String> {
        match import.get_specific_file() {
            Ok(file) => Self::from(&file),
            Err(err) => Err(err),
        }
    }

    fn from_java_file(java_file: JavaFile) -> Result<Self, String> {
        let structure_type = java_file.get_main_structure_type();
        if structure_type != JavaStructureType::Class {
            return Err(format!(
                "Expected java class, found java {:?} in file:\n{}\n",
                structure_type,
                try_to_absolute_path(&java_file.get_file_path())
            ));
        }

        Ok(Self::from_structure(java_file.get_structure().to_owned()))
    }

    pub(crate) fn get_structure(&self) -> &JavaStructure {
        &self.scanned_structure
    }

    pub(crate) fn get_structure_mut(&mut self) -> &mut JavaStructure {
        &mut self.scanned_structure
    }

    #[cfg(test)]
    pub(crate) fn get_file(&self) -> &PathBuf {
        &self.get_structure().get_file()
    }

    #[cfg(test)]
    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        self.get_structure().get_imports()
    }

    pub(crate) fn get_self_import(&self) -> JavaImport {
        self.get_structure().get_self_import()
    }
}

pub struct JavaClassBuilder {
    folder: Option<PathBuf>,

    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_abstract: bool,

    extended_class: Vec<JavaClass>,
    implemented_interfaces: Vec<JavaInterface>,

    name: Option<String>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
}

impl JavaClassBuilder {
    fn new_builder() -> Self {
        Self {
            folder: None,
            annotations: vec![],
            visibility: JavaVisibility::Package,
            is_static: false,
            is_abstract: false,
            extended_class: Vec::new(),
            implemented_interfaces: vec![],
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
    pub fn is_static(&mut self, input: bool) -> &mut Self {
        self.is_static = input;
        self
    }

    pub fn is_abstract(&mut self, input: bool) -> &mut Self {
        self.is_abstract = input;
        self
    }
    pub fn extended_class(&mut self, input: JavaClass) -> &mut Self {
        self.extended_class = vec![input];
        self
    }

    pub fn implemented_interfaces(&mut self, input: Vec<JavaInterface>) -> &mut Self {
        self.implemented_interfaces = input;
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

    pub fn build(&mut self) -> Result<JavaClass, String> {
        let minimal_build_usage =
            "JavaClass::builder()\n\t.folder(/* Mandatory folder */)\n\t.name(/* Class name */)\n\t.build()";
        if self.name.is_none() {
            return Err(format!(
                "Invalid java class build, name is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }
        let name = self.name.to_owned().expect("Java class name is mandatory");
        if self.folder.is_none() {
            return Err(format!(
                "Invalid java class build, folder is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }
        let folder = self.folder.as_ref().unwrap();
        if !folder.is_dir() {
            return Err(format!(
                "Invalid java class \"{}\" build, expected dir:\n{}\n",
                name,
                try_to_absolute_path(folder)
            ));
        }

        return match JavaStructure::builder()
            .file(&folder.join(format!("{}.java", name)))
            .structure_type(JavaStructureType::Class)
            .annotations(self.annotations.to_owned())
            .visibility(self.visibility)
            .is_static(self.is_static)
            .is_abstract(self.is_abstract)
            .extended_classes(self.extended_class.to_owned())
            .implemented_interfaces(self.implemented_interfaces.to_owned())
            .name(&name)
            .fields(self.fields.to_owned())
            .methods(self.methods.to_owned())
            .write()
        {
            Ok(structure) => Ok(JavaClass::from_structure(structure)),
            Err(err) => Err(format!("Invalid java class \"{}\" build, {}", name, err)),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_creation::file_creator::remove_file_if_exists;
    use crate::core::testing::test_assert::{assert_fail, assert_same_file};
    use crate::core::testing::test_path;
    use crate::java::dependency::org::springframework::spring_context::java_spring_context_factory;
    use crate::java::dto::java_class::JavaClass;
    use crate::java::dto::java_data_type::{JavaBasicDataType, JavaDataType};
    use crate::java::dto::java_field::JavaField;
    use crate::java::dto::java_interface::JavaInterface;
    use crate::java::dto::java_method::JavaMethod;
    use crate::java::dto::java_visibility::JavaVisibility;

    #[test]
    fn build_class_empty_service() {
        let folder = get_test_folder();
        let file_path = folder.join("EmptyService.java");
        let expected_file_content = get_test_file("ExpectedEmptyService");

        let annotations = vec![java_spring_context_factory::_create_service_annotation_usage()];
        match JavaClass::builder()
            .folder(&folder)
            .visibility(JavaVisibility::Public)
            .name("EmptyService")
            .annotations(annotations)
            .build()
        {
            Ok(java_class) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, java_class.get_file());
                assert_eq!(1, java_class.get_annotations().len());
                assert_eq!(JavaVisibility::Public, java_class.get_visibility());
                assert!(!java_class.is_static());
                assert!(!java_class.is_final());
                assert!(!java_class.is_abstract());
                assert_eq!("EmptyService", java_class.get_name());
                assert!(java_class.get_extended_class().is_none());
                assert_eq!(0, java_class.get_implemented_interfaces().len());
                assert_eq!(0, java_class.get_fields().len());
                assert_eq!(0, java_class.get_methods().len());
                assert_eq!(1, java_class.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn build_full_java_service_from_builder() {
        let folder = get_test_folder();
        let file_path = folder.join("FullJavaServiceFromBuilder.java");
        let expected_file_content = get_test_file("ExpectedFullJavaServiceFromBuilder");

        let annotations = vec![java_spring_context_factory::_create_service_annotation_usage()];
        let method = get_new_method();
        let field = get_private_field();
        let extended_class =
            JavaClass::from(&folder.join("JavaClassFrom.java")).expect("Extended class");
        let java_interface = JavaInterface::from(&folder.join("JavaInterfaceForClass.java"))
            .expect("Implemented interface");
        match JavaClass::builder()
            .folder(&folder)
            .annotations(annotations)
            .visibility(JavaVisibility::Public)
            .name("FullJavaServiceFromBuilder")
            .fields(vec![field])
            .methods(vec![method])
            .extended_class(extended_class)
            .implemented_interfaces(vec![java_interface])
            .build()
        {
            Ok(java_class) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, java_class.get_file());
                assert_eq!(1, java_class.get_annotations().len());
                assert_eq!(JavaVisibility::Public, java_class.get_visibility());
                assert!(!java_class.is_static());
                assert!(!java_class.is_final());
                assert!(!java_class.is_abstract());
                assert_eq!("FullJavaServiceFromBuilder", java_class.get_name());
                assert!(java_class.get_extended_class().is_some());
                assert_eq!(1, java_class.get_implemented_interfaces().len());
                assert_eq!(1, java_class.get_fields().len());
                assert_eq!(1, java_class.get_methods().len());
                assert_eq!(3, java_class.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_new_method() -> JavaMethod {
        JavaMethod::builder()
            .return_type(JavaDataType::Basic(JavaBasicDataType::Int))
            .name("newMethod")
            .build()
            .expect("newMethod is expected to be valid")
    }

    fn get_private_field() -> JavaField {
        JavaField::builder()
            .visibility(JavaVisibility::Private)
            .data_type(JavaDataType::Basic(JavaBasicDataType::Boolean))
            .name("field")
            .build()
            .expect("field is expected to be valid")
    }

    #[test]
    fn new_from_path_class() {
        let file_path = get_test_file("ExpectedFullJavaService");

        match JavaClass::from(&file_path) {
            Ok(java_class) => {
                assert_eq!("FullJavaService", java_class.get_name());
                if let Some(extended_class) = java_class.get_extended_class() {
                    assert_eq!("JavaClassFrom", extended_class.get_name());
                } else {
                    assert_fail("Extended class expected");
                }

                let interfaces = java_class.get_implemented_interfaces();
                assert_eq!(1, interfaces.len());
                if let Some(interface) = interfaces.get(0) {
                    assert_eq!("JavaInterfaceForClass", interface.get_name());
                } else {
                    assert_fail("Extended interface expected");
                }
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn class_with_static_method() {
        let file_path = get_test_file("ClassWithStaticMethod");

        match JavaClass::from(&file_path) {
            Ok(java_class) => {
                assert_eq!("ClassWithStaticMethod", java_class.get_name());
                assert_eq!(1, java_class.get_methods().len());
                if let Some(method) = java_class.get_methods().get(0) {
                    assert_eq!(JavaVisibility::Public, method.get_visibility());
                    assert!(method.is_static());
                    assert_eq!("staticMethod", method.get_name());
                }
            }
            Err(error_msg) => assert_fail(&error_msg),
        }
    }

    #[test]
    fn insert_method_in_new_class() {
        let folder = get_test_folder();
        let file_path = folder.join("EmptyClassWithNewInsertedMethod.java");
        let expected_file_content = get_test_file("ExpectedEmptyClassWithNewInsertedMethod");

        let new_method = new_method();

        let mut java_class = JavaClass::builder()
            .folder(&folder)
            .visibility(JavaVisibility::Public)
            .name("EmptyClassWithNewInsertedMethod")
            .build()
            .expect("Empty java class must be created");
        match java_class.insert_method(&new_method) {
            Ok(_) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn new_method() -> JavaMethod {
        JavaMethod::builder()
            .return_type(JavaDataType::Basic(JavaBasicDataType::Int))
            .visibility(JavaVisibility::Public)
            .name("newMethod")
            .build()
            .expect("newMethod is expected to be valid")
    }

    fn get_test_file(structure_name: &str) -> PathBuf {
        get_test_folder().join(format!("{}.java", structure_name).as_str())
    }

    fn get_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "java_class")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
