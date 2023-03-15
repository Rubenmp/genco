use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JsonNodeType {
    Document,
    Object,
    LBrace, // {
    RBrace, // }
    LBracket, // [
    RBracket, // ]
    Pair, // "name": "John"
    String,
    StringContent,
    QuotationMark, // "
    Number,
    Comma,
    Colon, // :
    Array,
    Null,
    EscapeSequence,
}

impl FromStr for JsonNodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<JsonNodeType, ()> {
        match input {
            "document" => Ok(JsonNodeType::Document),
            "object" => Ok(JsonNodeType::Object),
            "{" => Ok(JsonNodeType::LBrace),
            "}" => Ok(JsonNodeType::RBrace),
            "[" => Ok(JsonNodeType::LBracket),
            "]" => Ok(JsonNodeType::RBracket),
            "pair" => Ok(JsonNodeType::Pair),
            "string" => Ok(JsonNodeType::String),
            "string_content" => Ok(JsonNodeType::StringContent),
            "\"" => Ok(JsonNodeType::QuotationMark),
            "number" => Ok(JsonNodeType::Number),
            "," => Ok(JsonNodeType::Comma),
            ":" => Ok(JsonNodeType::Colon),
            "array" => Ok(JsonNodeType::Array),
            "null" => Ok(JsonNodeType::Null),
            "escape_sequence" => Ok(JsonNodeType::EscapeSequence),

            _ => Err(())
        }
    }
}


impl fmt::Display for JsonNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}