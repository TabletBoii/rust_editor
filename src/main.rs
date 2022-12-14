use std::io::{self, Read};
use crossterm::terminal;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(err: std::io::Error){
    panic!(e);
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        let byte = byte.unwrap();
        let character = byte as char;
        if character.is_control() {
            println!("{:?} \r", byte);
        }
        else {
            println!("{:?} ({})\r", byte, character);
        }
        if b == to_ctrl_byte('q'){
            break;
        }
    }
}
