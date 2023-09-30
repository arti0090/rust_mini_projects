use std::io;
use crate::hanger::Hanger;

mod hanger;

fn main() {
    let mut hanger = Hanger::new();
    hanger.play();

    // don't close terminal window after execution
    io::stdin().read_line(&mut String::new()).unwrap();
}
