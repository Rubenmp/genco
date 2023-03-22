use std::fmt;

#[derive(Debug)]
pub enum JavaDataType {
    Basic(JavaBasicDataType),
    CompoundDataType,
    Class,
}

impl JavaDataType {}

impl JavaDataType {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        if let JavaDataType::Basic(basic_data_type) = self {
            string += basic_data_type.to_string().as_str();
        }
        string
    }
}

#[derive(Debug)]
pub enum JavaBasicDataType {
    Byte,        // byte
    ByteClass,   // Byte
    Short,       // short
    ShortClass,  // Short
    Int,         // int
    IntClass,    // Integer
    Long,        // long
    LongClass,   // Long
    Float,       // float
    FloatClass,  // Float
    Double,      // double
    DoubleClass, // Double
    Char,
    Boolean,      // boolean
    BooleanClass, // Boolean
    String,
}

impl fmt::Display for JavaBasicDataType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;
        Ok(())
    }
}

pub(crate) fn new(java_node_content: &str) -> JavaDataType {
    let basic_data_type = new_basic_data_type(java_node_content);
    if let Some(data_type) = basic_data_type {
        return JavaDataType::Basic(data_type);
    }

    JavaDataType::Class
}

fn new_basic_data_type(java_node_content: &str) -> Option<JavaBasicDataType> {
    match java_node_content {
        "byte" => Some(JavaBasicDataType::Byte),
        "Byte" => Some(JavaBasicDataType::ByteClass),
        "short" => Some(JavaBasicDataType::Short),
        "Short" => Some(JavaBasicDataType::ShortClass),
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
        _ => None,
    }
}
