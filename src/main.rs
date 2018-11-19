use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

use std::env;
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::process::Command;
use std::str;

#[macro_use]
extern crate serde_json;

fn accept_commands(stream: TcpStream) {
    let reader = io::BufReader::new(stream);
    for cmd_str in reader.lines().map(|l| l.unwrap()) {
        let local = cmd_str.chars().nth(0) == Some('l');
        let cmd_tokens: Vec<String> = serde_json::from_str(&cmd_str[1..]).unwrap();

        let exe = env::current_exe().unwrap();
        let cwd = &cmd_tokens[0];
        let title = &cmd_tokens[1];
        let args = cmd_tokens.iter().skip(2);

        let mut cmd = Command::new("xgSubmit");
        cmd.current_dir(cwd).arg(format!("/caption={}", title));
        if local {
            cmd.arg("/allowremote=off");
        }
        cmd.arg("/command")
            .arg(exe)
            .arg("w")
            .args(args)
            .spawn()
            .expect("XGE-Launcher: failed to launch process!");
    }
}

fn report(id: &str, exit_code: i32, output: &str) {
    let result = json!([id, exit_code, output]);
    println!("mwt {}", result);
    ::std::process::exit(exit_code);
}

fn execute_wrapped(id: &str, exe: &str, args: Vec<&String>) {
    let mut cmd = Command::new(&exe);
    for arg in args {
        cmd.arg(arg);
    }

    let maybe_output = cmd.output();
    match maybe_output {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-7787);
            let stdout = str::from_utf8(&output.stdout).unwrap_or("couldn't decode output!");
            let stderr = str::from_utf8(&output.stderr).unwrap_or("couldn't decode output!");
            let output_str = stderr.to_owned() + stdout;

            report(id, exit_code, &output_str);
        }
        Err(e) => {
            report(
                id,
                -7787,
                &format!("XGE-Launcher: failed to execute process: {}", e),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "client" {
        let stream = TcpStream::connect(&args[2]).unwrap();
        accept_commands(stream);
    } else if args[1] == "w" {
        execute_wrapped(&args[2], &args[3], args.iter().skip(4).collect());
    }
}
