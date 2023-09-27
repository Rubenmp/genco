use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum YamlNodeType {
    FlowNode,
    Hyphen, // -
    BlockMappingPair,
    BlockMapping,
    Document,
    VerticalBar,
    Stream,
    Comment,
    PlainScalar,
    BlockScalar,
    Colon, // :
    BlockNode,
    ThreeHyphen, // ---
    IntegerScalar,
    StringScalar,
    BooleanScalar,
    BlockSequence,
    BlockSequenceItem,
    Apostrophe, // '
    SingleQuoteScalar,
    QuotationMark, // "
    DoubleQuoteScalar,
}

impl FromStr for YamlNodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<YamlNodeType, ()> {
        match input {
            "flow_node" => Ok(YamlNodeType::FlowNode),
            "-" => Ok(YamlNodeType::Hyphen),
            "block_mapping_pair" => Ok(YamlNodeType::BlockMappingPair),
            "block_mapping" => Ok(YamlNodeType::BlockMapping),
            "document" => Ok(YamlNodeType::Document),
            "|" => Ok(YamlNodeType::VerticalBar),
            "stream" => Ok(YamlNodeType::Stream),
            "comment" => Ok(YamlNodeType::Comment),
            "plain_scalar" => Ok(YamlNodeType::PlainScalar),
            "block_scalar" => Ok(YamlNodeType::BlockScalar),
            ":" => Ok(YamlNodeType::Colon),
            "block_node" => Ok(YamlNodeType::BlockNode),
            "---" => Ok(YamlNodeType::ThreeHyphen),
            "integer_scalar" => Ok(YamlNodeType::IntegerScalar),
            "string_scalar" => Ok(YamlNodeType::StringScalar),
            "boolean_scalar" => Ok(YamlNodeType::BooleanScalar),
            "block_sequence" => Ok(YamlNodeType::BlockSequence),
            "block_sequence_item" => Ok(YamlNodeType::BlockSequenceItem),
            "'" => Ok(YamlNodeType::Apostrophe),
            "single_quote_scalar" => Ok(YamlNodeType::SingleQuoteScalar),
            "\"" => Ok(YamlNodeType::QuotationMark),
            "double_quote_scalar" => Ok(YamlNodeType::DoubleQuoteScalar),

            _ => Err(()),
        }
    }
}

impl fmt::Display for YamlNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
