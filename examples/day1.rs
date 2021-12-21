//Christopher Hinson
//notes: dont love second depth calculation as it relies on the first for indexing, since we cannot
//"see" the next item of the depths in an anonymous function

use std::fs;

fn main() {
    let filename = "input1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let depths: Vec<i32> = contents
        .lines()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();

    let depths2: Vec<i32> = contents
        .lines()
        //parse from strings into i32 vec (one per line)
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter_mut()
        .enumerate()
        .map(|(index, item)| {
            if depths.get(index + 1) != None {
                *item += depths.get(index + 1).unwrap();
            }
            if depths.get(index + 2) != None {
                *item += depths.get(index + 2).unwrap();
            }
            *item //+ depths.get(index + 1).unwrap() + depths.get(index + 2).unwrap()
        })
        //collect it back into a vec of usize
        .collect::<Vec<i32>>();

    println!("raw depths(0-10): {:?}", &depths[0..11]);
    println!("sliding fram depths (0-10): {:?}", &depths2[0..11]);

    //part1////////////////////////////////////////////////////////////////////////////////////////
    let mut increases = 0;
    let mut prev = depths[0];
    for (i, _depth) in depths.iter().enumerate() {
        //println!("{}: {}", i, depth);
        if depths[i] > prev {
            increases += 1;
        }
        prev = depths[i];
    }

    println!("there are: {} increases in raw depth", increases);
    ///////////////////////////////////////////////////////////////////////////////////////////////

    //part2////////////////////////////////////////////////////////////////////////////////////////
    increases = 0;
    prev = depths2[0];
    for (i, _depth) in depths2.iter().enumerate() {
        if depths2[i] > prev {
            increases += 1;
        }
        prev = depths2[i];
    }

    println!(
        "there are: {} increases in depths (sliding frame)",
        increases
    );
    ///////////////////////////////////////////////////////////////////////////////////////////////
}
