use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::exit;

use shared::SendMessage;
use shared::{Command, CommandResult, CommandType};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut stream = match UnixStream::connect("socket") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error while attempting to connect to socket: {}", e);
            eprintln!("Is the daemon running?");
            exit(1);
        }
    };

    let path = PathBuf::from(&args[1]);
    let absolute_path = if path.is_absolute() {
        path
    } else {
        // This also needs to work for non-existent files to restore them outside the trash directory, therefore std::fs::canonicalize() wont work
        std::env::current_dir()
            .expect("current working dir should exist (do you have valid permissions?)")
            .join(path)
    };

    dbg!(&absolute_path.to_str());
    stream.send_message(Command {
        command_type: CommandType::AddFilesToTrash,
        thing: vec![absolute_path],
    })?;

    let command_result: CommandResult = stream.receive_message()?;
    println!("{}", command_result.status);
    Ok(())
}
