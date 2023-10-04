use std::{
    fs::File,
    io::Cursor,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use scraper::{Html, Selector};
use tempfile::Builder;
use url::{ParseError, Url};

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

enum Journal {
    Arxiv,
}
struct JournalEntry {
    url: Url,
    html: Html,
}

impl JournalEntry {
    fn from_url(url: &str) -> Self {
        let response = CLIENT.to_owned().get(url).send().unwrap();
        let html = response.text().unwrap();
        let document = Html::parse_document(&html);

        let url = Url::parse(url).unwrap();

        Self {
            url,
            html: document,
        }
    }

    fn get_title(&self) -> Option<String> {
        self.url.host_str().and_then(|host| match host {
            "arxiv.org" => {
                let selector = Selector::parse("h1.title").unwrap();
                let title = self.html.select(&selector).next().unwrap();

                let text = title.text().map(String::from).collect::<Vec<_>>();
                return text.get(1).cloned();
            }
            _ => None,
        })
    }

    fn download_pdf(&self) -> Option<File> {
        self.url.host_str().and_then(|host| match host {
            "arxiv.org" => {
                let arxiv_id = self
                    .url
                    .path_segments()
                    .map(|c| c.collect::<Vec<_>>())
                    .unwrap()
                    .last()
                    .unwrap()
                    .to_string();

                let mut pdf_url = self.url.clone();
                let pdf_id = arxiv_id.clone() + ".pdf";
                let path = ["pdf", &pdf_id];

                pdf_url.set_path(&path.join("/"));

                let response = CLIENT.to_owned().get(pdf_url.as_str()).send().unwrap();
                let mut file = File::create(pdf_id).unwrap();
                let mut content = Cursor::new(response.bytes().unwrap());
                std::io::copy(&mut content, &mut file).unwrap();

                Some(file)
            }
            _ => None,
        })
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Download {},
}

fn main() {
    let doc = JournalEntry::from_url("https://arxiv.org/abs/1606.06864");
    println!("{:?}", doc.get_title());
    doc.download_pdf();
}
