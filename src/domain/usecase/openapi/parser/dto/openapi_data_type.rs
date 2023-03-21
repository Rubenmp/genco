use crate::domain::usecase::openapi::parser::dto::openapi_schema::OpenapiSchema;
use std::fmt;

#[derive(Debug, Clone)]
pub enum OpenapiDataType {
    Integer(IntegerFormat),
    String,
    Null,
    Boolean,
    Number(NumberFormat),
    Bytes,
    ObjectSimple,
    Object(Box<OpenapiSchema>),
    ObjectName(String),
    Array(Vec<OpenapiDataType>),
}

#[derive(Debug, Clone)]
pub enum IntegerFormat {
    Int32,
    Int64,
}

impl fmt::Display for IntegerFormat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum NumberFormat {
    Float,
    Double,
}

impl fmt::Display for NumberFormat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;
        Ok(())
    }
}

impl OpenapiDataType {
    pub fn new_int32_type() -> OpenapiDataType {
        OpenapiDataType::Integer(IntegerFormat::Int32)
    }
    pub fn new_int64_type() -> OpenapiDataType {
        OpenapiDataType::Integer(IntegerFormat::Int64)
    }

    pub fn new_float_type() -> OpenapiDataType {
        OpenapiDataType::Number(NumberFormat::Float)
    }

    pub fn new_double_type() -> OpenapiDataType {
        OpenapiDataType::Number(NumberFormat::Double)
    }
}

impl fmt::Display for OpenapiDataType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", format!("{:?}", self).to_lowercase())
            .expect("Panicked writing OpenapiDataType");
        Ok(())
    }
}
