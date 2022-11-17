use std::{io::{self, Read}, process::exit, os::unix::prelude::AsRawFd};


// We unwrap here since we should panic if we can't change the terminal flags
// For now we are just disabling echoing in the terminal but eventually we will also enable raw mode
fn enable_raw_mode() -> () {
    let mut termios = termios::Termios::from_fd(io::stdin().as_raw_fd()).unwrap();

    // Flags in order:
    // Disable echoing to the terminal
    // Disable Canonical Mode (meaning we get input without waiting for a new line)
    // Disable SIGINT and SIGSTSP
    termios.c_lflag &= !(termios::ECHO | termios::ICANON | termios::ISIG);

    set_termios(io::stdin().as_raw_fd(), termios);
}


fn set_termios(fd: i32, termios: termios::Termios) -> (){
    termios::tcsetattr(fd, termios::TCSAFLUSH, &termios).unwrap();
}

fn main() {
    let original_termios = termios::Termios::from_fd(io::stdin().as_raw_fd()).unwrap();

    enable_raw_mode();

    // Main loop to read user input one byte at a time
    let mut input_byte: u8;
    let mut input_char: char;
    for byte in io::stdin().bytes() {
        input_byte = byte.unwrap();
        input_char = char::from(input_byte);


        if input_char == 'q' {
            set_termios(io::stdin().as_raw_fd(), original_termios);
            exit(0);
        }

        if input_char.is_control() {
            println!("{}", input_byte);
        } else {
            println!("{} -> {}", input_byte, input_char);
        } 
    }
}
