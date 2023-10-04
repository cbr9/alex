use std::{fs::File, io::Cursor};

use scraper::{Html, Selector};
use url::Url;

use crate::CLIENT;

use super::Journal;

pub struct Arxiv {
    id: String,
    url: Url,
    html: Html,
}

impl Arxiv {
    pub fn from_id(id: &str) -> Self {
        let url = format!("https://arxiv.org/abs/{}", id);
        let response = CLIENT.to_owned().get(&url).send().unwrap();
        let html = response.text().unwrap();
        let document = Html::parse_document(&html);

        let url = Url::parse(&url).unwrap();

        Self {
            url,
            id: id.to_string(),
            html: document,
        }
    }
}

impl Journal for Arxiv {
    fn title(&self) -> Option<String> {
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
        let mut pdf_url = self.url.clone();
        let pdf_id = self.id.clone() + ".pdf";
        let path = ["pdf", &pdf_id];

        pdf_url.set_path(&path.join("/"));

        let response = CLIENT.to_owned().get(pdf_url.as_str()).send().unwrap();
        let mut file = File::create(pdf_id).unwrap();
        let mut content = Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();

        Some(file)
    }
}
