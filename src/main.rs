use std::env;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::process::Command;

extern crate serde_json;
use serde_json::Value;

fn handle_client(stream: TcpStream) {
    let reader = io::BufReader::new(stream);
    for maybe_cmd in reader.lines() {
        let cmd_str = maybe_cmd.unwrap();
        let cmd_json: Value = serde_json::from_str(&cmd_str).unwrap();
        let cmd_tokens = cmd_json.as_array().unwrap();

        let title = cmd_tokens[0].as_str().unwrap();
        let cwd = cmd_tokens[1].as_str().unwrap();
        let executable = cmd_tokens[2].as_str().unwrap();

        let mut process = Command::new(executable);
        process.current_dir(cwd);
        process.arg(title);
        for arg in cmd_tokens.iter().skip(3) {
            process.arg(arg.as_str().unwrap());
        }
        process.spawn().expect("XGE-Launcher: failed to launch process!");
    }
}

fn main() {
    /*let listener = TcpListener::bind("127.0.0.1:7787")?;

    for stream in listener.incoming() {
        println!("client connected.");
        handle_client(stream?);
        break;
    }*/
    let args: Vec<String> = env::args().collect();
    let stream = TcpStream::connect(&args[1]).unwrap();
    handle_client(stream);
}
