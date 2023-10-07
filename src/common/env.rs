use std::{env, str::FromStr};

pub struct Env {}

impl Env {
  pub fn get<T: FromStr>(key: &str, or: &str) -> T {
    match env::var(key).unwrap_or(String::from(or)).parse::<T>() {
      Ok(value) => value,
      Err(_) => panic!("Can't get value for: {}", key),
    }
  }
}
