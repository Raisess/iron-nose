use reqwest::{header, Error, Response};
use urlencoding::encode;

pub enum HttpMethod {
  GET,
  POST,
}

pub struct HttpClient {
  __host: String,
  client: reqwest::Client,
}

impl HttpClient {
  pub fn format(quote: &str) -> String {
    return encode(quote).into_owned();
  }

  pub fn new(host: &str) -> Self {
    return Self {
      __host: host.to_string(),
      client: reqwest::Client::new(),
    }
  }

  pub async fn req(
    &self,
    method: HttpMethod,
    path: &str,
    body: Option<Vec<(&str, &str)>>,
    cookie: Option<&str>,
  ) -> Result<Response, Error> {
    match method {
      HttpMethod::GET => {
        let result: Result<Response, Error> =
          self.client.get(self.uri(path))
            .header(header::COOKIE, cookie.unwrap_or(""))
            .send()
            .await;

        return result;
      },
      HttpMethod::POST => {
        let result: Result<Response, Error> =
          self.client.post(self.uri(path))
            .header(header::COOKIE, cookie.unwrap_or(""))
            .header(header::REFERER, self.__host.to_string())
            .form(&body)
            .send()
            .await;

        return result
      }
    }
  }

  fn uri(&self, path: &str) -> String {
    return format!("{}{}", self.__host, path);
  }
}
