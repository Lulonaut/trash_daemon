use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;

use signal_hook::{consts::SIGINT, iterator::Signals};

use shared::{CommandResult, send_message};

fn read_line_from_stream(stream: &UnixStream) -> std::io::Result<String> {
    let mut string = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut string)?;

    Ok(string)
}

fn main() -> std::io::Result<()> {
    let socket_path = "socket";
    let _ = std::fs::remove_file(socket_path);

    let socket = UnixListener::bind(socket_path)?;

    let mut signals = Signals::new(&[SIGINT])?;

    std::thread::spawn(move || {
        for _ in signals.forever() {
            // Only SIGINT will make it here, exit gracefully
            match std::fs::remove_file(socket_path) {
                Ok(_) => {
                    exit(0)
                }
                Err(_) => {
                    eprintln!("Failed to remove socket file");
                    exit(1);
                }
            }
        }
    });
    for stream in socket.incoming() {
        let stream = stream?;

        if handle_client(stream).is_err() {
            eprintln!("Error occurred while handling client!");
        }
    }
    Ok(())
}

fn handle_client(stream: UnixStream) -> std::io::Result<()> {
    let message = read_line_from_stream(&stream)?;
    print!("{}", message);

    send_message(stream, CommandResult { status: 0 })?;
    Ok(())
}
