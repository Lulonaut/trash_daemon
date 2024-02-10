use std::io::{ErrorKind, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    AddFilesToTrash,
    RestoreFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub thing: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResult {
    pub status: i32,
}

pub trait SendMessage {
    fn send_message<S>(&mut self, payload: S) -> std::io::Result<()>
    where
        S: Serialize;

    fn receive_message<S>(&mut self) -> std::io::Result<S>
    where
        S: DeserializeOwned;
}

impl SendMessage for UnixStream {
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
