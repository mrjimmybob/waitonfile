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

fn hide_cursor() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}

fn show_cursor() {
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    let mut ascii_mode = false;

    for arg in &args[1..] {
        if arg == "--ascii" {
            ascii_mode = true;
        } else if filename.is_none() {
            filename = Some(arg.clone());
        }
    }

    if filename.is_none() {
        eprintln!("Usage: waitonfile [--ascii] <filename>");
        std::process::exit(1);
    }

    let filename = filename.unwrap();
    let path = Path::new(&filename);

    let spinner_utf8 = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let spinner_ascii = ['.', 'o', 'O', 'o'];

    let spinner: &[char] = if ascii_mode {
        &spinner_ascii[..]
    } else {
        &spinner_utf8[..]
    };

    let mut idx = 0;

    // Hide cursor and ensure it's restored on exit
    hide_cursor();
    let _guard = CursorGuard;

    // Ctrl+C handler
    ctrlc::set_handler(|| {
        show_cursor();
        std::process::exit(130);
    })
    .expect("Error setting Ctrl-C handler");

    //format!("\rWaiting for {}... [found] ✅    \n", filename)

    loop {
        if path.exists() {
            if is_file_writable(&filename) {
                let final_msg = if ascii_mode {
                    format!("\rWaiting for {}... [unlocked] [ok]   \n", filename)
                } else {
                    format!("\r✅ Waiting for {}... [unlocked]     \n", filename)
                };
                print!("{}", final_msg);
                io::stdout().flush().unwrap();
                break;
            } else {
                let waiting_msg = if ascii_mode {
                    format!(
                        "\rWaiting for \x1B[36m{}\x1B[0m... [\x1B[31mlocked\x1B[0m] \x1B[33m{}\x1B[0m",
                        filename, spinner[idx]
                    )
                } else {
                    format!(
                        "\r🔒 Waiting for \x1B[36m{}\x1B[0m... [\x1B[31mlocked\x1B[0m] \x1B[33m{}\x1B[0m",
                        filename, spinner[idx]
                    )
                };
                print!("{}", waiting_msg);
            }
        } else {
            let nofile_msg = if ascii_mode {
                format!(
                    "\rWaiting for \x1B[36m{}\x1B[0m... \x1B[33m{}\x1B[0m",
                    filename, spinner[idx]
                )
            } else {
                format!(
                    "\r🚫 Waiting for \x1B[36m{}\x1B[0m... \x1B[33m{}\x1B[0m",
                    filename, spinner[idx]
                )
            };
            print!("{}", nofile_msg);
        }

        io::stdout().flush().unwrap();
        idx = (idx + 1) % spinner.len();
        thread::sleep(Duration::from_millis(100));
    }
}

struct CursorGuard;

impl Drop for CursorGuard {
    fn drop(&mut self) {
        show_cursor();
    }
}
