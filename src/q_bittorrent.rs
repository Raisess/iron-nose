use reqwest::{Response, StatusCode};

use crate::common::http_client::{HttpClient, HttpMethod};

pub struct QBittorrent {
  __session_cookie: String,
  client: HttpClient,
}

impl QBittorrent {
  pub fn new(host: &str, port: u16) -> Self {
    return Self {
      __session_cookie: String::from("INVALID"),
      client: HttpClient::new(&format!("http://{}:{}", host, port)),
    }
  }

  pub async fn login(&mut self, username: &str, password: &str) -> () {
    let body = Some([("username", username), ("password", password)].to_vec());
    let response = self.request(HttpMethod::POST, "/api/v2/auth/login", body)
      .await;
    self.__session_cookie = 
      response.headers()["set-cookie"].to_str().unwrap().to_string();
  }

  pub async fn add_torrent(&mut self, name: &str, magnet_link: &str) -> () {
    let body = Some([("rename", name), ("urls", magnet_link)].to_vec());
    let response = self.request(HttpMethod::POST, "/api/v2/torrents/add", body)
      .await;

    if response.status() != StatusCode::OK {
      println!("Can't add torrent for: {}", name);
    }
  }

  async fn request(
    &mut self,
    method: HttpMethod,
    path: &str,
    body: Option<Vec<(&str, &str)>>,
  ) -> Response {
    let result = self.client.req(
      method,
      path,
      body,
      Some(&self.__session_cookie)
    ).await;

    match result {
      Ok(response) => response,
      Err(error) => panic!("Failed to make request: {}", error)
    }
  }
}
