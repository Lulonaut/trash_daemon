use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    AddFile,
    RestoreFile,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub payload: String,
    pub command_type: CommandType,
}

#[derive(Serialize, Deserialize)]
pub struct CommandResult {
    pub status: i32,
}

macro_rules! impl_send_and_receive {
    ($t:ty) => {
        impl $t {
            pub fn decode_from_stream(stream: UnixStream) -> std::io::Result<$t> {
                receive_message(stream)
            }
        }
    };
}

pub fn send_message<S>(mut stream: UnixStream, payload: S) -> std::io::Result<()>
where
    S: Serialize,
{
    let encoded = bincode::serialize(&payload).unwrap();
    stream.write_all(&*encoded)?;
    Ok(())
}

pub fn receive_message<S>(mut stream: UnixStream) -> std::io::Result<S>
where
    S: DeserializeOwned,
{
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    let decoded = bincode::deserialize(&response[..]).unwrap();
    Ok(decoded)
}
impl_send_and_receive!(CommandResult);
impl_send_and_receive!(Command);
