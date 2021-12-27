//Christopher Hinson
//notes

use std::fs;

fn main() {
    let filename = "input8.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //keep each line as a two part tuple of str vecs
    let lines = contents
        .lines()
        .map(|line| line.split("|").collect::<Vec<&str>>())
        .map(|line| {
            (
                line[0]
                    .strip_suffix(" ")
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<&str>>(),
                line[1]
                    .strip_prefix(" ")
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<&str>>(),
            )
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    //println!("input lines: {:?}", lines[0]);

    //for day one, we just want to find how many lines have 2,4,3,or 7 len strs in their output section

    let mut ones = 0;
    let mut fours = 0;
    let mut sevens = 0;
    let mut eights = 0;

    for line in lines {
        for item in line.1 {
            match item.len() {
                2 => ones += 1,
                4 => fours += 1,
                3 => sevens += 1,
                7 => eights += 1,
                _ => (),
            }
        }
    }
    println!(
        "ones: {}, fours:{}, sevens:{}, eights:{}\nsum:{}",
        ones,
        fours,
        sevens,
        eights,
        ones + fours + sevens + eights
    );
}
