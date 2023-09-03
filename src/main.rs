use std::error::Error;
use std::io;

use crate::battlefield::{Battlefield, FIELD_SIZE, XY};

mod ship;
mod battlefield;


fn main() {
    let mut field = Battlefield::generate();
    println!("{}", field);

    loop {
        println!("Enter XY to fire:");
        match read_stdin_xy() {
            Ok(xy) => {
                field.fire(xy);
                println!("{}", field);
            }

            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}

fn read_stdin_xy() -> Result<XY, Box<dyn Error>> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let user_input = user_input.trim();

    let coordinates = user_input.trim().parse::<usize>()?;
    if coordinates > 99 {
        return Err(From::from(format!("Wrong input: '{}'. Expected to be coordinates in range [00..99]", user_input)));
    }

    return Ok(XY(coordinates / FIELD_SIZE, coordinates % FIELD_SIZE));
}