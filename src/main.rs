use std::env;
use std::process::{self, Child, Command};
use std::path::Path;

extern crate regex;
use regex::Regex;

extern crate encoding_rs;
use encoding_rs::SHIFT_JIS;

const OPTION_WHERE_MKLINK: &str = "--batcall-where-mklink";
const PATHEXT: &str = "PATHEXT";

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE_EXE: Regex = Regex::new(r"\.[eE][xX][eE]$").unwrap();
    static ref RE_BAT: Regex = Regex::new(r"\.[bB][aA][tT]$").unwrap();
    static ref RE_CMD: Regex = Regex::new(r"\.[cC][mM][dD]$").unwrap();
}

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
    let bat_cmd = find_bat_cmd();
    if bat_cmd.is_none() {
        eprintln!(".bat or .cmd file is not found in {} env", PATHEXT);
        process::exit(1);
    }
    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg("call")
        .arg(bat_cmd.unwrap())
        .args(args)
        .spawn()
        .unwrap();

    let status = cmd.wait().unwrap();
    process::exit(status.code().unwrap());
}

fn do_option(cmd: &String) -> i32 {
    if RE_EXE.is_match(cmd) {
        eprintln!(".exe cannot be specified");
        return 1;
    }
    if RE_BAT.is_match(cmd) {
        eprintln!(".bat cannot be specified");
        return 1;
    }
    if RE_CMD.is_match(cmd) {
        eprintln!(".cmd cannot be specified");
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
    let mut found_cmd: Option<&str> = None;
    let str = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = str.as_str().split('\n').collect();
    for line in lines {
        let line = line.trim();
        if RE_EXE.is_match(line) {
            eprintln!("{}.exe is already exists: {}", cmd, line);
            return 1;
        }

        if found_bat.is_none() && RE_BAT.is_match(line) {
            found_bat = Some(line);
        }
        if found_cmd.is_none() && RE_CMD.is_match(line) {
            found_cmd = Some(line);
        }
    }

    if found_bat.is_none() && found_cmd.is_none() {
        eprintln!("{}.bat and {}.cmd is not found", cmd, cmd);
        return 1;
    }

    // mklink exeute
    let path = Path::new(if found_bat.is_some() { found_bat.unwrap() } else { found_cmd.unwrap() });
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

fn find_bat_cmd() -> Option<String> {
    let pathext = env::var(PATHEXT).expect(&format!("{} env is not found", PATHEXT));
    for ext in pathext.split(";") {
        if RE_BAT.is_match(ext) {
            if let Some(bat) = find_target(".bat") {
                return Some(bat);
            }
        } else if RE_CMD.is_match(ext) {
            if let Some(cmd) = find_target(".cmd") {
                return Some(cmd);
            }
        }
    }

    None
}

fn find_target(ext: &str) -> Option<String> {
    let cwd = env::current_exe().unwrap();
    let exe_path = Path::new(cwd.to_str().unwrap());
    let exe_dir = exe_path.parent().unwrap().to_str().unwrap();
    let exe_file = exe_path.file_name().unwrap().to_str().unwrap();

    let cmd: String = (RE_EXE.replace(exe_file, "") + ext.as_ref()).to_string();
    if !Path::new(&format!("{}/{}", exe_dir, cmd)).exists() {
        return None;
    }

    Some(cmd)
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
