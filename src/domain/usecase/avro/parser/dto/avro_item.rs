use std::fmt;

use crate::domain::usecase::avro::parser::dto::avro_item_type::AvroItemType;

#[derive(Debug, PartialEq)]
pub struct AvroItem {
    name: Option<String>,
    namespace: Option<String>,
    doc: Option<String>,
    item_type: AvroItemType,
    symbols: Option<Vec<String>>,
    default: Option<String>,
    fields: Option<Vec<AvroItem>>,
}

impl AvroItem {
    pub fn new(
        name: Option<String>,
        namespace: Option<String>,
        doc: Option<String>,
        item_type: AvroItemType,
        symbols: Option<Vec<String>>,
        default: Option<String>,
        fields: Option<Vec<AvroItem>>,
    ) -> AvroItem {
        AvroItem {
            name,
            namespace,
            doc,
            item_type,
            symbols,
            default,
            fields,
        }
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn get_namespace(&self) -> Option<String> {
        self.namespace.clone()
    }
    pub fn get_doc(&self) -> Option<String> {
        self.doc.clone()
    }

    pub fn get_item_type(&self) -> &AvroItemType {
        &self.item_type
    }

    pub fn get_symbols(&self) -> Option<Vec<String>> {
        self.symbols.clone()
    }

    pub fn get_default(&self) -> Option<String> {
        self.default.clone()
    }

    pub fn get_fields(&self) -> &Option<Vec<AvroItem>> {
        &self.fields
    }

    pub(crate) fn is_just_type(&self) -> bool {
        self.name.is_none()
            && self.namespace.is_none()
            && self.doc.is_none()
            && self.symbols.is_none()
            && self.default.is_none()
            && self.fields.is_none()
    }
}

impl fmt::Display for AvroItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
