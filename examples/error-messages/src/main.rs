extern crate wasc;

use wasc::compile;

fn main() {
    let input = include_str!("main.wasc");
    let output = compile(input);

    println!("{:?}", output);
}
