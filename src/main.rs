use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                println!("Binary: {0:08b} ASCII: {0:#03}{1}\r", b,
                    if !c.is_control() {
                        format!(" Character: {0:#?}", c)
                    } else {
                        String::from("")
                    });
                if c == 'q' {
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    disable_raw_mode().unwrap();
}
