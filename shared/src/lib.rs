use std::io::{ErrorKind, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub mod config;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    AddFilesToTrash,
    RestoreFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub paths: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResult {
    pub status: i32,
    pub message: Option<String>,
}

pub trait CommandIO {
    fn send_message<S>(&mut self, payload: S) -> std::io::Result<()>
    where
        S: Serialize;

    fn receive_message<S>(&mut self) -> std::io::Result<S>
    where
        S: DeserializeOwned;
}

impl CommandIO for UnixStream {
    fn send_message<S>(&mut self, payload: S) -> std::io::Result<()>
    where
        S: Serialize,
    {
        let encoded = bincode::serialize(&payload).unwrap();
        self.write_all(&encoded)?;
        // Send EOF. We only send and receive a single message, both as the client and the server, so this is fine
        self.shutdown(Shutdown::Write)?;
        Ok(())
    }

    fn receive_message<S>(&mut self) -> std::io::Result<S>
    where
        S: DeserializeOwned,
    {
        let mut response = Vec::new();
        self.read_to_end(&mut response)?;

        bincode::deserialize(&response[..])
            .map_err(|_| std::io::Error::new(ErrorKind::InvalidData, "Invalid data"))
    }
}

pub fn convert_to_absolute_path<S>(path: S) -> PathBuf
where
    S: Into<PathBuf>,
{
    let path = path.into();
    if path.is_absolute() {
        path
    } else {
        // This also needs to work for non-existent files to restore them outside the trash directory, therefore std::fs::canonicalize() wont work
        std::env::current_dir()
            .expect("current working dir should exist (do you have valid permissions?)")
            .join(path)
    }
}
