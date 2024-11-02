use anyhow::Result;
use reqwest::{
    blocking::{Client as RwClient, Response}, Method
};

pub fn get_crate_data(crate_name: &str, cargo_ver: &str) -> Result<Response> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    println!("request {}", url);
    let req_builder = RwClient::new()
        .request(Method::GET, url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", cargo_ver);
    Ok(req_builder.send()?)
}
