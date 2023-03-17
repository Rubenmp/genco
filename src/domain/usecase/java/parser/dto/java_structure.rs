use crate::domain::usecase::java::parser::dto::java_annotation::JavaAnnotation;
use crate::domain::usecase::java::parser::dto::java_method::JavaMethod;
use crate::domain::usecase::java::parser::dto::java_node::JavaNode;
use crate::domain::usecase::java::parser::dto::java_node_type::JavaNodeType;
use crate::domain::usecase::java::parser::dto::java_structure_type::JavaStructureType;

#[allow(unused)]
#[derive(Debug)]
pub struct JavaStructure {
    structure_type: JavaStructureType,
    annotations: Vec<JavaAnnotation>,
    methods: Vec<JavaMethod>,
    substructures: Vec<JavaStructure>,
}

impl JavaStructure {
    pub fn new(root_node: &JavaNode) -> Self {
        let structure_type_opt: Option<JavaStructureType> =
            get_java_structure_type(root_node.get_node_type());
        let mut annotations = Vec::new();
        let mut methods = Vec::new();
        let mut substructures = Vec::new();

        for child in root_node.get_children() {
            if let Some(node_type) = child.get_node_type() {
                if JavaNodeType::Annotation == node_type
                    || JavaNodeType::MarkerAnnotation == node_type
                {
                    annotations.push(JavaAnnotation::new(&child));
                } else if JavaNodeType::MethodDecl == node_type {
                    methods.push(JavaMethod::new(&child));
                } else if is_java_structure_type(Some(node_type)) {
                    substructures.push(JavaStructure::new(child));
                }
            }
        }

        JavaStructure {
            structure_type: structure_type_opt
                .expect("Invalid structure type building JavaStructure"),
            annotations,
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
