use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn is_writable(path: &str) -> bool {
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .is_ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: waitonfile.exe <filename>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    println!("Waiting for {}...", file_path);

    while !Path::new(file_path).exists() || !is_writable(file_path) {
        sleep(Duration::from_secs(1));
    }

    println!("[found]");
}

