use std::fmt;
use std::path::Path;

use crate::core::file_system::file_cache::FileCache;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::import::JavaImport;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;
use crate::java::scanner::file::java_file::JavaFile;
use crate::java::scanner::file::java_file_imports::JavaFileImports;

#[derive(Debug, Clone)]
pub struct JavaDataType {
    primitive: Option<JavaPrimitiveDataType>,
    non_primitive: Option<JavaNonPrimitiveDataType>,
}

// Public methods
impl JavaDataType {
    pub fn from_path(java_file_path: &Path) -> Self {
        let java_file = JavaFile::from_user_input_path(java_file_path).unwrap();
        let import = java_file.get_self_import();
        let result = JavaNonPrimitiveDataType::from_explicit_import(import);

        Self::from_non_primitive(result)
    }

    pub fn int() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Int)
    }

    pub fn long() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Long)
    }

    pub fn float() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Float)
    }

    pub fn double() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Double)
    }

    pub fn boolean() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Boolean)
    }

    pub fn string() -> Self {
        let non_primitive =
            JavaNonPrimitiveDataType::without_import(JavaNonPrimitiveDataTypeWithoutImport::String);
        Self::from_non_primitive(non_primitive)
    }

    pub fn char() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Char)
    }

    pub fn byte() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Byte)
    }

    pub fn short() -> Self {
        Self::from_primitive(JavaPrimitiveDataType::Short)
    }

    fn get_primitive(&self) -> &Option<JavaPrimitiveDataType> {
        &self.primitive
    }

    fn get_non_primitive(&self) -> &Option<JavaNonPrimitiveDataType> {
        &self.non_primitive
    }
}

// Public crate methods
impl JavaDataType {
    pub(crate) fn from_import(import: JavaImport) -> Self {
        Self::from_non_primitive(JavaNonPrimitiveDataType::from_explicit_import(import))
    }
    pub(crate) fn get_import_opt(&self) -> Option<JavaImport> {
        if let Some(non_primitive) = self.non_primitive.to_owned() {
            return non_primitive.get_import();
        }

        None
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
            return Ok(JavaDataType::boolean());
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
        let result = JavaDataType::from_import(explicit_import);

        Ok(result)
    }

    pub(crate) fn new_basic_data_type(basic_type: &str) -> Option<Self> {
        if let Some(primitive) = match basic_type {
            "int" => Some(JavaPrimitiveDataType::Int),
            "long" => Some(JavaPrimitiveDataType::Long),
            "float" => Some(JavaPrimitiveDataType::Float),
            "double" => Some(JavaPrimitiveDataType::Double),
            "char" => Some(JavaPrimitiveDataType::Char),
            "boolean" => Some(JavaPrimitiveDataType::Boolean),
            "byte" => Some(JavaPrimitiveDataType::Byte),
            "short" => Some(JavaPrimitiveDataType::Short),
            _ => None,
        } {
            return Some(Self::from_primitive(primitive));
        }

        if let Some(non_primitive) = match basic_type {
            "Integer" => Some(JavaNonPrimitiveDataTypeWithoutImport::IntClass),
            "Long" => Some(JavaNonPrimitiveDataTypeWithoutImport::LongClass),
            "Float" => Some(JavaNonPrimitiveDataTypeWithoutImport::FloatClass),
            "Double" => Some(JavaNonPrimitiveDataTypeWithoutImport::DoubleClass),
            "Boolean" => Some(JavaNonPrimitiveDataTypeWithoutImport::BooleanClass),
            "String" => Some(JavaNonPrimitiveDataTypeWithoutImport::String),
            "Byte" => Some(JavaNonPrimitiveDataTypeWithoutImport::ByteClass),
            "Short" => Some(JavaNonPrimitiveDataTypeWithoutImport::ShortClass),
            _ => None,
        } {
            let result_non_primitive = JavaNonPrimitiveDataType::without_import(non_primitive);
            return Some(Self::from_non_primitive(result_non_primitive));
        }

        None
    }
}

// Private methods
impl JavaDataType {
    fn from_non_primitive(result: JavaNonPrimitiveDataType) -> JavaDataType {
        Self {
            primitive: None,
            non_primitive: Some(result),
        }
    }

    fn from_primitive(primitive: JavaPrimitiveDataType) -> JavaDataType {
        Self {
            primitive: Some(primitive),
            non_primitive: None,
        }
    }

    fn from_data_type_identifier_including_basic_data_type(
        type_id_node: &JavaNode,
        file_imports: &JavaFileImports,
        java_file_cache: &FileCache,
    ) -> Result<JavaDataType, String> {
        let type_id = type_id_node.get_content_from_cache(java_file_cache);
        let basic_data_type = Self::new_basic_data_type(&type_id);
        if let Some(data_type) = basic_data_type {
            return Ok(data_type);
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
            Ok(JavaDataType::float())
        } else if JavaNodeType::Double == child_node_type {
            Ok(JavaDataType::double())
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
            Ok(JavaDataType::int())
        } else if JavaNodeType::Long == child_node_type {
            Ok(JavaDataType::long())
        } else if JavaNodeType::Byte == child_node_type {
            Ok(JavaDataType::byte())
        } else if JavaNodeType::Short == child_node_type {
            Ok(JavaDataType::short())
        } else if JavaNodeType::Char == child_node_type {
            Ok(JavaDataType::char())
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

#[derive(Debug, Clone)]
pub(crate) enum JavaPrimitiveDataType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    Boolean,
}

#[derive(Debug, Clone)]
pub(crate) enum JavaNonPrimitiveDataTypeWithoutImport {
    ByteClass,
    ShortClass,
    IntClass,
    LongClass,
    FloatClass,
    DoubleClass,
    BooleanClass,
    String,
}

#[derive(Debug, Clone)]
pub(crate) struct JavaNonPrimitiveDataType {
    without_import_opt: Option<JavaNonPrimitiveDataTypeWithoutImport>,
    java_import_opt: Option<JavaImport>,
}

impl JavaNonPrimitiveDataType {
    fn get_import(&self) -> Option<JavaImport> {
        if let Some(java_import) = self.java_import_opt.clone() {
            return Some(java_import);
        }

        None
    }
    fn from_explicit_import(import: JavaImport) -> Self {
        Self {
            without_import_opt: None,
            java_import_opt: Some(import),
        }
    }

    fn without_import(input_type: JavaNonPrimitiveDataTypeWithoutImport) -> Self {
        Self {
            without_import_opt: Some(input_type),
            java_import_opt: None,
        }
    }
}

impl fmt::Display for JavaDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(primitive) = self.get_primitive() {
            write!(f, "{}", primitive)?;
        }

        if let Some(non_primitive) = self.get_non_primitive() {
            write!(f, "{}", non_primitive)?;
        }

        Ok(())
    }
}

impl fmt::Display for JavaPrimitiveDataType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            JavaPrimitiveDataType::Byte => "byte".to_string(),
            JavaPrimitiveDataType::Short => "short".to_string(),
            JavaPrimitiveDataType::Int => "int".to_string(),
            JavaPrimitiveDataType::Long => "long".to_string(),
            JavaPrimitiveDataType::Float => "float".to_string(),
            JavaPrimitiveDataType::Double => "double".to_string(),
            JavaPrimitiveDataType::Char => "char".to_string(),
            JavaPrimitiveDataType::Boolean => "boolean".to_string(),
        };

        write!(fmt, "{}", string)?;
        Ok(())
    }
}

impl fmt::Display for JavaNonPrimitiveDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(without_import) = self.without_import_opt.clone() {
            write!(f, "{}", without_import)?;
        }
        if let Some(with_import) = self.java_import_opt.clone() {
            write!(f, "{}", with_import.get_last_node())?;
        }

        Ok(())
    }
}

impl fmt::Display for JavaNonPrimitiveDataTypeWithoutImport {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            JavaNonPrimitiveDataTypeWithoutImport::ByteClass => "Byte".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::ShortClass => "Short".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::IntClass => "Integer".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::LongClass => "Long".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::FloatClass => "Float".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::DoubleClass => "Double".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::BooleanClass => "Boolean".to_string(),
            JavaNonPrimitiveDataTypeWithoutImport::String => "String".to_string(),
        };

        write!(fmt, "{}", string)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::testing::test_assert::assert_fail;
    use crate::core::testing::test_path::get_java_test_file;
    use crate::java::data_type::JavaDataType;

    #[test]
    fn new_class() {
        let file_path = get_java_test_file(
            get_current_file_path(),
            "java_data_type",
            "JavaDataTypeClass.java",
        );

        let returned_type = JavaDataType::from_path(&file_path);

        if let Some(import) = returned_type.get_import_opt() {
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

        let returned_type = JavaDataType::from_path(&file_path);

        if let Some(import) = returned_type.get_import_opt() {
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

        let returned_type = JavaDataType::from_path(&file_path);

        if let Some(import) = returned_type.get_import_opt() {
            assert_eq!("org.test.JavaDataTypeInterface", import.get_route());
        } else {
            assert_fail("Expected interface data type");
        }
    }

    #[test]
    fn java_get_imports_trait() {
        let java_type = JavaDataType::int();

        let import_opt = java_type.get_import_opt();

        assert!(import_opt.is_none())
    }

    #[test]
    fn to_string_basic_data_type() {
        assert_eq!("int", JavaDataType::int().to_string());
        assert_eq!("String", JavaDataType::string().to_string());
    }

    pub fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
