use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::exit;

use shared::{CommandIO, convert_to_absolute_path};
use shared::{Command, CommandResult, CommandType};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config = shared::config::read_config()?;

    let mut stream = match UnixStream::connect(config.socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error while attempting to connect to socket: {}", e);
            eprintln!("Is the daemon running?");
            exit(1);
        }
    };

    let path = PathBuf::from(&args[1]);
    let absolute_path = convert_to_absolute_path(path);

    dbg!(&absolute_path.to_str());
    stream.send_message(Command {
        command_type: CommandType::AddFilesToTrash,
        paths: vec![absolute_path],
    })?;

    let command_result: CommandResult = stream.receive_message()?;
    println!("{};{:?}", command_result.status, command_result.message);
    Ok(())
}
