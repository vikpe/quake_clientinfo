# quake_clientinfo [![Test](https://github.com/vikpe/quake_clientinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_clientinfo/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_clientinfo)](https://crates.io/crates/quake_clientinfo) [![docs.rs](https://img.shields.io/docsrs/quake_clientinfo)](https://docs.rs/quake_clientinfo/)

> Parse QuakeWorld clientinfo strings

## Usage

```rust
use quake_clientinfo::Clientinfo;

let info = Clientinfo::from(r#"\team\red\name\Alpha\*spectator\1"#);
assert_eq!(info.name, Some("Alpha".to_string()));
assert_eq!(info.team, Some("red".to_string()));
assert_eq!(info.spectator, Some(1));
assert_eq!(info.topcolor, None);
```

## Fields

```rust
pub struct Clientinfo {
    pub name: Option<String>,
    pub team: Option<String>,
    pub topcolor: Option<i32>,
    pub bottomcolor: Option<i32>,
    pub spectator: Option<i32>,
    pub client: Option<String>,
}
```

## See also

* [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) - Parse QuakeWorld serverinfo strings
* [quake_infostring](https://github.com/vikpe/quake_infostring) - Parse QuakeWorld info strings
