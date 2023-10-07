use tokio;
use iron_nose::q_bittorrent::QBittorrent;
use iron_nose::torrent_sniff::TorrentSniff;

#[tokio::main]
async fn main() {
  let mut q_bittorrent = QBittorrent::new("192.168.3.10", 8080);
  q_bittorrent.login("admin", "adminadmin").await;

  let torrent_sniff = TorrentSniff::new("192.168.3.10", 8090);
  let torrents = torrent_sniff.search("Indiana Jones").await;

  println!("Torrents: {:#?}", torrents);

  match torrents.get(0) {
    Some(torrent) => {
      println!("Torrent: {:#?}", torrent);
      match torrent.magnet_links.get(0) {
        Some(magnet_link) => q_bittorrent
          .add_torrent(&torrent.name, &magnet_link.link).await,
        None => panic!("No magnet link found for torrent: {}", torrent.name),
      }
    },
    None => panic!("No torrent found!"),
  }
}
