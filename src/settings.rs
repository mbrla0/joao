use serde_derive::{Deserialize, Serialize};

use std::path::PathBuf;
use std::time::Duration;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct FilesystemLogger {
    pub enabled: bool,
    pub name: String,
    pub max_queue_size: u64,
    pub max_file_size: u64,
    pub history_size: u64,
    pub directory: PathBuf,
    pub flush_period: Duration,
}
impl Default for FilesystemLogger {
    fn default() -> FilesystemLogger {
        FilesystemLogger {
            enabled: false,
            name: "log".to_owned(),
            max_queue_size: 67108864,
            max_file_size: 67108864,
            history_size: 8,
            directory: PathBuf::from("./log/"),
            flush_period: Duration::new(10, 0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Logging {
    pub level: log::Level,
}
impl Default for Logging {
    fn default() -> Logging {
        let level = if cfg!(debug_assertions) {
            log::Level::Trace
        } else {
            log::Level::Info
        };

        Logging { level: level }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Auth {
    pub algorithm: jwt::Algorithm,
    pub secret: String,
}
impl Default for Auth {
    fn default() -> Auth {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        use std::iter;

        let mut rng = thread_rng();
        Auth {
            algorithm: jwt::Algorithm::HS256,
            secret: iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(32)
                .collect(),
        }
    }
}

use std::collections::BTreeMap;
use std::net::{SocketAddr, ToSocketAddrs};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Settings {
    pub listen_address: String,
    pub database_address: String,
    pub database_id: u8,
    pub workers: u16,
    pub keep_alive: u32,
    pub size_limits: BTreeMap<String, u64>,
    pub logging: Logging,
    pub filesystem_logger: FilesystemLogger,
    pub auth: Auth,
}
impl Default for Settings {
    fn default() -> Settings {
        Settings {
            listen_address: "0.0.0.0:6969".to_owned(),
            database_address: "127.0.0.1:6380".to_owned(),
            database_id: 0,
            workers: 4,
            keep_alive: 0,
            size_limits: BTreeMap::new(),
            logging: Default::default(),
            filesystem_logger: Default::default(),
            auth: Default::default(),
        }
    }
}
impl Settings {
    pub fn listen(&self) -> Result<Option<SocketAddr>, std::io::Error> {
        self.listen_addrs().map(|mut iter| iter.next())
    }

    pub fn database(&self) -> Result<Option<SocketAddr>, std::io::Error> {
        self.database_addrs().map(|mut iter| iter.next())
    }

    pub fn listen_addrs(&self) -> Result<impl Iterator<Item = SocketAddr>, std::io::Error> {
        self.listen_address.as_str().to_socket_addrs()
    }

    pub fn database_addrs(&self) -> Result<impl Iterator<Item = SocketAddr>, std::io::Error> {
        self.database_address.as_str().to_socket_addrs()
    }
}
