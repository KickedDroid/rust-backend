#![feature(decl_macro, proc_macro_hygiene)]

extern crate rocket;
extern crate reqwest;

use rocket::http::RawStr;
use rocket::*;

use std::time::Duration;
use reqwest::blocking::ClientBuilder;
use sha3::{Digest, Sha3_384};

#[get("/")]
fn index() -> &'static str {
    "Navigate to http://localhost:8000/check/<GitHub username>"
}

#[get("/check/<user>")]
fn check(user: &RawStr) -> Result<String, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.github.com/users/{}", user);
    
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        Ok(format!("{} is a user!", user))
    } else {
        Ok(format!("{} is not a user!", user))
    }
}

#[post("/api/withdrawl/<address>/<amount>")]
fn withdrawl(address: &RawStr, amount: &RawStr) -> Result<String, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.aquanow.io/accounts/v1/transaction");
    let _parsed_amount = amount.parse::<f32>().unwrap();
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let api_key = "";

    // hash the api key 
    let mut hasher = Sha3_384::new();
    hasher.update(api_key);
    let _sig = hasher.finalize();
    let res = client.post(request_url)
        .body("")
        .header("x-api-key", "")
        .header("x-signature", "")
        .header("x-nonce", "")
        .send()?;

    if res.status().is_success() {
        Ok(format!("Succes"))
    } else {
        Ok(format!("Error"))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, check, withdrawl]).launch();
}