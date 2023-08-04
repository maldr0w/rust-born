
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

// use std::io::{self, Read};  // use = import, io = input/output, so we import io
// use std::io; 
// use std::io::read; => read enables us to use bytes()
// use std::something => use standard library::library_name
// ; is end of statement


// ctrl+d => reached end of file = exit
// ctrl+c => terminate immediately

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap(); // Ownership system _something tells readers to hold on to stdout even though we are not using it
    
    // for every byte you can read from the keyboard, bind to b and execute following block
    for b in io::stdin().bytes() {  // stdin => standard input stream, gives access to everything that can be put into the program
        
        // let c = b.unwrap() as char;  // 
        // println!("{}", c);

        let b = b.unwrap(); // declaring b for the second time: variable shadowing, first b is not useful to us
        let c = b as char;  // as transforms primitive value into another, byte to a single character
        if c.is_control() {  // if c is a control character = non-printable char that we dont want to print to screen
            println!("{:?} \r", b);  // println macro prints its input in a single line {:?} is a placeholder, \r => carriage return => output is printed line by line without indentation, carriage return moves cursor back to beginning of current line
        } else {                        // println! then adds a new line \n, moves cursor down a line
            println!("{:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}


