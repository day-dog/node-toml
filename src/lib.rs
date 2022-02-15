#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;


#[napi]
fn parse(input: String) -> serde_json::Value {
    let val: toml::Value = toml::from_str(&*input).unwrap();
    let json_str = serde_json::to_string_pretty(&val).unwrap();
    serde_json::from_str(json_str.as_str()).unwrap()
}
