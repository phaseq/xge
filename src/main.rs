use std::process::Command;
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead};
 
extern crate serde_json;
use serde_json::Value;
 
fn handle_client(stream: TcpStream) {
	let reader = io::BufReader::new(stream);
	for maybe_cmd in reader.lines() {
		let cmd_str = maybe_cmd.unwrap();
		let cmd_json: Value = serde_json::from_str(&cmd_str).unwrap();
		let cmd_tokens = cmd_json.as_array().unwrap();
		
		let mut process = Command::new(cmd_tokens[1].as_str().unwrap());
		process.current_dir(cmd_tokens[0].as_str().unwrap());
		for arg in cmd_tokens.iter().skip(2) {
			process.arg(arg.as_str().unwrap());
		}
		process.spawn().expect("failed to launch process!");
	}
}

fn main() -> io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:7787")?;

	for stream in listener.incoming() {
		println!("client connected.");
		handle_client(stream?);
		break;
	}
	Ok(())
}