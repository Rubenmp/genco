use crate::domain::usecase::avro::parser::dto::avro_item::AvroItem;

#[derive(Debug, PartialEq)]
pub enum AvroItemType {
    Enum,
    RecordSimple,
    Record(Box<AvroItem>),
    RecordName(String),
    ArraySimple, // TODO: remove this redundant type
    Array(Vec<AvroItemType>),
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
