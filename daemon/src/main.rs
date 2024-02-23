use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;

use signal_hook::{consts::SIGINT, iterator::Signals};

use shared::{Command, CommandIO, CommandResult, CommandType};
use shared::config::{Config, read_config};

use crate::trash_folder::add_files_to_trash;

mod trash_folder;

fn main() -> std::io::Result<()> {
    let config = read_config()?;
    eprintln!("Using {:?} as the trash folder root", {
        &config.trash_folder_path
    });

    let _ = std::fs::remove_file(&config.socket_path);

    eprintln!("Starting UNIX socket at {:?}", &config.socket_path);
    let socket = UnixListener::bind(config.socket_path.clone())?;

    let mut signals = Signals::new([SIGINT])?;

    let socket_path = config.socket_path.clone();
    std::thread::spawn(move || {
        if signals.forever().next().is_some() {
            // Only SIGINT will make it here, exit gracefully
            match std::fs::remove_file(socket_path) {
                Ok(_) => exit(0),
                Err(_) => {
                    eprintln!("Failed to remove socket file");
                    exit(1);
                }
            }
        }
    });

    for stream in socket.incoming() {
        let mut stream = stream?;

        if handle_client(&mut stream, config.clone()).is_err() {
            eprintln!("Error occurred while handling client!");
        }
    }
    Ok(())
}

fn handle_client(stream: &mut UnixStream, config: Config) -> std::io::Result<()> {
    let command: Command = stream.receive_message()?;
    dbg!(&command);
    match command.command_type {
        CommandType::AddFilesToTrash => {
            if let Err(e) = add_files_to_trash(&command.paths, config.trash_folder_path) {
                return stream.send_message(CommandResult {
                    status: 1,
                    message: Some(e.to_string()),
                });
            }
        }
        CommandType::RestoreFiles => {}
    }

    stream.send_message(CommandResult {
        status: 0,
        message: Some(String::from("OK")),
    })
}
