use core::num;

use crfs::{Attribute, Model};
use regex::Regex;
use usaddress_rs::ADDRESS_PARSER_DATA;

lazy_static::lazy_static! {
    static ref TRAILING_ZEROS: Regex = Regex::new("(0+)$").unwrap();
}

lazy_static::lazy_static! {
    static ref AMPERSAND: Regex = Regex::new("(&#38;)|(&amp;)").unwrap();
}

lazy_static::lazy_static! {
    static ref UNICODE: Regex = Regex::new("^[\W]*)|(^.\w]*$)").unwrap();
}

lazy_static::lazy_static! {
    static ref PUNCT: Regex = Regex::new("[.]").unwrap();
}

fn main() {
    let model = Model::new(ADDRESS_PARSER_DATA).unwrap();

    let labels = [
        "AddressNumber",
        "StreetNamePreDirectional",
        "StreetName",
        "StreetNamePostType",
        "OccupancyIdentifier",
        "OccupancyType",
        "StreetNamePreType",
        "PlaceName",
        "ZipCode",
        "StateName",
        "LandmarkName",
        "USPSBoxType",
        "USPSBoxID",
        "StreetNamePostDirectional",
        "AddressNumberSuffix",
        "USPSBoxGroupID",
        "USPSBoxGroupType",
        "SubaddressIdentifier",
        "SubaddressType",
        "Recipient",
        "StreetNamePreModifier",
        "BuildingName",
        "AddressNumberPrefix",
        "IntersectionSeparator",
        "CornerOf",
        "NotAddress",
    ];

}


fn trailing_zeros(token: &str) -> &str {
    let mut results = TRAILING_ZEROS.find_iter(token);
    results.next().map(|m| m.as_str()).unwrap_or("")

}

fn digits(token: &str) -> &str {
    let len = token.len();
    let num_digits = token.chars().filter(|c| c.is_ascii_digit()).count();
    
    if num_digits == len {
        "all_digits"
    } else if num_digits > 0 {
        "some_digits"
    } else {
        "no_digits"
    }
}

fn token_features(token: &str) -> Vec<Attribute> {
    let token_clean = 
        match token {
            "&" => token,
            "#" => token,
            "½" => token,
            _ => &UNICODE.replace_all(token, "")
        };
    
    let token_abbrev = &PUNCT.replace_all(&token_clean.to_lowercase(), "");

    let abbrev = ;

    vec![
        Attribute::new("abbrev")
    ]
}
