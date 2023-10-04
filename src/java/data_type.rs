use crate::core::file_system::file_cache::FileCache;
use std::fmt;
use std::path::Path;

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::import::JavaImport;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file::JavaFile;
use crate::java::scanner::file::java_file_imports::JavaFileImports;

#[derive(Debug, Clone)]
pub enum JavaDataType {
    Basic(JavaBasicDataType),
    //Class(JavaImport),
    //Interface(JavaImport),
    //Enum(JavaImport),
    FromImport(JavaImport),
}

// Public methods
impl JavaDataType {
    pub fn new_from_path(java_file_path: &Path) -> JavaDataType {
        let java_file = JavaFile::from_user_input_path(java_file_path).unwrap();
        let import = java_file.get_self_import();

        JavaDataType::FromImport(import)
    }
}

// Public crate methods
impl JavaDataType {
    // Crate or private methods
    pub(crate) fn get_import(&self) -> Option<&JavaImport> {
        match &self {
            JavaDataType::Basic(_) => None,
            JavaDataType::FromImport(import) => Some(import),
        }
    }

    pub(crate) fn get_data_type(
        data_type_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<Self, String> {
        let node_type = data_type_node
            .get_node_type()
            .ok_or("Unexpected node type")?;
        if node_type.is_data_type_id_identifier() {
            return JavaDataType::from_data_type_identifier_including_basic_data_type(
                data_type_node,
                file_imports,
                java_file_cache,
            );
        } else if JavaNodeType::Boolean == node_type {
            return Ok(JavaDataType::Basic(JavaBasicDataType::Boolean));
        } else if JavaNodeType::IntegralType == node_type {
            return Self::get_data_type_from_integral_type(data_type_node, java_file_cache);
        } else if JavaNodeType::FloatingPointType == node_type {
            return Self::get_data_type_from_floating_point_type(data_type_node, java_file_cache);
        }

        Err(format!(
            "Unrecognized JavaNodeType parsing \"{}\" in file:\n{}\n",
            data_type_node
                .get_content_from_cache(java_file_cache)
                .as_str(),
            try_to_absolute_path(java_file_cache.get_path())
        ))
    }

    pub(crate) fn from_data_type_identifier_with_import(
        type_id_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<JavaDataType, String> {
        let explicit_import =
            file_imports.get_explicit_import_from_identifier(type_id_node, java_file_cache)?;
        let result = JavaDataType::FromImport(explicit_import);

        Ok(result)
    }
}

// Private methods
impl JavaDataType {
    fn from_data_type_identifier_including_basic_data_type(
        type_id_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<JavaDataType, String> {
        let type_id = type_id_node.get_content_from_cache(java_file_cache);
        let basic_data_type = new_basic_data_type(&type_id);
        if let Some(data_type) = basic_data_type {
            return Ok(JavaDataType::Basic(data_type));
        }

        Self::from_data_type_identifier_with_import(type_id_node, file_imports, java_file_cache)
    }

    fn get_data_type_from_floating_point_type(
        data_type_node: &JavaNode,
        java_file_cache: &FileCache,
    ) -> Result<JavaDataType, String> {
        let child_node = data_type_node
            .get_children()
            .first()
            .expect("First java child node expected");
        let child_node_type = child_node
            .get_node_type()
            .expect("First java child node type expected");

        if JavaNodeType::Float == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Float))
        } else if JavaNodeType::Double == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Double))
        } else {
            Err(format!(
                "Unrecognized FloatingPointType JavaNodeType parsing \"{}\" in file:\n{}\n",
                data_type_node
                    .get_content_from_cache(java_file_cache)
                    .as_str(),
                try_to_absolute_path(java_file_cache.get_path())
            ))
        }
    }

    fn get_data_type_from_integral_type(
        data_type_node: &JavaNode,
        java_file_cache: &FileCache,
    ) -> Result<JavaDataType, String> {
        let child_node = data_type_node.get_children().get(0).ok_or(format!(
            "Missing mandatory child node building IntegralType JavaNodeType \"{}\" in file:\n{}\n",
            data_type_node.get_content_from_cache(java_file_cache),
            try_to_absolute_path(java_file_cache.get_path())
        ))?;
        let child_node_type = child_node.get_node_type().ok_or(format!(
            "Missing mandatory node type building IntegralType JavaNodeType \"{}\" in file:\n{}\n",
            data_type_node.get_content_from_cache(java_file_cache),
            try_to_absolute_path(java_file_cache.get_path())
        ))?;

        if JavaNodeType::Int == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Int))
        } else if JavaNodeType::Long == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Long))
        } else if JavaNodeType::Byte == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Byte))
        } else if JavaNodeType::Short == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Short))
        } else if JavaNodeType::Char == child_node_type {
            Ok(JavaDataType::Basic(JavaBasicDataType::Char))
        } else {
            Err(format!(
                "Unrecognized Integral JavaNodeType parsing \"{}\" in file:\n{}\n",
                data_type_node
                    .get_content_from_cache(java_file_cache)
                    .as_str(),
                try_to_absolute_path(java_file_cache.get_path())
            ))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum JavaBasicDataType {
    Byte,
    // byte
    ByteClass,
    // Byte
    Short,
    // short
    ShortClass,
    // Short
    Int,
    // int
    IntClass,
    // Integer
    Long,
    // long
    LongClass,
    // Long
    Float,
    // float
    FloatClass,
    // Float
    Double,
    // double
    DoubleClass,
    // Double
    Char,
    Boolean,
    // boolean
    BooleanClass,
    // Boolean
    String,
}

impl fmt::Display for JavaBasicDataType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            JavaBasicDataType::Byte => "byte".to_string(),
            JavaBasicDataType::ByteClass => "Byte".to_string(),
            JavaBasicDataType::Short => "short".to_string(),
            JavaBasicDataType::ShortClass => "Short".to_string(),
            JavaBasicDataType::Int => "int".to_string(),
            JavaBasicDataType::IntClass => "Integer".to_string(),
            JavaBasicDataType::Long => "long".to_string(),
            JavaBasicDataType::LongClass => "Long".to_string(),
            JavaBasicDataType::Float => "float".to_string(),
            JavaBasicDataType::FloatClass => "Float".to_string(),
            JavaBasicDataType::Double => "double".to_string(),
            JavaBasicDataType::DoubleClass => "Double".to_string(),
            JavaBasicDataType::Char => "char".to_string(),
            JavaBasicDataType::Boolean => "boolean".to_string(),
            JavaBasicDataType::BooleanClass => "Boolean".to_string(),
            JavaBasicDataType::String => "String".to_string(),
        };

        write!(fmt, "{}", string)?;
        Ok(())
    }
}

fn new_basic_data_type(java_node_content: &str) -> Option<JavaBasicDataType> {
    match java_node_content {
        "int" => Some(JavaBasicDataType::Int),
        "Integer" => Some(JavaBasicDataType::IntClass),
        "long" => Some(JavaBasicDataType::Long),
        "Long" => Some(JavaBasicDataType::LongClass),
        "float" => Some(JavaBasicDataType::Float),
        "Float" => Some(JavaBasicDataType::FloatClass),
        "double" => Some(JavaBasicDataType::Double),
        "Double" => Some(JavaBasicDataType::DoubleClass),
        "char" => Some(JavaBasicDataType::Char),
        "boolean" => Some(JavaBasicDataType::Boolean),
        "Boolean" => Some(JavaBasicDataType::BooleanClass),
        "String" => Some(JavaBasicDataType::String),
        "byte" => Some(JavaBasicDataType::Byte),
        "Byte" => Some(JavaBasicDataType::ByteClass),
        "short" => Some(JavaBasicDataType::Short),
        "Short" => Some(JavaBasicDataType::ShortClass),
        _ => None,
    }
}

impl fmt::Display for JavaDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        if let JavaDataType::Basic(basic_data_type) = self {
            string += basic_data_type.to_string().as_str();
        } else if let JavaDataType::FromImport(java_import) = self {
            string += java_import.get_last_node().as_str();
        }

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_java_test_file;
    use crate::java::data_type::{JavaBasicDataType, JavaDataType};

    #[test]
    fn new_class() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "java_data_type",
            "JavaDataTypeClass.java",
        );

        let returned_type = JavaDataType::new_from_path(&file_path);

        if let JavaDataType::FromImport(import) = returned_type {
            assert!(import.is_explicit_import());
            assert_eq!("org.test.JavaDataTypeClass", import.get_route());
        } else {
            assert_fail("Expected class data type");
        }
    }

    #[test]
    fn new_enum() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "java_data_type",
            "JavaDataTypeEnum.java",
        );

        let returned_type = JavaDataType::new_from_path(&file_path);

        if let JavaDataType::FromImport(import) = returned_type {
            assert!(import.is_explicit_import());
            assert_eq!("org.test.JavaDataTypeEnum", import.get_route());
        } else {
            assert_fail("Expected enum data type");
        }
    }

    #[test]
    fn new_interface() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "java_data_type",
            "JavaDataTypeInterface.java",
        );

        let returned_type = JavaDataType::new_from_path(&file_path);

        if let JavaDataType::FromImport(import) = returned_type {
            assert!(import.is_explicit_import());
            assert_eq!("org.test.JavaDataTypeInterface", import.get_route());
        } else {
            assert_fail("Expected interface data type");
        }
    }

    #[test]
    fn java_get_imports_trait() {
        let java_type = JavaDataType::Basic(JavaBasicDataType::Int);

        let import_opt = java_type.get_import();

        assert!(import_opt.is_none())
    }

    #[test]
    fn to_string_basic_data_type() {
        assert_eq!(
            "int",
            JavaDataType::Basic(JavaBasicDataType::Int).to_string()
        );
        assert_eq!(
            "String",
            JavaDataType::Basic(JavaBasicDataType::String).to_string()
        );
    }

    pub fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
