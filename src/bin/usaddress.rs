use crfs::Model;

use std::io::{BufWriter, Write};
use usaddress::ADDRESS_PARSER_DATA;

const LABELS: &'static [&'static str] = &[
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

fn main() {
    let _model = Model::new(ADDRESS_PARSER_DATA).unwrap();
    let mut out = BufWriter::with_capacity(1_000, std::io::stdout());
    if let Some(address) = std::env::args().skip(1).next() {
        match usaddress::parse(&address) {
            Ok(labelled) => {
                write!(&mut out, "<AddressString>").expect("Unable to write to stdout");
                for (token, label) in labelled {
                    write!(&mut out, "<{}>{}</{}>", label, token, label)
                        .expect("Unable to write to stdout");
                }
                writeln!(&mut out, "</AddressString>").expect("Unable to write to stdout");
            }
            Err(e) => writeln!(std::io::stderr(), "{}", e).expect("Unable to write to stderr"),
        }
    }
}
