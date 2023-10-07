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
  q_bittorrent.login(
    &Env::get::<String>("Q_BITTORRENT_USER", "admin"),
    &Env::get::<String>("Q_BITTORRENT_PASS", "adminadmin"),
  ).await;

  let torrent_sniff = TorrentSniff::new(
    &Env::get::<String>("TORRENT_SNIFF_HOST", "192.168.3.10"),
    Env::get::<u16>("TORRENT_SNIFF_PORT", "8090"),
  );
  let torrents = torrent_sniff.search(TEMP_SEARCH_TERM).await;

  match torrents.get(0) {
    Some(torrent) => {
      match torrent.magnet_links.get(0) {
        Some(magnet_link) => q_bittorrent
          .add_torrent(&torrent.name, &magnet_link.link).await,
        None => panic!("No magnet link found for torrent: {}", torrent.name),
      }
    },
    None => panic!("No torrent found!"),
  }
}
