//use std::fmt;
use std::fs;

fn main() {
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let filename = "input3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.lines().collect();
    let mut lines_vec: Vec<usize> = lines
        .iter_mut()
        //.inspect(|x| println!("{}", x))
        .map(|y| usize::from_str_radix(y, 2).unwrap())
        .collect::<Vec<usize>>();

    println!("{:b}", lines_vec[0]);
    let tot_lines = lines_vec.len();
    println!("number of lines: {}", tot_lines);
    ///////////////////////////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //create our masks, one for each bit
    let mut masks: Vec<u32> = Vec::new();
    for digit in 0..(lines_vec[0] as f32).log2().ceil() as u32 {
        masks.push(1 << digit as usize);
        //println!("{:b}", masks.last().unwrap());
    }
    println!("{:?}", masks);
    ///////////////////////////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut tots = vec![0; masks.len()];

    //for each digit in one input line
    for (i, line) in lines_vec.iter().enumerate() {
        //println!("line is: {:b}", line);
        //apply each mask, and if the result is non-zero
        //inrement the column's total
        for (j, mask) in masks.iter().enumerate() {
            if (*line as u32 & *mask as u32) != 0 {
                tots[j] += 1;
                //println!("increasing bit {}", j);
            }
        }
    }
    //this prints "backwards" because of debugging formatter
    println!("final totals  {:?}", tots);
    /*print!("totals: ");
    let viewable = tots.iter().rev().enumerate();
    for i in viewable {
        print!("{} ", i.1);
    }*/
    ///////////////////////////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //check which totals are greater than 50%
    let mut out: Vec<bool> = vec![false; tots.len()];
    for (i, total) in tots.iter().enumerate() {
        if total >= &(tot_lines / 2) {
            out[i] = true;
        }
    }
    print!("\ncommon values: ");
    println!("{:?}", out);
    /*let viewable = out.iter().rev().enumerate();
    for i in viewable {
        print!("{} ", *i.1 as i32);
    }*/
    ///////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////part 1/////////////////////////////////////////////////////////////
    /*let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, place) in out.iter().enumerate() {
        if *place {
            gamma = gamma | masks[i];
        } else {
            epsilon = epsilon | masks[i];
        }
    }

    println!("\ngamma: {:b} or {}", gamma, gamma);
    println!("epsilon: {:b} or {}", epsilon, epsilon);
    let power = (gamma as u32) * (epsilon as u32);

    println!("power: {}", power);*/
    ///////////////////////////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //oxygen
    let mut o2 = lines_vec.clone();
    for (place, value) in out.iter().rev().enumerate() {
        println!("\nthere are {} lines in o2", o2.len());
        if o2.len() == 1 {
            break;
        }
        println!(
            "retaining all lines with {} in the {} place",
            *value as i32, place
        );

        o2.retain(|line| ((line & masks[place] as usize) != 0) == *value);
    }

    println!("after filtering, there are {} lines", o2.len());
    let o2_rating = o2.pop().unwrap();
    println!("o2 rating is {:b} or {}", o2_rating, o2_rating);

    //co2
    let mut co2 = lines_vec.clone();
    for (place, value) in out.iter().rev().enumerate() {
        println!("there are {} lines in co2", co2.len());
        if co2.len() == 2 {
            break;
        }
        println!(
            "\nretaining all lines with {} in the {} place",
            (!value) as i32,
            place
        );

        co2.retain(|line| ((line & masks[place] as usize) != 0) == !value);
    }
    println!("after filtering, there are {} lines", co2.len());
    println!("{:?}", co2);
    let co2_rating = co2.pop().unwrap();
    println!("co2 rating is {:b} or {}", co2_rating, co2_rating);
    ///////////////////////////////////////////////////////////////////////////////////////////////
}
