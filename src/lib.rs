pub const ADDRESS_PARSER_DATA: &[u8] = include_bytes!("../usaddr.crfsuite");
mod error;
mod features;
mod tokenize;

use features::tokens_to_features;
pub use tokenize::tokenize;

use crfs::Model;

lazy_static::lazy_static! {
    pub static ref MODEL: Model<'static> = {
        let model = Model::new(ADDRESS_PARSER_DATA);
        model.expect("Model failed to load")
    };
}

pub fn parse(address: &str) -> Result<Vec<(String, String)>, crate::error::Error> {
    let tokens = tokenize(address);
    let tok = tokens.clone();

    let mut tagger = MODEL.tagger()?;

    let features = tokens_to_features(tokens);

    let labels = tagger.tag(&features)?;

    Ok(tok
        .into_iter()
        .zip(labels.iter().map(|s| s.to_string()))
        .collect())
}
