use std::process::Command;
use std::process::exit;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("[Error] Usage: exeat <Path> <Command> (<args>..)");
        exit(1);
    }
    args.remove(0);
    let path: &str = &args.remove(0);
    let op: &str = &args.remove(0);
    let mut command = Command::new(op).current_dir(path).args(&args).spawn().expect("Failed to exec.");
    command.wait().unwrap();
}
