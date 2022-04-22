// Turns clippy into aggressive mode
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let l = ["1", "2", "3"];
    for i in &l {
        println!("{}", i);
    }
}
