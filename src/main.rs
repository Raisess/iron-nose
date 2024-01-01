use core::time;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;

use tokio;

use iron_nose::common::env::Env;
use iron_nose::gateways::q_bittorrent::QBittorrent;
use iron_nose::gateways::torrent_sniff::TorrentSniff;

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

  loop {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
    println!("Iron Nose - Torrent Search");

    let search_term = prompt("===> Search: ");

    println!("Searching...");
    let torrents = torrent_sniff.search(&search_term).await;
    println!("Showing results...\n");

    let mut i: u8 = 0;
    for torrent in torrents.iter() {
      println!("{} - {}", i, torrent.name);
      println!("Provider: {}", torrent.provider);
      println!("Links:");
      let mut j: u8 = 0;
      for magnet_link in torrent.magnet_links.iter() {
        println!("\t{}: - {}", j, magnet_link.link);
        j += 1;
      }

      println!("\n-------------------------------\n");
      i += 1;
    }

    let target = prompt("===> Option [Movie/Link, e.g.: 1,2]: ");
    let [torrent_idx, magnet_link_idx] = target.trim()
      .split(",")
      .map(|i| i.trim().parse::<usize>().unwrap_or(0))
      .collect::<Vec<usize>>()[..2]
      else { panic!("Failed to retrive option") };

    match torrents.get(torrent_idx) {
      Some(torrent) => {
        match torrent.magnet_links.get(magnet_link_idx) {
          Some(magnet_link) => {
            println!("Adding {} to QBittorrent...", torrent.name);
            q_bittorrent.add_torrent(&torrent.name, &magnet_link.link).await;
          },
          None => println!("Magnet link not found for index: {}", magnet_link_idx),
        }
      },
      None => println!("Torrent not found for index: {}", torrent_idx),
    }

    sleep(time::Duration::from_secs(3));
  }
}

fn prompt(text: &str) -> String {
  print!("{}", text);

  let mut input = String::new();
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("No input");
  return input;
}
