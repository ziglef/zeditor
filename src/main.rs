use std::{io::{self, Read}, process::exit, os::unix::prelude::AsRawFd};


// We unwrap here since we should panic if we can't change the terminal flags
// For now we are just disabling echoing in the terminal but eventually we will also enable raw mode
fn enable_raw_mode() -> () {
    let mut raw = termios::Termios::from_fd(io::stdin().as_raw_fd()).unwrap();

    raw.c_lflag &= !termios::ECHO;

    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSAFLUSH, &raw).unwrap();
}


// We unwrap here since we should panic if we can't change the terminal flags
// For now we are just enabling echoing in the terminal but eventually we will also disable raw mode
fn disable_raw_mode() -> () {
    let mut raw = termios::Termios::from_fd(io::stdin().as_raw_fd()).unwrap();

    raw.c_lflag |= termios::ECHO;

    termios::tcsetattr(io::stdin().as_raw_fd(), termios::TCSAFLUSH, &raw).unwrap();
}

fn main() {
    enable_raw_mode();

    // Main loop to read user input one byte at a time
    let mut input: char;
    for byte in io::stdin().bytes() {
        match byte {
            Ok(byte_value) => input = char::from(byte_value), 
            Err(err) => {
                println!("Error parsing bytes from stdin.\nError code: {}", err);
                disable_raw_mode();
                exit(-1);
            }
        }

        if input == 'q' {
            disable_raw_mode();
            exit(0);
        }

        print!("{}", input)
    }
}
