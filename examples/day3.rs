//use std::fmt;
use std::fs;

fn main() {
    let filename = "input3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.lines().collect();
    let mut lines_vec: Vec<usize> = lines
        .iter_mut()
        //.inspect(|x| println!("{}", x))
        .map(|y| usize::from_str_radix(y, 2).unwrap())
        .collect::<Vec<usize>>();

    println!("{:b}", lines_vec[0]);

    //create our masks, one for each bit
    let mut masks: Vec<u32> = Vec::new();
    for digit in 0..(lines_vec[0] as f32).log2().ceil() as u32 {
        masks.push(1 << digit as usize);
        //println!("{:b}", masks.last().unwrap());
    }
    //masks.reverse();
    println!("{:?}", masks);

    let tot_lines = lines_vec.len();
    println!("number of lines: {}", tot_lines);
    let mut tots = vec![0; masks.len()];

    //for each line in the input
    for (i, line) in lines_vec.iter().enumerate() {
        println!("line is: {:b}", line);
        //apply each mask, and if the result is non-zero
        //inrement the column's total
        for (j, mask) in masks.iter().enumerate() {
            if (*line as u32 & *mask as u32) != 0 {
                tots[j] += 1;
                println!("increasing bit {}", j);
            }
        }
    }

    println!("final totals  {:?}", tots);
    //check which totals are greater than 50%
    let mut out: Vec<bool> = vec![false; tots.len()];
    for (i, total) in tots.iter().enumerate() {
        if total >= &(tot_lines / 2) {
            out[i] = true;
        }
    }
    println!("places > 1/2: {:?}", out);
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, place) in out.iter().enumerate() {
        if *place {
            gamma = gamma | masks[i];
        } else {
            epsilon = epsilon | masks[i];
        }
    }

    println!("gamma: {:b} or {}", gamma, gamma);
    println!("epsilon: {:b} or {}", epsilon, epsilon);
    let power = (gamma as u32) * (epsilon as u32);

    println!("power: {}", power);
}
