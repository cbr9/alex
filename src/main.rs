use lazy_static::lazy_static;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};

use crate::journals::{arxiv::Arxiv, Journal};

mod journals;

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("alex"));
        headers
    };
    static ref CLIENT: Client = reqwest::blocking::Client::builder()
        .default_headers(HEADERS.to_owned())
        .build()
        .unwrap();
}

fn main() {
    let doc = Arxiv::from_id("1606.06864");
    println!("{:?}", doc.title());
    doc.download_pdf();
}
