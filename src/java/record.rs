use std::path::{Path, PathBuf};

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::java::annotation_usage::JavaAnnotationUsage;
use crate::java::field::JavaField;
use crate::java::import::JavaImport;
use crate::java::method::JavaMethod;
use crate::java::scanner::file::java_file::JavaFile;
use crate::java::scanner::file::java_structure::JavaStructure;
use crate::java::scanner::file::java_structure_type::JavaStructureType;
use crate::java::visibility::JavaVisibility;

/// # JavaRecord
/// A Java Record can be used to write it into a file
/// or as a reference for other methods.
#[derive(Debug)]
pub struct JavaRecord {
    scanned_file: JavaFile,
}

impl JavaRecord {
    // Public methods
    /// # Builder pattern
    /// This method allows to create a new Java record
    /// and export it to a file. The "name" parameter is mandatory.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::record::JavaRecord;
    ///
    /// let dir = &env::current_dir().unwrap().join("doc/test/java/record/src/main/java/org/test");
    /// let java_record = JavaRecord::builder().folder(dir).name("NewRecord").build();
    /// java_record.expect("Java record must be created");
    /// ```
    pub fn builder() -> JavaRecordBuilder {
        JavaRecordBuilder::new_builder()
    }

    /// # from
    /// Creates a reference to a java record from a given "file_path".
    /// If the provided file does not exist or it is not a valid java record
    /// an error is returned. The input java file is not modified.
    ///
    /// ```
    /// use std::env;
    /// use genco::java::record::JavaRecord;
    ///
    /// let existing_file = env::current_dir().unwrap().join("AnyRecord.java");
    /// // let record = JavaRecord::from(&existing_file);
    /// ```
    pub fn from(file_path: &Path) -> Result<Self, String> {
        let java_file = JavaFile::from_user_input_path(file_path)?;
        Self::from_java_file(java_file)
    }

    /// # copy_to
    /// Copy the current java record into the output_dir keeping the file name.
    /// It returns the new referenced record or an error if data copy did not succeed.
    ///
    /// If the output directory does not exist it will be created if it belongs to a java project.
    /// The output file only differs in the java package that will be adapted.
    /// ```
    /// use std::env;
    /// use genco::java::record::JavaRecord;
    ///
    /// let base_dir = &env::current_dir().unwrap().join("doc/test/java/record/src/main/java/org/test");
    /// let java_record = JavaRecord::builder().folder(&base_dir).name("JavaRecordToCopyToOutputFolder").build().expect("Valid record");
    ///
    /// let output_dir = base_dir.join("output_directory");
    /// let copied_java_record = java_record.copy_to(&output_dir).expect("Java record must be copied");
    /// // New created java record in <output_dir>/JavaRecordToCopyToOutputFolder.java
    /// ```
    pub fn copy_to(&self, output_dir: &Path) -> Result<Self, String> {
        let output_file = self.scanned_file.copy_to_output_folder(output_dir)?;

        Self::from_java_file(output_file)
    }

    /// # insert_method
    /// Insert a new method into the record and write it to the file.
    pub fn insert_method(&mut self, method: &JavaMethod) -> Result<(), String> {
        match self.scanned_file.insert_method(method) {
            Ok(result_java_file) => self.scanned_file = result_java_file,
            Err(err) => {
                return Err(err);
            }
        };

        Ok(())
    }

    /// # get_annotations
    /// Get the java annotations of the record
    pub fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        self.get_structure().get_annotations()
    }

    /// # get_visibility
    /// Get the java visibility of the record
    pub fn get_visibility(&self) -> JavaVisibility {
        self.get_structure().get_visibility()
    }

    /// # is_static
    /// It returns if the current JavaRecord is static.
    pub fn is_static(&self) -> bool {
        self.get_structure().is_static()
    }

    /// # get_name
    /// It returns the current JavaRecord name.
    pub fn get_name(&self) -> &str {
        self.get_structure().get_name()
    }

    /// # get_methods
    /// Get the methods of the current JavaRecord.
    pub fn get_methods(&self) -> &Vec<JavaMethod> {
        self.get_structure().get_methods()
    }

    /// # get_fields
    /// Get the fields of the current JavaRecord.
    pub fn get_fields(&self) -> &Vec<JavaField> {
        self.get_structure().get_fields()
    }
}

impl JavaRecord {
    // Crate or private methods
    fn write(file: &Path, structure: JavaStructure) -> Result<Self, String> {
        let scanned_file = JavaFile::write(file, structure)?;

        Ok(Self { scanned_file })
    }

    pub(crate) fn from_import(import: &JavaImport) -> Result<Self, String> {
        match import.get_specific_file() {
            Ok(file) => Self::from(&file),
            Err(err) => Err(err),
        }
    }

    fn from_java_file(java_file: JavaFile) -> Result<Self, String> {
        let structure_type = java_file.get_main_structure_type();
        if structure_type != JavaStructureType::Record {
            return Err(format!(
                "Expected java record, found java {:?} in file:\n{}\n",
                structure_type,
                try_to_absolute_path(java_file.get_file_path())
            ));
        }

        let java_record = Self {
            scanned_file: java_file,
        };
        Ok(java_record)
    }

    pub(crate) fn get_structure(&self) -> &JavaStructure {
        self.get_scanned_file().get_structure()
    }

    fn get_scanned_file(&self) -> &JavaFile {
        &self.scanned_file
    }

    #[cfg(test)]
    pub(crate) fn get_file(&self) -> &PathBuf {
        self.get_scanned_file().get_file_path()
    }

    #[cfg(test)]
    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        self.get_structure().get_imports()
    }

    pub(crate) fn get_self_import(&self) -> JavaImport {
        self.get_scanned_file().get_self_import()
    }
}

pub struct JavaRecordBuilder {
    folder: Option<PathBuf>,

    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,

    name: Option<String>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
}

impl JavaRecordBuilder {
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
        self.folder = Some(input.to_path_buf());
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

    pub fn build(&mut self) -> Result<JavaRecord, String> {
        let minimal_build_usage =
            "JavaRecord::builder()\n\t.folder(/* Mandatory folder */)\n\t.name(/* Record name */)\n\t.build()";
        if self.name.is_none() {
            return Err(format!(
                "Invalid java record build, name is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }
        let name = self.name.clone().expect("Java record name is mandatory");
        if self.folder.is_none() {
            return Err(format!(
                "Invalid java record build, folder is mandatory. Example:\n{}\n",
                minimal_build_usage
            ));
        }
        let folder = self.folder.as_ref().expect("Folder must exist");
        if !folder.is_dir() {
            return Err(format!(
                "Invalid java record \"{}\" build, expected dir:\n{}\n",
                name,
                try_to_absolute_path(folder)
            ));
        }

        let file = folder.join(format!("{}.java", name));
        return match JavaStructure::builder()
            .structure_type(JavaStructureType::Record)
            .annotations(self.annotations.clone())
            .visibility(self.visibility)
            .name(&name)
            .fields(self.fields.clone())
            .methods(self.methods.clone())
            .build()
        {
            Ok(structure) => Ok(JavaRecord::write(&file, structure)?),
            Err(err) => Err(format!("Invalid java record \"{}\" build, {}", name, err)),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::core::testing::test_assert::{assert_fail, assert_same_file};
    use crate::core::testing::test_path;
    use crate::java::dependency::org::springframework::spring_context::java_spring_context_factory;
    use crate::java::record::JavaRecord;
    use crate::java::visibility::JavaVisibility;

    #[test]
    fn build_record_test() {
        let folder = get_java_record_root_test_folder();
        let file_path = folder.join("JavaRecordBuild.java");
        let expected_file_content = get_expected_file("JavaRecordBuild");

        let annotations = vec![java_spring_context_factory::_create_service_annotation_usage()];
        match JavaRecord::builder()
            .folder(&folder)
            .visibility(JavaVisibility::Public)
            .name("JavaRecordBuild")
            .annotations(annotations)
            .build()
        {
            Ok(java_record) => {
                assert_same_file(&expected_file_content, &file_path);
                let _ = fs::remove_file(&file_path).expect("Result file must be removed");
                assert_eq!(&file_path, java_record.get_file());
                assert_eq!(1, java_record.get_annotations().len());
                assert_eq!(JavaVisibility::Public, java_record.get_visibility());
                assert!(!java_record.is_static());
                assert_eq!("JavaRecordBuild", java_record.get_name());
                assert_eq!(0, java_record.get_fields().len());
                assert_eq!(0, java_record.get_methods().len());
                assert_eq!(1, java_record.get_imports().len());
            }
            Err(err) => {
                // let _ = fs::remove_file(&file_path).expect("Result file must be removed");
                assert_fail(&err);
            }
        }
    }

    #[test]
    fn new_record_from_path() {
        let file_path = get_test_file("JavaRecord");

        match JavaRecord::from(&file_path) {
            Ok(java_record) => {
                assert_eq!("JavaRecord", java_record.get_name());
            }
            Err(err) => assert_fail(&err),
        }
    }

    fn get_expected_file(structure_name: &str) -> PathBuf {
        get_java_record_root_test_folder()
            .join("expected")
            .join(format!("{}.java", structure_name).as_str())
    }

    fn get_test_file(structure_name: &str) -> PathBuf {
        get_java_record_root_test_folder().join(format!("{}.java", structure_name).as_str())
    }

    fn get_java_record_root_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "record")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
