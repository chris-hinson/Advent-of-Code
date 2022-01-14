//Christopher Hinson
//notes:

use std::fs;

fn main() {
    let filename = "./inputs/input22.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{contents}");
}
