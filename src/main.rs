use std::fs;

use env_logger;
use log::{info, warn};
use serde_json;
use tokio;

use iron_nose::common::env::Env;
use iron_nose::q_bittorrent::QBittorrent;
use iron_nose::torrent_sniff::TorrentSniff;

const SEARCH_TERMS_FILE: &str = "terms.json";

#[tokio::main]
async fn main() {
  env_logger::init();

  info!(target: "QBittorrent", "Connecting to client...");
  let mut q_bittorrent = QBittorrent::new(
    &Env::get::<String>("Q_BITTORRENT_HOST", "192.168.3.10"),
    Env::get::<u16>("Q_BITTORRENT_PORT", "8080"),
  );
  q_bittorrent
    .login(
      &Env::get::<String>("Q_BITTORRENT_USER", "admin"),
      &Env::get::<String>("Q_BITTORRENT_PASS", "adminadmin"),
    )
    .await;
  info!(target: "QBittorrent", "Connected and logged in!");

  let torrent_sniff = TorrentSniff::new(
    &Env::get::<String>("TORRENT_SNIFF_HOST", "192.168.3.10"),
    Env::get::<u16>("TORRENT_SNIFF_PORT", "8090"),
  );


  match fs::read_to_string(SEARCH_TERMS_FILE) {
    Ok(content) => {
      let data = serde_json::from_str::<Vec<String>>(&content)
        .unwrap_or(Vec::new());

      for item in data {
        handle_download(&q_bittorrent, &torrent_sniff, item)
          .await;
      }
    },
    Err(error) => panic!("Failed to read file: {}", error),
  }
}

async fn handle_download(
  q_bittorrent: &QBittorrent,
  torrent_sniff: &TorrentSniff,
  search_term: String,
) {
  info!(target: "TorrentSniff", "Searching for: {}...", search_term);
  let torrents = torrent_sniff.search(&search_term).await;
  info!(target: "TorrentSniff", "Search completed, found {} results!", torrents.len());

  for torrent in torrents {
    match torrent.magnet_links.get(0) {
      Some(magnet_link) => {
        info!(target: "QBittorrent", "Adding {} for download...", torrent.name);
        q_bittorrent.add_torrent(&torrent.name, &magnet_link.link).await;
        info!(target: "QBittorrent", "Successfully added {}", torrent.name);
        break;
      },
      None => warn!("No magnet link found for torrent: {}", torrent.name),
    }
  }
}
