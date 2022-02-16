#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Error, Result, tokio};


// parse sync method for node.js
// example
//  `
// [test]
// foo = "bar"
// `
// to { test: { foo: 'bar' } }
#[napi]
fn parse_sync(input: String) -> Result<serde_json::Value> {
    let res = toml::from_str(&*input).map_err(|err| Error::from_reason(err.to_string()))?;
    Ok(res)
}


// parse async method for node.js
// example
//  `
// [test]
// foo = "bar"
// `
// to { test: { foo: 'bar' } }
#[napi]
async fn parse(input: String) -> Result<serde_json::Value> {
    tokio::task::spawn(async move {
        let res = toml::from_str(&*input).map_err(|err| Error::from_reason(err.to_string()))?;
        Ok(res)
    }).await.unwrap()
}



