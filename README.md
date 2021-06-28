# Rust port of [usaddress](https://github.com/datamade/usaddress)

## Install

```
cargo install usaddress
```
or

```
[dependencies]
usaddress = "0.1"
```

## Contains a binary that can tag an address

```bash
usaddress "170th St and Broadway Ave New York, NY 10033"

<AddressString><AddressNumber>170th</AddressNumber><StreetName>St</StreetName><StreetName>and</StreetName><StreetName>Broadway</StreetName><StreetNamePostType>Ave</StreetNamePostType><PlaceName>New</PlaceName><PlaceName>York,</PlaceName><StateName>NY</StateName><ZipCode>10033</ZipCode></AddressString>
```

or usage of the library

```rust

match usaddress::parse("170th St and Broadway Ave New York, NY 10033") {
    Ok(tagged_addresses) => {
        ...
    },
    Err(e) => {
        ...
    }
}
```
