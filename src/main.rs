use tokio;
use iron_nose::common::env::Env;
use iron_nose::q_bittorrent::QBittorrent;
use iron_nose::torrent_sniff::TorrentSniff;

const TEMP_SEARCH_TERM: &str = "Indiana Jones";

#[tokio::main]
async fn main() {
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

  let torrent_sniff = TorrentSniff::new(
    &Env::get::<String>("TORRENT_SNIFF_HOST", "192.168.3.10"),
    Env::get::<u16>("TORRENT_SNIFF_PORT", "8090"),
  );

  handle_download(&q_bittorrent, &torrent_sniff, TEMP_SEARCH_TERM.to_string())
    .await;
}

async fn handle_download(
  q_bittorrent: &QBittorrent,
  torrent_sniff: &TorrentSniff,
  search_term: String,
) {
  let torrents = torrent_sniff.search(&search_term).await;

  for torrent in torrents {
    match torrent.magnet_links.get(0) {
      Some(magnet_link) => {
        q_bittorrent.add_torrent(&torrent.name, &magnet_link.link).await;
        break;
      },
      None => println!("No magnet link found for torrent: {}", torrent.name),
    }
  }
}
