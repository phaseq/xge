use std::env;
use std::process::Command;
use std::str;

#[macro_use]
extern crate serde_json;

fn report(id: &str, exit_code: i32, output: &str) {
    let result = json!([id, exit_code, output]);
    println!("mwt {}", result);
    ::std::process::exit(exit_code);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _title = &args[1];
    let id = &args[2];
    let mut cmd = Command::new(&args[3]);
    for arg in args.iter().skip(4) {
        cmd.arg(arg);
    }

    let maybe_output = cmd.output();
    if !maybe_output.is_ok() {
        report(id, -7787, "failed to execute process!");
    }
    let output = cmd.output().unwrap();
    let exit_code = output.status.code().unwrap_or(-7787);
    let stdout = str::from_utf8(&output.stdout).unwrap_or("couldn't decode output!");
    let stderr = str::from_utf8(&output.stderr).unwrap_or("couldn't decode output!");
    let output_str = stderr.to_owned() + stdout;

    report(id, exit_code, &output_str);
}
