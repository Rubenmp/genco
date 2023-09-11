use crate::java::parser::dto::java_annotation::JavaAnnotation;
use crate::java::parser::dto::java_node::JavaNode;
use crate::java::parser::dto::java_node_type::JavaNodeType;
use crate::java::scanner::file::dto::java_field_declaration::JavaFieldDeclaration;
use crate::java::scanner::file::dto::java_method::JavaMethod;
use crate::java::scanner::file::dto::java_structure_type::JavaStructureType;

#[allow(unused)]
#[derive(Debug)]
pub struct JavaStructure {
    structure_type: JavaStructureType,
    annotations: Vec<JavaAnnotation>,
    fields: Vec<JavaFieldDeclaration>,
    methods: Vec<JavaMethod>,
    substructures: Vec<JavaStructure>,
}

impl JavaStructure {
    pub fn new(root_node: &JavaNode) -> Self {
        let structure_type_opt: Option<JavaStructureType> =
            get_java_structure_type(root_node.get_node_type());
        let mut annotations = Vec::new();
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut substructures = Vec::new();

        for structure_child in root_node.get_children() {
            if let Some(structure_node_type) = structure_child.get_node_type() {
                if JavaNodeType::Modifiers == structure_node_type {
                    for annotation in JavaAnnotation::new_from_modifiers(&structure_child) {
                        annotations.push(annotation);
                    }
                } else if JavaNodeType::ClassBody == structure_node_type {
                    for body_child in structure_child.get_children() {
                        if let Some(body_node_type) = body_child.get_node_type() {
                            if JavaNodeType::FieldDeclaration == body_node_type {
                                fields.push(JavaFieldDeclaration::new(&body_child));
                            } else if JavaNodeType::MethodDecl == body_node_type {
                                methods.push(JavaMethod::new(&body_child));
                            }
                        }
                    }
                } else if is_java_structure_type(Some(structure_node_type)) {
                    substructures.push(JavaStructure::new(structure_child));
                }
            }
        }

        JavaStructure {
            structure_type: structure_type_opt
                .expect("Invalid structure type building JavaStructure"),
            annotations,
            fields,
            methods,
            substructures,
        }
    }

    pub fn add_annotation(&mut self, annotation: JavaAnnotation) {
        self.annotations.push(annotation);
    }
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
