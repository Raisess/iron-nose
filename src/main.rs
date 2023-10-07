use tokio;
use iron_nose::q_bittorrent::QBittorrent;
use iron_nose::torrent_sniff::TorrentSniff;

#[tokio::main]
async fn main() {
  let torrent_sniff = TorrentSniff::new("192.168.3.10", 8090);
  torrent_sniff.search("Indiana Jones").await;

  let mut q_bittorrent = QBittorrent::new("192.168.3.10", 8080);
  q_bittorrent.login("admin", "adminadmin").await;
  //q_bittorrent.add_torrent("Indiana Jones", "magnet:?xt=urn:btih:9A82E7C94DA8DA5FEFE4230096CBCDE9CCC47FF8&dn=Indiana+Jones+and+the+Dial+of+Destiny+%282023%29+%5B1080p%5D+%5BYTS.MX%5D&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fopen.tracker.cl%3A1337%2Fannounce&tr=udp%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Ftracker.dler.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Fipv4.tracker.harry.lu%3A80%2Fannounce&tr=https%3A%2F%2Fopentracker.i2p.rocks%3A443%2Fannounce").await;
}
