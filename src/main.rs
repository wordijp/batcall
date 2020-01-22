use std::env;
use std::process::{self, Child, Command};
use std::path::Path;

extern crate regex;
use regex::Regex;

extern crate encoding_rs;
use encoding_rs::SHIFT_JIS;

const OPTION_WHERE_MKLINK: &str = "--batcall-where-mklink";

fn main() {
    let mut _args = env::args();
    // get bat & args
    let bin: String = _args.nth(0).unwrap();
    let args: Vec<String> = _args.collect();
    
    if args.len() >= 1 && args[0] == OPTION_WHERE_MKLINK {
        if args.len() != 2 {
            eprintln!("usage) {} {} <target>", bin, OPTION_WHERE_MKLINK);
            process::exit(1);
        }

        if self_is_symlink() {
            eprintln!("Cannot be run mklink by symlink.");
            eprintln!("please run from original: {}", Path::new(env::current_exe().unwrap().to_str().unwrap()).read_link().unwrap().to_str().unwrap());
            process::exit(1);
        }

        let status_code = do_option(&args[1]);
        process::exit(status_code);
    }
    
    if !self_is_symlink() {
        eprintln!("Cannot be run directly.");
        eprintln!("please create symlink");
        eprintln!("  usage) {} {} <target>", bin , OPTION_WHERE_MKLINK);
        eprintln!("         or");
        eprintln!("         mklink <target> {}", env::current_exe().unwrap().to_str().unwrap());
        process::exit(1);
    }

    // run
    let re = Regex::new(r"\.[eE][xX][eE]$").unwrap();
    let bat: String = (re.replace(&bin, "") + ".bat").to_string();
    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg("call")
        .arg(bat)
        .args(args)
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    process::exit(status.code().unwrap());
}

fn do_option(cmd: &String) -> i32 {
    let re_exe = Regex::new(r"\.[eE][xX][eE]$").unwrap();
    let re_bat = Regex::new(r"\.[bB][aA][tT]$").unwrap();

    if re_exe.is_match(cmd) {
        eprintln!(".exe cannot be specified");
        return 1;
    }
    if re_bat.is_match(cmd) {
        eprintln!(".bat cannot be specified");
        return 1;
    }

    let output = Command::new("where")
        .arg(&cmd)
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        eprint!("{}", to_utf8_string(output.stderr.as_ref()));
        return 1;
    }

    let mut found_bat: Option<&str> = None;
    let str = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = str.as_str().split('\n').collect();
    for line in lines {
        let line = line.trim();
        if re_exe.is_match(line) {
            eprintln!("{}.exe is already exists: {}", cmd, line);
            return 1;
        }

        if found_bat == None && re_bat.is_match(line) {
            found_bat = Some(line);
        }
    }
    
    if found_bat == None {
        eprintln!("{}.bat is not found", cmd);
        return 1;
    }
    
    // mklink exeute
    let path = Path::new(found_bat.unwrap());
    let bat_dir = path.parent().unwrap();
    env::set_current_dir(bat_dir).expect("failed: change bat file current dir");
    
    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg("mklink")
        .arg(cmd.to_string() + ".exe")
        .arg(env::current_exe().unwrap())
        .spawn()
        .unwrap();
    cmd.wait().unwrap();

    0
}

fn self_is_symlink() -> bool {
    let cwd = env::current_exe().unwrap();
    let path = Path::new(cwd.to_str().unwrap());

    path.read_link().is_ok()
}

fn to_utf8_string(v: &Vec<u8>) -> String {
    let decoded = SHIFT_JIS.decode(v);
    if !decoded.2 {
        return decoded.0.to_string();
    }

    String::from_utf8(v.to_owned()).unwrap()
}
