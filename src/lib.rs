pub const ADDRESS_PARSER_DATA: &'static [u8] = include_bytes!("../usaddr.crfsuite");
mod tokenize;

pub use tokenize::tokenize;
