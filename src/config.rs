use std::{net::SocketAddr, path::Path, path::PathBuf};

#[derive(Debug)]
pub(crate) struct Config {
  pub interface: String,
  pub ip_address: SocketAddr,
  pub config_location: PathBuf,
}

impl Config {
  pub(crate) fn default() -> Config {
    Config {
      interface: "lo".to_string(),
      ip_address: "0.0.0.0:5353"
        .parse::<std::net::SocketAddrV4>()
        .unwrap()
        .into(),
      config_location: Path::new(".").to_owned(),
    }
  }
}
