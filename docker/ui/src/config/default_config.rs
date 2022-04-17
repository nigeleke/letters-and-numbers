#[derive(Debug)]
pub struct DefaultConfig {
  pub api_url: String,
}

#[cfg(feature = "config-dev")]
impl Default for DefaultConfig {
  fn default() -> Self {
    DefaultConfig {
      api_url: "http://localhost:8088".to_owned(),
    }
  }
}

#[cfg(feature = "config-prod")]
impl Default for DefaultConfig {
  fn default() -> Self {
    DefaultConfig {
      api_url: "http://pidp11.local/lettersandnumbers".to_owned(),
    }
  }
}
