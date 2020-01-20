use std::env;
use std::process::{self, Child, Command};

extern crate regex;
use regex::Regex;

fn main() {
    let mut _args = env::args();
    // get bat & args
    let bin = _args.nth(0).unwrap();
    let re = Regex::new(r"\.[eE][xX][eE]$").unwrap();
    let bat: String = (re.replace(&bin, "") + ".bat").to_string();
    let args: Vec<_> = _args.collect();

    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg("call")
        .arg(bat)
        .args(args)
        .spawn()
        .unwrap();

    let status_code = cmd.wait().unwrap();
    process::exit(status_code.code().unwrap());
}
