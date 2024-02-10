use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;

use signal_hook::{consts::SIGINT, iterator::Signals};

use shared::{Command, CommandResult, SendMessage};

fn main() -> std::io::Result<()> {
    let socket_path = "socket";
    let _ = std::fs::remove_file(socket_path);

    let mut signals = Signals::new([SIGINT])?;
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

    let socket = UnixListener::bind(socket_path)?;
    for stream in socket.incoming() {
        let mut stream = stream?;

        if handle_client(&mut stream).is_err() {
            eprintln!("Error occurred while handling client!");
        }
    }
    Ok(())
}

fn handle_client(stream: &mut UnixStream) -> std::io::Result<()> {
    let command: Command = stream.receive_message()?;
    dbg!(&command);

    stream.send_message(CommandResult { status: 0 })
}
