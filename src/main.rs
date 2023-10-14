use std::error::Error;
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn Error>> {
    let mut _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        println!("{}", b);
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        if c == 'q' {
            break;
        }
    }
    Ok(())
}
