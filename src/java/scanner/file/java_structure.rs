use std::path::{Path, PathBuf};

use crate::core::file_system::file_creator::file_creator;
use crate::core::file_system::file_overwriting::file_overwriting::FileOverwriting;
use crate::core::file_system::path_helper::to_absolute_path_str;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_class::JavaClass;
use crate::java::dto::java_data_type::JavaDataType;
use crate::java::dto::java_field::JavaField;
use crate::java::dto::java_import::JavaImport;
use crate::java::dto::java_indentation_config::JavaIndentation;
use crate::java::dto::java_interface::JavaInterface;
use crate::java::dto::java_method::JavaMethod;
use crate::java::dto::java_visibility::JavaVisibility;
use crate::java::dto::{java_annotation_usage, java_visibility};
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_imports_scan::JavaImportsScan;
use crate::java::scanner::file::java_structure_type::JavaStructureType;
use crate::java::scanner::package::java_package_scanner;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct JavaStructure {
    // Metadata
    file: PathBuf,
    structure_type: JavaStructureType,
    struct_body_start_byte: usize,
    struct_body_end_byte: usize,

    // Modifiers
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_final: bool,
    is_abstract: bool,

    // Class specific
    extended_class: Vec<JavaImport>,
    implemented_interfaces: Vec<JavaImport>,

    // Rest of the fields
    name: String,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
    is_root_structure: bool,
    substructures: Vec<JavaStructure>,
}

impl JavaStructure {
    pub(crate) fn builder() -> JavaStructureBuilder {
        JavaStructureBuilder::new_builder()
    }

    pub(crate) fn new(
        root_struct_decl_node: &JavaNode,
        file_imports: &JavaImportsScan,
        input_java_file: &Path,
    ) -> Result<Self, String> {
        new_structure_internal(root_struct_decl_node, file_imports, input_java_file, true)
    }

    pub(crate) fn get_file(&self) -> &PathBuf {
        &self.file
    }

    pub(crate) fn get_dir(&self) -> PathBuf {
        let mut path = self.file.to_path_buf();
        path.pop();
        path.to_owned()
    }

    pub(crate) fn get_type(&self) -> JavaStructureType {
        self.structure_type.to_owned()
    }

    pub(crate) fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        &self.annotations
    }

    pub(crate) fn get_visibility(&self) -> JavaVisibility {
        self.visibility.to_owned()
    }
    pub(crate) fn is_static(&self) -> bool {
        self.is_static
    }

    pub(crate) fn is_final(&self) -> bool {
        self.is_final
    }
    pub(crate) fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_extended_class(&self) -> Option<JavaClass> {
        if let Some(extension_class) = self.extended_class.first() {
            return match JavaClass::from_import(extension_class) {
                Ok(java_class) => Some(java_class),
                Err(err) => {
                    self.log_warning_scanning_super_class(extension_class, &err);
                    return None;
                }
            };
        }
        None
    }

    fn log_warning_scanning_super_class(&self, extension_class: &JavaImport, err: &str) {
        logger::log_warning(&format!(
            "Unexpected error scanning \"{}\" (super class of \"{}\"):\n{}",
            extension_class,
            self.get_name(),
            err
        ));
    }

    pub(crate) fn get_implemented_interfaces(&self) -> Vec<JavaInterface> {
        let mut result = Vec::new();
        for interface_import in &self.implemented_interfaces {
            match JavaInterface::from_import(interface_import) {
                Ok(java_interface) => result.push(java_interface),
                Err(err) => self.log_warning_scanning_interface(interface_import, &err),
            };
        }

        result
    }

    fn log_warning_scanning_interface(&self, interface_import: &JavaImport, err: &str) {
        logger::log_warning(&format!(
            "Unexpected error scanning \"{}\" (implemented interface of \"{}\"):\n{}",
            interface_import,
            self.get_name(),
            err
        ));
    }

    pub(crate) fn get_fields(&self) -> &Vec<JavaField> {
        &self.fields
    }
    pub(crate) fn get_methods(&self) -> &Vec<JavaMethod> {
        &self.methods
    }
    pub(crate) fn is_root_structure(&self) -> bool {
        self.is_root_structure
    }

    pub(crate) fn insert_method(&mut self, _method: &JavaMethod) -> Result<(), String> {
        todo!()
    }

    /// TODO: this is only to for parent structures, not substructures
    pub(crate) fn get_self_import(&self) -> JavaImport {
        if self.is_root_structure() {
            return JavaImport::new_explicit_import_from_file(self.get_file())
                .expect("Import for java structure must exist");
        } else {
            panic!("get_self_import of substructure is not supported");
        }
    }
    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        let mut imports = Vec::new();
        for import in self.get_annotation_imports() {
            imports.push(import);
        }

        if let Some(extended_class) = self.get_extended_class() {
            imports.push(extended_class.get_self_import());
        }

        for import in self.get_implemented_interfaces_imports() {
            imports.push(import);
        }

        for import in self.get_field_imports() {
            imports.push(import);
        }

        for import in self.get_method_imports() {
            imports.push(import);
        }

        imports
    }

    fn get_implemented_interfaces_imports(&self) -> Vec<JavaImport> {
        self.get_implemented_interfaces()
            .iter()
            .map(|interface| interface.get_self_import())
            .collect()
    }

    fn get_field_imports(&self) -> Vec<JavaImport> {
        self.get_fields()
            .iter()
            .flat_map(|field| field.get_imports())
            .collect()
    }

    fn get_method_imports(&self) -> Vec<JavaImport> {
        self.get_methods()
            .iter()
            .flat_map(|field| field.get_imports())
            .collect()
    }

    fn get_annotation_imports(&self) -> Vec<JavaImport> {
        self.get_annotations()
            .iter()
            .flat_map(|annotation| annotation.get_imports())
            .collect()
    }

    /// # write_to_file
    /// Export java structure into a specific directory "export_directory"
    /// that must be inside a java project, creating a java file with
    /// the name of the structure.
    pub(crate) fn write_to_file(&self) -> Result<(), String> {
        self.validate_output_file()?;
        let mut result = self.write_package();
        self.write_imports(&mut result);

        result += self.get_skeleton_without_imports().as_str();
        self.write_body(&mut result);
        self.write_to_file_internal(&mut result);
        Ok(())
    }

    fn write_imports(&self, result: &mut String) {
        let imports: Vec<JavaImport> = self.get_imports();

        for import in &imports {
            *result += import.to_string().as_str();
            *result += "\n";
        }
        if !imports.is_empty() {
            *result += "\n";
        }
    }
    fn get_skeleton_without_imports(&self) -> String {
        let mut result = "".to_string();
        self.write_annotations(&mut result);
        self.write_visibility(&mut result);
        result += self.get_name();
        self.write_extensions_and_implementations(&mut result);
        result
    }

    fn write_body(&self, result: &mut String) {
        *result += " {\n";
        let mut java_indentation = JavaIndentation::default();
        java_indentation.increase_level();

        for (index, field) in self.get_fields().iter().enumerate() {
            if index > 0 {
                *result += "\n";
            }
            *result += field.get_str(&java_indentation).as_str();
        }

        if !self.get_methods().is_empty() {
            *result += "\n";
        }
        for (index, method) in self.get_methods().iter().enumerate() {
            if index > 0 {
                *result += "\n";
            }
            method.write_to_string(result, &java_indentation);
        }

        java_indentation.decrease_level();
        *result += format!("{}}}\n", java_indentation.get_current_indentation()).as_str();
    }

    fn write_to_file_internal(&self, result: &mut str) {
        let file_path = self.get_file();
        if file_path.exists() && file_path.is_file() {
            file_creator::remove_file_if_exists(file_path);
        }

        file_creator::create_file_if_not_exist(file_path);
        let mut overwriting = FileOverwriting::new(file_path);
        overwriting.append(result);
        overwriting.write_all();
    }

    fn write_package(&self) -> String {
        let mut result = "package ".to_string();
        result += java_package_scanner::get_package_from_dir(&self.get_dir()).as_str();
        result += ";\n\n";
        result
    }

    fn write_visibility(&self, result: &mut String) {
        *result += self.get_visibility().to_file_string().as_str();
        if self.is_static() {
            *result += "static ";
        }
        if self.is_final() {
            *result += "final ";
        }
        if self.is_abstract() {
            *result += "abstract ";
        }
        match self.get_type() {
            JavaStructureType::Class => *result += "class ",
            JavaStructureType::Interface => *result += "interface ",
            JavaStructureType::Enum => *result += "enum ",
        }
    }

    fn write_extensions_and_implementations(&self, result: &mut String) {
        if let Some(extension) = self.get_extended_class() {
            *result += " extends ";
            *result += extension.get_name();
        }

        let interfaces = self.get_implemented_interfaces();
        if !interfaces.is_empty() {
            *result += " implements ";
            for (index, interface) in interfaces.iter().enumerate() {
                if index > 0 {
                    *result += ", ";
                }
                *result += interface.get_name();
            }
        }
    }

    fn write_annotations(&self, result: &mut String) {
        let indentation = JavaIndentation::default();
        for annotation in self.get_annotations() {
            *result += annotation.to_file_string(&indentation).as_str();
        }
    }
    fn validate_output_file(&self) -> Result<(), String> {
        let file = self.get_file();
        if file.exists() && file.is_dir() {
            return Err(format!(
                "expecting an output file but a dir was found:\n{}\n",
                to_absolute_path_str(file)
            ));
        }

        Ok(())
    }
}

fn new_structure_internal(
    root_node: &JavaNode,
    file_imports: &JavaImportsScan,
    input_java_file: &Path,
    is_root_structure: bool,
) -> Result<JavaStructure, String> {
    let structure_type_opt: Option<JavaStructureType> =
        get_java_structure_type(root_node.get_node_type_opt());
    let mut visibility = JavaVisibility::Package;
    let mut is_static = false;
    let mut is_final = false;
    let mut is_abstract = false;
    let mut name_opt = None;
    let mut annotations = Vec::new();
    let mut extended_class = Vec::new();
    let mut implemented_interfaces = Vec::new();
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    let mut substructures = Vec::new();
    let mut struct_body_start_byte_opt: Option<usize> = None;
    let mut struct_body_end_byte_opt: Option<usize> = None;

    //root_node.print_tree_and_panic();
    for child_node in root_node.get_children() {
        if let Some(structure_node_type) = child_node.get_node_type_opt() {
            if JavaNodeType::Modifiers == structure_node_type {
                for modifier in child_node.get_children() {
                    if let Some(node_type) = modifier.get_node_type_opt() {
                        if java_annotation_usage::is_java_node_annotation(&node_type) {
                            match JavaAnnotationUsage::new_from_java_node(
                                modifier,
                                file_imports,
                                input_java_file,
                            ) {
                                Ok(annotation) => annotations.push(annotation),
                                Err(err) => logger::log_warning(&err),
                            };
                        } else if java_visibility::is_visibility_node_type(&node_type) {
                            visibility = java_visibility::new(&node_type);
                        } else if JavaNodeType::Static == node_type {
                            is_static = true;
                        } else if JavaNodeType::Abstract == node_type {
                            is_abstract = true;
                        } else if JavaNodeType::Final == node_type {
                            is_final = true;
                        }
                    }
                }
            } else if JavaNodeType::Id == structure_node_type {
                name_opt = Some(child_node.get_content());
            } else if JavaNodeType::Superclass == structure_node_type {
                if let Some(import) = extract_super_class(file_imports, input_java_file, child_node)
                {
                    extended_class.push(import);
                }
            } else if JavaNodeType::SuperInterfaces == structure_node_type {
                for import in extract_interfaces(child_node, file_imports, input_java_file) {
                    implemented_interfaces.push(import);
                }
            } else if is_structure_body(&structure_node_type) {
                for body_child in child_node.get_children() {
                    if let Some(body_node_type) = body_child.get_node_type_opt() {
                        if JavaNodeType::FieldDeclaration == body_node_type {
                            match JavaField::new(body_child, file_imports, input_java_file) {
                                Ok(field) => fields.push(field),
                                Err(err) => logger::log_warning(&err),
                            };
                        } else if JavaNodeType::MethodDecl == body_node_type {
                            match JavaMethod::new_from_node(
                                body_child,
                                file_imports,
                                input_java_file,
                            ) {
                                Ok(method) => methods.push(method),
                                Err(err) => log_invalid_method_decl(input_java_file, err),
                            }
                        } else if JavaNodeType::LBrace == body_node_type {
                            // This does not take into account comments in that line
                            struct_body_start_byte_opt = Some(body_child.get_end_byte());
                        } else if JavaNodeType::RBrace == body_node_type {
                            struct_body_end_byte_opt = Some(body_child.get_start_byte());
                        }
                    }
                }
            } else if is_java_structure_type(Some(structure_node_type)) {
                match new_structure_internal(child_node, file_imports, input_java_file, false) {
                    Ok(new_substructure) => substructures.push(new_substructure),
                    Err(err) => logger::log_warning(&err),
                }
            }
        }
    }

    let structure_type = structure_type_opt.ok_or("Invalid structure type")?;
    let name = name_opt.ok_or("Invalid structure name")?;
    let struct_body_start_byte =
        struct_body_start_byte_opt.ok_or("Body structure start not found")?;
    let struct_body_end_byte = struct_body_end_byte_opt.ok_or("Body structure end not found")?;
    Ok(JavaStructure {
        file: input_java_file.to_owned(),
        structure_type,
        struct_body_start_byte,
        struct_body_end_byte,
        annotations,
        visibility,
        is_static,
        is_final,
        is_abstract,
        extended_class,
        implemented_interfaces,
        name,
        fields,
        methods,
        is_root_structure,
        substructures,
    })
}

fn extract_super_class(
    file_imports: &JavaImportsScan,
    input_java_file: &Path,
    child_node: &JavaNode,
) -> Option<JavaImport> {
    let children = child_node.get_children();
    if !is_second_child_an_extended_class_id(children) {
        log_unrecognized_super_class(child_node, input_java_file);
        return None;
    }

    let second_child_content = children.get(1).unwrap().get_content();
    let data_type =
        JavaDataType::from_import_type_id(&second_child_content, file_imports, input_java_file);
    if let Ok(JavaDataType::FromImport(import)) = data_type {
        return Some(import);
    }

    log_unrecognized_super_class_type_id_import(child_node, input_java_file);
    None
}

fn log_invalid_method_decl(input_java_file: &Path, err: String) {
    logger::log_warning(
        format!(
            "Invalid method ({}) in file:\n{}\n",
            err,
            to_absolute_path_str(input_java_file)
        )
        .as_str(),
    )
}

fn is_structure_body(structure_node_type: &JavaNodeType) -> bool {
    &JavaNodeType::ClassBody == structure_node_type
        || &JavaNodeType::InterfaceBody == structure_node_type
        || &JavaNodeType::EnumBody == structure_node_type
}

fn is_java_structure_type(node_type_opt: Option<JavaNodeType>) -> bool {
    get_java_structure_type(node_type_opt).is_some()
}

fn get_java_structure_type(node_type_opt: Option<JavaNodeType>) -> Option<JavaStructureType> {
    if let Some(node_type) = node_type_opt {
        if JavaNodeType::ClassDecl == node_type {
            return Some(JavaStructureType::Class);
        } else if JavaNodeType::InterfaceDeclaration == node_type {
            return Some(JavaStructureType::Interface);
        } else if JavaNodeType::EnumDeclaration == node_type {
            return Some(JavaStructureType::Enum);
        }
    }

    None
}

pub struct JavaStructureBuilder {
    file: Option<PathBuf>,
    structure_type: Option<JavaStructureType>,
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_final: bool,
    is_abstract: bool,
    extended_class: Vec<JavaClass>,
    implemented_interfaces: Vec<JavaInterface>,
    name: Option<String>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
    substructures: Vec<JavaStructure>,
}

impl JavaStructureBuilder {
    fn new_builder() -> Self {
        Self {
            file: None,
            structure_type: None,
            annotations: vec![],
            visibility: JavaVisibility::Public,
            is_static: false,
            is_final: false,
            is_abstract: false,
            extended_class: Vec::new(),
            implemented_interfaces: vec![],
            name: None,
            fields: vec![],
            methods: vec![],
            substructures: vec![],
        }
    }
    pub fn file(&mut self, input: &Path) -> &mut Self {
        self.file = Some(input.to_owned());
        self
    }
    pub fn structure_type(&mut self, input: JavaStructureType) -> &mut Self {
        self.structure_type = Some(input.to_owned());
        self
    }
    pub fn annotations(&mut self, input: Vec<JavaAnnotationUsage>) -> &mut Self {
        self.annotations = input.to_owned();
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

    pub fn is_final(&mut self, input: bool) -> &mut Self {
        self.is_final = input;
        self
    }

    pub fn is_abstract(&mut self, input: bool) -> &mut Self {
        self.is_abstract = input;
        self
    }
    pub fn extended_classes(&mut self, input: Vec<JavaClass>) -> &mut Self {
        self.extended_class = input;
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
    pub fn substructures(&mut self, input: Vec<JavaStructure>) -> &mut Self {
        self.substructures = input;
        self
    }
    pub fn write(&mut self) -> Result<JavaStructure, String> {
        let name = self.get_name()?;

        // There is a better way to avoid this mapping to JavaImport
        // JavaStructure stores JavaImports to do lazy loading and prevent circular dependencies
        // If the JavaClass and/or JavaInterfaces are already scanned, the class could store both
        //
        //     extended_class: Vec<JavaClass>,
        //     implemented_interfaces: Vec<JavaInterface>,
        //
        // and
        //
        //     extended_class_imports: Vec<JavaImport>,
        //     implemented_interfaces_imports: Vec<JavaImport>,
        //
        // handling transformation(s)/scan(s) in the getter methods, preventing duplicated scans.
        // So few people would use JavaInterface and JavaClass builders (only reason for this optimization)
        // though, because methods like "JavaClass::from(&existing_file)" are much more convenient.
        let classes = self.get_extended_class_imports();
        let implemented_interfaces = self.get_interfaces_imports();

        let structure = JavaStructure {
            file: self.file.to_owned().ok_or("File output is mandatory")?,
            structure_type: self.structure_type.ok_or("Structure type is mandatory")?,
            struct_body_start_byte: 0,
            struct_body_end_byte: 0,
            annotations: self.annotations.to_owned(),
            visibility: self.visibility,
            is_static: self.is_static,
            is_final: self.is_final,
            is_abstract: self.is_abstract,
            extended_class: classes,
            implemented_interfaces,
            name,
            fields: self.fields.to_owned(),
            methods: self.methods.to_owned(),
            is_root_structure: true,
            substructures: self.substructures.to_owned(),
        };

        structure.write_to_file()?;
        Ok(structure)
    }

    fn get_name(&mut self) -> Result<String, String> {
        Ok(self
            .name
            .to_owned()
            .ok_or("Structure name is mandatory")?
            .to_string())
    }

    fn get_extended_class_imports(&mut self) -> Vec<JavaImport> {
        self.extended_class
            .to_owned()
            .iter()
            .clone()
            .map(|class| class.get_self_import())
            .collect()
    }

    fn get_interfaces_imports(&mut self) -> Vec<JavaImport> {
        self.implemented_interfaces
            .to_owned()
            .iter()
            .clone()
            .map(|class| class.get_self_import())
            .collect()
    }
}

fn log_unrecognized_super_class_type_id_import(
    super_class_node: &JavaNode,
    input_java_file: &Path,
) {
    let log = format!(
        "Unrecognized superclass type id \"{}\" in file:\n{}\n",
        super_class_node.get_content(),
        to_absolute_path_str(input_java_file)
    );
    logger::log_warning(&log);
}

fn log_unrecognized_super_class(super_class_node: &JavaNode, input_java_file: &Path) {
    let log = format!(
        "Unrecognized superclass \"{}\" in file:\n{}\n",
        super_class_node.get_content(),
        to_absolute_path_str(input_java_file)
    );
    logger::log_warning(&log);
}

fn is_second_child_an_extended_class_id(children: &[JavaNode]) -> bool {
    is_first_child_of_type(children, JavaNodeType::Extends)
        && Some(JavaNodeType::TypeIdentifier) == children.get(1).and_then(|t| t.get_node_type_opt())
}

fn extract_interfaces(
    super_interfaces_node: &JavaNode,
    file_imports: &JavaImportsScan,
    input_java_file: &Path,
) -> Vec<JavaImport> {
    let mut result = Vec::new();
    let children = super_interfaces_node.get_children();
    if is_first_child_of_type(children, JavaNodeType::Implements)
        && Some(JavaNodeType::InterfaceTypeList)
            == children.get(1).and_then(|t| t.get_node_type_opt())
    {
        for interface_type in children
            .get(1)
            .expect("Interface already checked")
            .get_children()
        {
            let data_type = JavaDataType::from_import_type_id(
                &interface_type.get_content(),
                file_imports,
                input_java_file,
            );
            if let Ok(JavaDataType::FromImport(import)) = data_type {
                result.push(import);
            } else {
                let log = format!(
                    "Unrecognized interface type id \"{}\" in file:\n{}\n",
                    interface_type.get_content(),
                    to_absolute_path_str(input_java_file)
                );
                logger::log_warning(&log);
            }
        }
        return result;
    }

    let log = format!(
        "Unrecognized interfaces \"{}\" in file:\n{}\n",
        super_interfaces_node.get_content(),
        to_absolute_path_str(input_java_file)
    );
    logger::log_warning(&log);
    Vec::new()
}

fn is_first_child_of_type(children: &[JavaNode], node_type: JavaNodeType) -> bool {
    Some(node_type) == children.get(0).and_then(|t| t.get_node_type_opt())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_creator::file_creator::remove_file_if_exists;
    use crate::core::file_system::path_helper::to_absolute_path_str;
    use crate::core::testing::test_assert::{assert_fail, assert_same_file};
    use crate::core::testing::test_path;
    use crate::java::dependency::org::springframework::spring_context::java_spring_context_factory;
    use crate::java::dto::java_class::JavaClass;
    use crate::java::dto::java_interface::JavaInterface;
    use crate::java::dto::java_visibility::JavaVisibility::{Package, Private, Protected, Public};
    use crate::java::scanner::file::java_structure::JavaStructure;
    use crate::java::scanner::file::java_structure_type::JavaStructureType;

    #[test]
    fn builder_test() {
        let file_path = get_test_file("JavaChildServiceImpl");
        let expected_file_content = get_test_file("ExpectedJavaChildServiceImpl");

        match JavaStructure::builder()
            .file(&file_path)
            .structure_type(JavaStructureType::Class)
            .visibility(Public)
            .name("JavaChildServiceImpl")
            .implemented_interfaces(vec![get_java_interface("JavaInterfaceForStructure")])
            .extended_classes(vec![get_parent_java_class()])
            .write()
        {
            Ok(structure) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, structure.get_file());
                assert_eq!(JavaStructureType::Class, structure.get_type());
                assert_eq!(0, structure.get_annotations().len());
                assert_eq!(Public, structure.get_visibility());
                assert!(!structure.is_static());
                assert!(!structure.is_final());
                assert!(!structure.is_abstract());
                assert_eq!("JavaChildServiceImpl", structure.get_name());
                assert!(structure.get_extended_class().is_some());
                assert_eq!(1, structure.get_implemented_interfaces().len());
                assert_eq!(0, structure.get_fields().len());
                assert_eq!(0, structure.get_methods().len());
                assert_eq!(2, structure.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn generate_public_abstract_class_with_interface() {
        let file_path = get_test_file("JavaServiceAbstract");
        let expected_file_content = get_test_file("ExpectedPublicStaticAbstractClassWithInterface");
        remove_file_if_exists(&file_path);

        match JavaStructure::builder()
            .file(&file_path)
            .structure_type(JavaStructureType::Class)
            .visibility(Public)
            .is_static(true)
            .is_abstract(true)
            .name("JavaServiceAbstract")
            .implemented_interfaces(vec![get_java_interface("JavaInterfaceForStructure")])
            .write()
        {
            Ok(structure) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, structure.get_file());
                assert_eq!(JavaStructureType::Class, structure.get_type());
                assert_eq!(0, structure.get_annotations().len());
                assert_eq!(Public, structure.get_visibility());
                assert!(structure.is_static());
                assert!(structure.is_abstract());
                assert_eq!("JavaServiceAbstract", structure.get_name());
                assert!(structure.get_extended_class().is_none());
                assert_eq!(1, structure.get_implemented_interfaces().len());
                assert_eq!(0, structure.get_fields().len());
                assert_eq!(0, structure.get_methods().len());
                assert_eq!(1, structure.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn generate_package_class_with_interfaces_and_extension() {
        let file_path = get_test_file("PackageClassWithInterfacesAndExtension");
        let expected_file_content = get_test_file("ExpectedPackageClassWithInterfacesAndExtension");
        remove_file_if_exists(&file_path);

        let interfaces = vec![
            get_java_interface("JavaInterfaceForStructure1"),
            get_java_interface("JavaInterfaceForStructure2"),
        ];

        match JavaStructure::builder()
            .file(&file_path)
            .structure_type(JavaStructureType::Class)
            .visibility(Package)
            .name("PackageClassWithInterfacesAndExtension")
            .implemented_interfaces(interfaces)
            .extended_classes(vec![get_parent_java_class()])
            .write()
        {
            Ok(structure) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, structure.get_file());
                assert_eq!(JavaStructureType::Class, structure.get_type());
                assert_eq!(0, structure.get_annotations().len());
                assert_eq!(Package, structure.get_visibility());
                assert!(!structure.is_static());
                assert!(!structure.is_final());
                assert!(!structure.is_abstract());
                assert_eq!(
                    "PackageClassWithInterfacesAndExtension",
                    structure.get_name()
                );
                assert!(structure.get_extended_class().is_some());
                assert_eq!(2, structure.get_implemented_interfaces().len());
                assert_eq!(0, structure.get_fields().len());
                assert_eq!(0, structure.get_methods().len());
                assert_eq!(3, structure.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        }
    }

    #[test]
    fn generate_class_with_annotation() {
        let file_path = get_test_file("JavaServiceBean");
        let expected_file_content = get_test_file("ExpectedClassSkeletonWithAnnotation");
        remove_file_if_exists(&file_path);

        let annotations = vec![java_spring_context_factory::_create_service_annotation_usage()];

        match JavaStructure::builder()
            .file(&file_path)
            .structure_type(JavaStructureType::Class)
            .visibility(Protected)
            .name("JavaServiceBean")
            .annotations(annotations)
            .write()
        {
            Ok(structure) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, structure.get_file());
                assert_eq!(JavaStructureType::Class, structure.get_type());
                assert_eq!(1, structure.get_annotations().len());
                assert_eq!(Protected, structure.get_visibility());
                assert!(!structure.is_static());
                assert!(!structure.is_final());
                assert!(!structure.is_abstract());
                assert_eq!("JavaServiceBean", structure.get_name());
                assert!(structure.get_extended_class().is_none());
                assert_eq!(0, structure.get_implemented_interfaces().len());
                assert_eq!(0, structure.get_fields().len());
                assert_eq!(0, structure.get_methods().len());
                assert_eq!(1, structure.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        };
    }

    #[test]
    fn generate_final_interface() {
        let file_path = get_test_file("JavaFinalInterface");
        let expected_file_content = get_test_file("ExpectedJavaFinalInterface");
        remove_file_if_exists(&file_path);

        match JavaStructure::builder()
            .file(&file_path)
            .structure_type(JavaStructureType::Interface)
            .visibility(Private)
            .is_final(true)
            .name("JavaFinalInterface")
            .write()
        {
            Ok(structure) => {
                assert_same_file(&expected_file_content, &file_path);
                remove_file_if_exists(&file_path);
                assert_eq!(&file_path, structure.get_file());
                assert_eq!(JavaStructureType::Interface, structure.get_type());
                assert_eq!(0, structure.get_annotations().len());
                assert_eq!(Private, structure.get_visibility());
                assert!(!structure.is_static());
                assert!(structure.is_final());
                assert!(!structure.is_abstract());
                assert_eq!("JavaFinalInterface", structure.get_name());
                assert!(structure.get_extended_class().is_none());
                assert_eq!(0, structure.get_implemented_interfaces().len());
                assert_eq!(0, structure.get_fields().len());
                assert_eq!(0, structure.get_methods().len());
                assert_eq!(0, structure.get_imports().len());
            }
            Err(err) => assert_fail(&err),
        };
    }

    fn get_parent_java_class() -> JavaClass {
        get_java_class("JavaParentClassForStructure")
    }

    fn get_java_class(class_name: &str) -> JavaClass {
        let parent_class_path = get_test_file(class_name);

        JavaClass::from(&parent_class_path).expect(
            format!(
                "Invalid java class \"{}\" found in file:\n{:?}",
                class_name,
                to_absolute_path_str(&parent_class_path)
            )
            .as_str(),
        )
    }

    fn get_java_interface(name: &str) -> JavaInterface {
        JavaInterface::from(&get_test_file(name)).unwrap()
    }

    fn get_test_file(structure_name: &str) -> PathBuf {
        get_test_folder().join(format!("{}.java", structure_name).as_str())
    }

    fn get_test_folder() -> PathBuf {
        test_path::get_java_project_test_folder(get_current_file_path(), "java_structure")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
