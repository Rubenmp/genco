use std::path::Path;

use crate::core::file_system::file_cache::FileCache;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::annotation_usage::JavaAnnotationUsage;
use crate::java::class::JavaClass;
use crate::java::data_type::JavaDataType;
use crate::java::field::JavaField;
use crate::java::import::JavaImport;
use crate::java::indentation_config::JavaIndentation;
use crate::java::interface::JavaInterface;
use crate::java::method::JavaMethod;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file_imports;
use crate::java::scanner::file::java_file_imports::JavaFileImports;
use crate::java::scanner::file::java_structure_type::JavaStructureType;
use crate::java::visibility::JavaVisibility;
use crate::java::{annotation_usage, visibility};

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct JavaStructure {
    // Metadata
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
    substructures: Vec<JavaStructure>,
}

impl JavaStructure {
    pub(crate) fn builder() -> JavaStructureBuilder {
        JavaStructureBuilder::new_builder()
    }

    pub(crate) fn new(
        root_struct_decl_node: &JavaNode,
        file_imports: &JavaFileImports,
        file_cache: &FileCache,
    ) -> Result<Self, String> {
        new_structure_internal(root_struct_decl_node, file_imports, file_cache)
    }

    pub(crate) fn get_type(&self) -> JavaStructureType {
        self.structure_type
    }

    pub(crate) fn get_start_byte(&self) -> usize {
        self.struct_body_start_byte
    }

    pub(crate) fn get_annotations(&self) -> &Vec<JavaAnnotationUsage> {
        &self.annotations
    }

    pub(crate) fn get_visibility(&self) -> JavaVisibility {
        self.visibility
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

    pub(crate) fn get_imports(&self) -> Vec<JavaImport> {
        let mut imports = Vec::new();
        for import in self.get_annotation_imports() {
            imports.push(import.clone());
        }

        if let Some(extended_class) = self.get_extended_class() {
            imports.push(extended_class.get_self_import().clone());
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

    pub(crate) fn get_imports_sorted_asc(&self) -> Vec<JavaImport> {
        java_file_imports::get_sorted_asc(self.get_imports())
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

    fn get_annotation_imports(&self) -> Vec<&JavaImport> {
        self.get_annotations()
            .iter()
            .flat_map(|annotation| annotation.get_imports())
            .collect()
    }

    pub(crate) fn get_skeleton_without_imports(&self) -> String {
        let mut result = "".to_string();
        self.write_annotations(&mut result);
        self.write_visibility(&mut result);
        result += self.get_name();
        self.write_extensions_and_implementations(&mut result);
        result
    }

    pub(crate) fn write_body(&self, result: &mut String) {
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

    fn write_visibility(&self, result: &mut String) {
        *result += self.get_visibility().as_file_string().as_str();
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
            JavaStructureType::Record => *result += "record ",
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
}

fn new_structure_internal(
    root_node: &JavaNode,
    file_imports: &JavaFileImports,
    java_file_cache: &FileCache,
) -> Result<JavaStructure, String> {
    let structure_type_opt: Option<JavaStructureType> =
        get_java_structure_type(root_node.get_node_type());
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
    let input_java_file = java_file_cache.get_path();

    for child_node in root_node.get_children() {
        if let Some(structure_node_type) = child_node.get_node_type() {
            if JavaNodeType::Modifiers == structure_node_type {
                for modifier in child_node.get_children() {
                    if let Some(node_type) = modifier.get_node_type() {
                        if annotation_usage::is_java_node_annotation(&node_type) {
                            match JavaAnnotationUsage::new_from_java_node_unchecked(
                                modifier,
                                file_imports,
                                java_file_cache,
                            ) {
                                Ok(annotation) => annotations.push(annotation),
                                Err(err) => logger::log_warning(&err),
                            };
                        } else if visibility::is_visibility_node_type(&node_type) {
                            visibility = visibility::new(&node_type);
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
                name_opt = Some(child_node.get_content_from_cache(java_file_cache));
            } else if JavaNodeType::Superclass == structure_node_type {
                if let Some(import) = extract_super_class(file_imports, java_file_cache, child_node)
                {
                    extended_class.push(import);
                }
            } else if JavaNodeType::SuperInterfaces == structure_node_type {
                match extract_interfaces(child_node, file_imports, java_file_cache) {
                    Ok(result) => implemented_interfaces = result,
                    Err(err) => logger::log_warning(&err),
                }
            } else if is_structure_body(&structure_node_type) {
                for body_child in child_node.get_children() {
                    if let Some(body_node_type) = body_child.get_node_type() {
                        if JavaNodeType::FieldDeclaration == body_node_type {
                            match JavaField::new(body_child, file_imports, java_file_cache) {
                                Ok(field) => fields.push(field),
                                Err(err) => logger::log_warning(&err),
                            };
                        } else if JavaNodeType::MethodDecl == body_node_type {
                            match JavaMethod::new_from_node(
                                body_child,
                                file_imports,
                                java_file_cache,
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
                match new_structure_internal(child_node, file_imports, java_file_cache) {
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
        substructures,
    })
}

fn extract_super_class(
    file_imports: &JavaFileImports,
    input_java_file_cache: &FileCache,
    child_node: &JavaNode,
) -> Option<JavaImport> {
    let children = child_node.get_children();
    if !is_second_child_an_extended_class_id(children) {
        log_unrecognized_super_class(child_node, input_java_file_cache);
        return None;
    }

    let second_child_with_extended_class_type = children
        .get(1)
        .expect("Super class must have a second child");
    match JavaDataType::from_data_type_identifier_with_import(
        second_child_with_extended_class_type,
        file_imports,
        input_java_file_cache,
    ) {
        Ok(data_type) => {
            // TODO: filter here that the import is from an interface
            if let Some(import) = data_type.get_import_opt() {
                return Some(import);
            }
            log_unrecognized_super_class_type_id_import(child_node, input_java_file_cache);
            None
        }
        Err(err) => {
            logger::log_warning(&err);
            None
        }
    }
}

fn log_invalid_method_decl(input_java_file: &Path, err: String) {
    logger::log_warning(
        format!(
            "Invalid method ({}) in file:\n{}\n",
            err,
            try_to_absolute_path(input_java_file)
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

pub(crate) struct JavaStructureBuilder {
    structure_type: Option<JavaStructureType>,
    annotations: Vec<JavaAnnotationUsage>,
    visibility: JavaVisibility,
    is_static: bool,
    is_final: bool,
    is_abstract: bool,
    extended_class: Vec<JavaImport>,
    implemented_interfaces: Vec<JavaImport>,
    name: Option<String>,
    fields: Vec<JavaField>,
    methods: Vec<JavaMethod>,
}

impl JavaStructureBuilder {
    fn new_builder() -> Self {
        Self {
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
        }
    }
    pub fn structure_type(&mut self, input: JavaStructureType) -> &mut Self {
        self.structure_type = Some(input);
        self
    }
    pub fn annotations(&mut self, input: Vec<JavaAnnotationUsage>) -> &mut Self {
        self.annotations = input.clone();
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
    pub fn extended_classes(&mut self, input: Vec<JavaImport>) -> &mut Self {
        self.extended_class = input;
        self
    }
    pub fn implemented_interfaces(&mut self, input: Vec<JavaImport>) -> &mut Self {
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

    pub fn build(&self) -> Result<JavaStructure, String> {
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
            structure_type: self.structure_type.ok_or("Structure type is mandatory")?,
            struct_body_start_byte: 0,
            struct_body_end_byte: 0,
            annotations: self.annotations.clone(),
            visibility: self.visibility,
            is_static: self.is_static,
            is_final: self.is_final,
            is_abstract: self.is_abstract,
            extended_class: classes.clone(),
            implemented_interfaces: implemented_interfaces.clone(),
            name,
            fields: self.fields.clone(),
            methods: self.methods.clone(),
            substructures: vec![],
        };

        Ok(structure)
    }

    fn get_name(&self) -> Result<String, String> {
        Ok(self
            .name
            .clone()
            .ok_or("Structure name is mandatory")?
            .to_string())
    }

    fn get_extended_class_imports(&self) -> &Vec<JavaImport> {
        &self.extended_class
    }

    fn get_interfaces_imports(&self) -> &Vec<JavaImport> {
        &self.implemented_interfaces
    }
}

fn log_unrecognized_super_class_type_id_import(
    super_class_node: &JavaNode,
    java_file_cache: &FileCache,
) {
    let log = format!(
        "Unrecognized superclass type id \"{}\" in file:\n{}\n",
        super_class_node.get_content_from_cache(java_file_cache),
        try_to_absolute_path(java_file_cache.get_path())
    );
    logger::log_warning(&log);
}

fn log_unrecognized_super_class(super_class_node: &JavaNode, java_file_cache: &FileCache) {
    let log = format!(
        "Unrecognized superclass \"{}\" in file:\n{}\n",
        super_class_node.get_content_from_cache(java_file_cache),
        try_to_absolute_path(java_file_cache.get_path())
    );
    logger::log_warning(&log);
}

fn is_second_child_an_extended_class_id(children: &[JavaNode]) -> bool {
    let second_child_node_type = children.get(1).and_then(|t| t.get_node_type());
    is_first_child_of_type(children, JavaNodeType::Extends)
        && (Some(JavaNodeType::TypeIdentifier) == second_child_node_type
            || Some(JavaNodeType::ScopedTypeIdentifier) == second_child_node_type)
}

fn extract_interfaces(
    super_interfaces_node: &JavaNode,
    file_imports: &JavaFileImports,
    input_java_file_cache: &FileCache,
) -> Result<Vec<JavaImport>, String> {
    let mut result = Vec::new();
    let children = super_interfaces_node.get_children();
    if is_first_child_of_type(children, JavaNodeType::Implements)
        && Some(JavaNodeType::InterfaceTypeList) == children.get(1).and_then(|t| t.get_node_type())
    {
        for interface_type in children
            .get(1)
            .expect("Interface already checked")
            .get_children()
        {
            let data_type = JavaDataType::from_data_type_identifier_with_import(
                interface_type,
                file_imports,
                input_java_file_cache,
            )
            .expect("Interface definition must be ok");
            if let Some(import) = data_type.get_import_opt() {
                result.push(import);
            } else {
                let log = format!(
                    "Unrecognized interface type id \"{}\" in file:\n{}\n",
                    interface_type.get_content_from_cache(input_java_file_cache),
                    try_to_absolute_path(input_java_file_cache.get_path())
                );
                logger::log_warning(&log);
            }
        }

        return Ok(result);
    }

    let log = format!(
        "Unrecognized interfaces \"{}\" in file:\n{}\n",
        super_interfaces_node.get_content_from_cache(input_java_file_cache),
        try_to_absolute_path(input_java_file_cache.get_path())
    );
    logger::log_warning(&log);
    Ok(Vec::new())
}

fn is_first_child_of_type(children: &[JavaNode], node_type: JavaNodeType) -> bool {
    Some(node_type) == children.get(0).and_then(|t| t.get_node_type())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::path_helper::try_to_absolute_path;
    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path;
    use crate::java::class::JavaClass;
    use crate::java::dependency::org::springframework::spring_context::java_spring_context_factory;
    use crate::java::import::JavaImport;
    use crate::java::interface::JavaInterface;
    use crate::java::scanner::file::java_structure::JavaStructure;
    use crate::java::scanner::file::java_structure_type::JavaStructureType;
    use crate::java::visibility::JavaVisibility::{Package, Private, Protected, Public};

    #[test]
    fn builder_test() {
        match JavaStructure::builder()
            .structure_type(JavaStructureType::Class)
            .is_final(true)
            .visibility(Public)
            .name("JavaChildServiceImpl")
            .implemented_interfaces(vec![get_java_interface_import("JavaInterfaceForStructure")])
            .extended_classes(vec![get_parent_java_class_import()])
            .build()
        {
            Ok(structure) => {
                assert_eq!(JavaStructureType::Class, structure.get_type());
                assert_eq!(0, structure.get_annotations().len());
                assert_eq!(Public, structure.get_visibility());
                assert!(!structure.is_static());
                assert!(structure.is_final());
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
        match JavaStructure::builder()
            .structure_type(JavaStructureType::Class)
            .visibility(Public)
            .is_static(true)
            .is_abstract(true)
            .name("JavaServiceAbstract")
            .implemented_interfaces(vec![get_java_interface_import("JavaInterfaceForStructure")])
            .build()
        {
            Ok(structure) => {
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
        let interfaces = vec![
            get_java_interface_import("JavaInterfaceForStructure1"),
            get_java_interface_import("JavaInterfaceForStructure2"),
        ];

        match JavaStructure::builder()
            .structure_type(JavaStructureType::Class)
            .visibility(Package)
            .name("PackageClassWithInterfacesAndExtension")
            .implemented_interfaces(interfaces)
            .extended_classes(vec![get_parent_java_class_import()])
            .build()
        {
            Ok(structure) => {
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
        let annotations = vec![java_spring_context_factory::_create_service_annotation_usage()];

        match JavaStructure::builder()
            .structure_type(JavaStructureType::Class)
            .visibility(Protected)
            .name("JavaServiceBean")
            .annotations(annotations)
            .build()
        {
            Ok(structure) => {
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
        match JavaStructure::builder()
            .structure_type(JavaStructureType::Interface)
            .visibility(Private)
            .is_final(true)
            .name("JavaFinalInterface")
            .build()
        {
            Ok(structure) => {
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

    fn get_parent_java_class_import() -> JavaImport {
        get_parent_java_class().get_self_import()
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
                try_to_absolute_path(&parent_class_path)
            )
            .as_str(),
        )
    }

    fn get_java_interface_import(name: &str) -> JavaImport {
        get_java_interface(name).get_self_import()
    }

    fn get_java_interface(name: &str) -> JavaInterface {
        JavaInterface::from(&get_test_file(name))
            .expect("get_java_interface failed crating interface")
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
