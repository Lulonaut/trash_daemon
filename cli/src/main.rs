use shared::{receive_message, Command, CommandResult, CommandType};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("socket")?;
    stream.write_all(b"hello world\n")?;

    let command_result: CommandResult = CommandResult::decode_from_stream(stream)?;
    println!("{}", command_result.status);
    Ok(())
}
