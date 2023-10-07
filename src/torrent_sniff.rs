use serde::Deserialize;
use serde_json;

use crate::common::http_client::{HttpClient, HttpMethod};

#[derive(Deserialize, Debug)]
pub struct MagnetLink {
  pub id: String,
  pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct Torrent {
  pub id: String,
  pub name: String,
  pub provider: String,
  pub magnet_links: Vec<MagnetLink>,
}

pub struct TorrentSniff {
  client: HttpClient,
}

impl TorrentSniff {
  pub fn new(host: &str, port: u16) -> Self {
    return Self {
      client: HttpClient::new(&format!("http://{}:{}", host, port)),
    }
  }

  pub async fn search(self, term: &str) -> Vec<Torrent> {
    let path = format!("/json?search={}", HttpClient::format(term));
    match self.client.req(HttpMethod::GET, &path, None, None).await {
      Ok(response) => {
        let text = response.text().await.unwrap_or(String::from("[]"));
        match serde_json::from_str::<Vec<Torrent>>(&text) {
          Ok(data) => data,
          Err(error) => panic!("Failed to deserialize result: {}", error),
        }
      },
      Err(error) => panic!("Failed to search from torrent sniff: {}", error),
    }
  }
}
