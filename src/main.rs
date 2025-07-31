use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
    thread,
    time::Duration,
};

fn is_file_writable(path: &str) -> bool {
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .is_ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: waitonfile <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);

    let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut idx = 0;

    print!("Waiting for {}... ", filename);
    io::stdout().flush().unwrap();

    loop {
        if path.exists() && is_file_writable(filename) {
            println!("\r[found]                        ");
            break;
        }

        print!("\rWaiting for {}... {}", filename, spinner[idx]);
        io::stdout().flush().unwrap();

        idx = (idx + 1) % spinner.len();
        thread::sleep(Duration::from_millis(100));
    }
}
