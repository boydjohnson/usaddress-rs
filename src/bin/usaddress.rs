use crfs::{Attribute, Model};
use regex::Regex;
use std::borrow::Cow;
use std::collections::BTreeSet;
use usaddress_rs::tokenize;
use usaddress_rs::ADDRESS_PARSER_DATA;

lazy_static::lazy_static! {
    static ref TRAILING_ZEROS: Regex = Regex::new("(0+)$").unwrap();
}

lazy_static::lazy_static! {
    static ref UNICODE: Regex = Regex::new(r"(^[\W]*)|([^.\w]*$)").unwrap();
}

lazy_static::lazy_static! {
    static ref PUNCT: Regex = Regex::new("[.]").unwrap();
}

lazy_static::lazy_static! {
    static ref ENDS_IN_PUNCT: Regex = Regex::new(r".+[^.\w]").unwrap();
}

lazy_static::lazy_static! {
    static ref DIRECTIONS: BTreeSet<&'static str> = ["n", "s", "e", "w",
    "ne", "nw", "se", "sw",
    "north", "south", "east", "west",
    "northeast", "northwest", "southeast", "southwest"].iter().map(|c| *c).collect();
}

lazy_static::lazy_static! {
static ref STREET_NAMES: BTreeSet<&'static str> = [
    "allee",
    "alley",
    "ally",
    "aly",
    "anex",
    "annex",
    "annx",
    "anx",
    "arc",
    "arcade",
    "av",
    "ave",
    "aven",
    "avenu",
    "avenue",
    "avn",
    "avnue",
    "bayoo",
    "bayou",
    "bch",
    "beach",
    "bend",
    "bg",
    "bgs",
    "bl",
    "blf",
    "blfs",
    "bluf",
    "bluff",
    "bluffs",
    "blvd",
    "bnd",
    "bot",
    "bottm",
    "bottom",
    "boul",
    "boulevard",
    "boulv",
    "br",
    "branch",
    "brdge",
    "brg",
    "bridge",
    "brk",
    "brks",
    "brnch",
    "brook",
    "brooks",
    "btm",
    "burg",
    "burgs",
    "byp",
    "bypa",
    "bypas",
    "bypass",
    "byps",
    "byu",
    "camp",
    "canyn",
    "canyon",
    "cape",
    "causeway",
    "causwa",
    "causway",
    "cen",
    "cent",
    "center",
    "centers",
    "centr",
    "centre",
    "ci",
    "cir",
    "circ",
    "circl",
    "circle",
    "circles",
    "cirs",
    "ck",
    "clb",
    "clf",
    "clfs",
    "cliff",
    "cliffs",
    "club",
    "cmn",
    "cmns",
    "cmp",
    "cnter",
    "cntr",
    "cnyn",
    "common",
    "commons",
    "cor",
    "corner",
    "corners",
    "cors",
    "course",
    "court",
    "courts",
    "cove",
    "coves",
    "cp",
    "cpe",
    "cr",
    "crcl",
    "crcle",
    "crecent",
    "creek",
    "cres",
    "crescent",
    "cresent",
    "crest",
    "crk",
    "crossing",
    "crossroad",
    "crossroads",
    "crscnt",
    "crse",
    "crsent",
    "crsnt",
    "crssing",
    "crssng",
    "crst",
    "crt",
    "cswy",
    "ct",
    "ctr",
    "ctrs",
    "cts",
    "curv",
    "curve",
    "cv",
    "cvs",
    "cyn",
    "dale",
    "dam",
    "div",
    "divide",
    "dl",
    "dm",
    "dr",
    "driv",
    "drive",
    "drives",
    "drs",
    "drv",
    "dv",
    "dvd",
    "est",
    "estate",
    "estates",
    "ests",
    "ex",
    "exp",
    "expr",
    "express",
    "expressway",
    "expw",
    "expy",
    "ext",
    "extension",
    "extensions",
    "extn",
    "extnsn",
    "exts",
    "fall",
    "falls",
    "ferry",
    "field",
    "fields",
    "flat",
    "flats",
    "fld",
    "flds",
    "fls",
    "flt",
    "flts",
    "ford",
    "fords",
    "forest",
    "forests",
    "forg",
    "forge",
    "forges",
    "fork",
    "forks",
    "fort",
    "frd",
    "frds",
    "freeway",
    "freewy",
    "frg",
    "frgs",
    "frk",
    "frks",
    "frry",
    "frst",
    "frt",
    "frway",
    "frwy",
    "fry",
    "ft",
    "fwy",
    "garden",
    "gardens",
    "gardn",
    "gateway",
    "gatewy",
    "gatway",
    "gdn",
    "gdns",
    "glen",
    "glens",
    "gln",
    "glns",
    "grden",
    "grdn",
    "grdns",
    "green",
    "greens",
    "grn",
    "grns",
    "grov",
    "grove",
    "groves",
    "grv",
    "grvs",
    "gtway",
    "gtwy",
    "harb",
    "harbor",
    "harbors",
    "harbr",
    "haven",
    "havn",
    "hbr",
    "hbrs",
    "height",
    "heights",
    "hgts",
    "highway",
    "highwy",
    "hill",
    "hills",
    "hiway",
    "hiwy",
    "hl",
    "hllw",
    "hls",
    "hollow",
    "hollows",
    "holw",
    "holws",
    "hrbor",
    "ht",
    "hts",
    "hvn",
    "hway",
    "hwy",
    "inlet",
    "inlt",
    "is",
    "island",
    "islands",
    "isle",
    "isles",
    "islnd",
    "islnds",
    "iss",
    "jct",
    "jction",
    "jctn",
    "jctns",
    "jcts",
    "junction",
    "junctions",
    "junctn",
    "juncton",
    "key",
    "keys",
    "knl",
    "knls",
    "knol",
    "knoll",
    "knolls",
    "ky",
    "kys",
    "la",
    "lake",
    "lakes",
    "land",
    "landing",
    "lane",
    "lanes",
    "lck",
    "lcks",
    "ldg",
    "ldge",
    "lf",
    "lgt",
    "lgts",
    "light",
    "lights",
    "lk",
    "lks",
    "ln",
    "lndg",
    "lndng",
    "loaf",
    "lock",
    "locks",
    "lodg",
    "lodge",
    "loop",
    "loops",
    "lp",
    "mall",
    "manor",
    "manors",
    "mdw",
    "mdws",
    "meadow",
    "meadows",
    "medows",
    "mews",
    "mi",
    "mile",
    "mill",
    "mills",
    "mission",
    "missn",
    "ml",
    "mls",
    "mn",
    "mnr",
    "mnrs",
    "mnt",
    "mntain",
    "mntn",
    "mntns",
    "motorway",
    "mount",
    "mountain",
    "mountains",
    "mountin",
    "msn",
    "mssn",
    "mt",
    "mtin",
    "mtn",
    "mtns",
    "mtwy",
    "nck",
    "neck",
    "opas",
    "orch",
    "orchard",
    "orchrd",
    "oval",
    "overlook",
    "overpass",
    "ovl",
    "ovlk",
    "park",
    "parks",
    "parkway",
    "parkways",
    "parkwy",
    "pass",
    "passage",
    "path",
    "paths",
    "pike",
    "pikes",
    "pine",
    "pines",
    "pk",
    "pkway",
    "pkwy",
    "pkwys",
    "pky",
    "pl",
    "place",
    "plain",
    "plaines",
    "plains",
    "plaza",
    "pln",
    "plns",
    "plz",
    "plza",
    "pne",
    "pnes",
    "point",
    "points",
    "port",
    "ports",
    "pr",
    "prairie",
    "prarie",
    "prk",
    "prr",
    "prt",
    "prts",
    "psge",
    "pt",
    "pts",
    "pw",
    "pwy",
    "rad",
    "radial",
    "radiel",
    "radl",
    "ramp",
    "ranch",
    "ranches",
    "rapid",
    "rapids",
    "rd",
    "rdg",
    "rdge",
    "rdgs",
    "rds",
    "rest",
    "ri",
    "ridge",
    "ridges",
    "rise",
    "riv",
    "river",
    "rivr",
    "rn",
    "rnch",
    "rnchs",
    "road",
    "roads",
    "route",
    "row",
    "rpd",
    "rpds",
    "rst",
    "rte",
    "rue",
    "run",
    "rvr",
    "shl",
    "shls",
    "shoal",
    "shoals",
    "shoar",
    "shoars",
    "shore",
    "shores",
    "shr",
    "shrs",
    "skwy",
    "skyway",
    "smt",
    "spg",
    "spgs",
    "spng",
    "spngs",
    "spring",
    "springs",
    "sprng",
    "sprngs",
    "spur",
    "spurs",
    "sq",
    "sqr",
    "sqre",
    "sqrs",
    "sqs",
    "squ",
    "square",
    "squares",
    "st",
    "sta",
    "station",
    "statn",
    "stn",
    "str",
    "stra",
    "strav",
    "strave",
    "straven",
    "stravenue",
    "stravn",
    "stream",
    "street",
    "streets",
    "streme",
    "strm",
    "strt",
    "strvn",
    "strvnue",
    "sts",
    "sumit",
    "sumitt",
    "summit",
    "te",
    "ter",
    "terr",
    "terrace",
    "throughway",
    "tl",
    "tpk",
    "tpke",
    "tr",
    "trace",
    "traces",
    "track",
    "tracks",
    "trafficway",
    "trail",
    "trailer",
    "trails",
    "trak",
    "trce",
    "trfy",
    "trk",
    "trks",
    "trl",
    "trlr",
    "trlrs",
    "trls",
    "trnpk",
    "trpk",
    "trwy",
    "tunel",
    "tunl",
    "tunls",
    "tunnel",
    "tunnels",
    "tunnl",
    "turn",
    "turnpike",
    "turnpk",
    "un",
    "underpass",
    "union",
    "unions",
    "uns",
    "upas",
    "valley",
    "valleys",
    "vally",
    "vdct",
    "via",
    "viadct",
    "viaduct",
    "view",
    "views",
    "vill",
    "villag",
    "village",
    "villages",
    "ville",
    "villg",
    "villiage",
    "vis",
    "vist",
    "vista",
    "vl",
    "vlg",
    "vlgs",
    "vlly",
    "vly",
    "vlys",
    "vst",
    "vsta",
    "vw",
    "vws",
    "walk",
    "walks",
    "wall",
    "way",
    "ways",
    "well",
    "wells",
    "wl",
    "wls",
    "wy",
    "xc",
    "xg",
    "xing",
    "xrd",
    "xrds",
].iter().map(|c| *c).collect();
}

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
    let model = Model::new(ADDRESS_PARSER_DATA).unwrap();

    if let Some(address) = std::env::args().skip(1).next() {
        println!("{:?}", address);
        let tokens = tokenize(&address);
        println!("tokens: {:?}", tokens);
        let tok = tokens.clone();

        let mut tagger = model.tagger().unwrap();

        let features = tokens_to_features(tokens);
        for t in &features {
            println!("{:?}", t);
        }

        let val = tagger.tag(&&features).unwrap();

        for (tok, label) in tok.iter().zip(val.iter()) {
            println!("{}: {}", tok, label);
        }
    }
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
    let token_clean = match token {
        "&" => Cow::from(token),
        "#" => Cow::from(token),
        "½" => Cow::from(token),
        _ => UNICODE.replace_all(token, ""),
    };

    let lower = token_clean.to_lowercase();

    let token_abbrev = PUNCT.replace_all(lower.as_str(), "");

    let abbrev = if token_clean.ends_with('.') { 1.0 } else { 0.0 };

    let mut digits_str = String::from("digits:");
    digits_str.push_str(digits(token_clean.as_ref()));

    let word = if token_abbrev.chars().any(|c| !c.is_ascii_digit()) {
        (format!("word:{}", token_abbrev), 1.0)
    } else {
        ("word".to_string(), 0.0)
    };

    let trailing = if token_abbrev.chars().all(|c| c.is_ascii_digit()) {
        (
            format!("trailing.zeros:{}", trailing_zeros(&token_abbrev)),
            1.0,
        )
    } else {
        ("trailing.zeros".to_string(), 0.0)
    };

    let length = if token_abbrev.chars().all(|c| c.is_ascii_digit()) {
        (format!("length:d:{}", token_abbrev.len()), 1.0)
    } else {
        (format!("length:w:{}", token_abbrev.len()), 1.0)
    };

    let endsinpunc = if ENDS_IN_PUNCT.is_match(token) {
        if let Some(l) = token.chars().rev().next() {
            (format!("endsinpunc:{}", l), 1.0)
        } else {
            ("endsinpunc".to_string(), 0.0)
        }
    } else {
        ("endsinpunc".to_string(), 0.0)
    };

    let directional = if DIRECTIONS.contains(token_abbrev.as_ref()) {
        1.0
    } else {
        0.0
    };

    let street_name = if STREET_NAMES.contains(token_abbrev.as_ref()) {
        1.0
    } else {
        0.0
    };

    let has_vowels = if token_abbrev
        .chars()
        .skip(1)
        .any(|c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
    {
        1.0
    } else {
        0.0
    };

    vec![
        Attribute::new("abbrev", abbrev),
        Attribute::new(digits_str, 1.0),
        Attribute::new(word.0, word.1),
        Attribute::new(trailing.0, trailing.1),
        Attribute::new(length.0, length.1),
        Attribute::new(endsinpunc.0, endsinpunc.1),
        Attribute::new("directional", directional),
        Attribute::new("street_name", street_name),
        Attribute::new("has.vowels", has_vowels),
    ]
}

fn tokens_to_features(address: Vec<String>) -> Vec<Vec<Attribute>> {
    let (mut feat, mut prev) = if let Some(first) = address.first() {
        let seq = token_features(&first);
        (Some(vec![seq.clone()]), Some(seq))
    } else {
        (None, None)
    };

    for token in address.into_iter().skip(1) {
        let mut token_feat = token_features(&token);
        let mut current = token_feat.clone();
        let cur = current.clone();

        if let Some(last) = feat.as_mut().map(|f| f.last_mut()).flatten() {
            prepend(&mut current, "next");
            last.append(&mut current);
        }

        if let Some(p) = &mut prev {
            prepend(p, "previous");
            token_feat.append(p);
        }

        prev = Some(cur);

        if let Some(f) = &mut feat {
            f.push(token_feat);
        }
    }

    if let Some(first) = feat.as_mut().map(|f| f.first_mut()).flatten() {
        first.push(Attribute::new("address.start", 1.0));
    }

    if let Some(last) = feat.as_mut().map(|f| f.last_mut()).flatten() {
        last.push(Attribute::new("address.end", 1.0))
    }

    if let Some(second) = feat.as_mut().map(|f| f.iter_mut().skip(1).next()).flatten() {
        second.push(Attribute::new("previous:address.start", 1.0));
    }

    if let Some(second_to_last) = feat
        .as_mut()
        .map(|f| f.iter_mut().rev().skip(1).next())
        .flatten()
    {
        second_to_last.push(Attribute::new("next:address.end", 1.0));
    }

    feat.unwrap_or_default()
}

fn prepend(seq: &mut Vec<Attribute>, prefix: &str) {
    for item in seq {
        item.name = format!("{}:{}", prefix, item.name);
    }
}
