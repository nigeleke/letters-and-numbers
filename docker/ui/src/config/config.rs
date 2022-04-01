#[derive(Debug)]
pub struct Config {
  pub api_url: String,
}

#[cfg(feature = "config-dev")]
impl Default for Config {
  fn default() -> Self {
    Config {
      api_url: "http://localhost:8088".to_owned(),
    }
  }
}

#[cfg(feature = "config-prod")]
impl Default for Config {
  fn default() -> Self {
    Config {
      api_url: "http://lettersandnumbers.pi.local".to_owned(),
    }
  }
}
