use std::fmt;

use crate::domain::usecase::avro::parser::dto::avro_item::AvroItem;

#[derive(Debug, PartialEq)]
pub enum AvroItemType {
    Enum,
    RecordSimple,
    Record(Box<AvroItem>),
    RecordName(String),
    Array(Vec<AvroItemType>),
    ArrayItems(Box<AvroItemType>),
    Null,
    Int,
    Long,
    Float,
    Double,
    String,
    Bytes,
    Boolean,
    Map,
}

impl fmt::Display for AvroItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
