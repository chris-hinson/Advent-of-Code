//Christopher Hinson
//notes: using brute force here doesnt feel great, but off the top of my head i couldnt think of a
//spectacular algorithm for finding min average cost. oh well :(

use std::fs;

fn main() {
    let filename = "input7.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let crabs = contents
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        //.inspect(|crab| println!("{:?}", crab))
        .map(|crab| crab.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = crabs.iter().max().unwrap();
    let min = crabs.iter().min().unwrap();

    //println!("crabs: {:?}", crabs);

    println!("max crab: {}, min crab: {}", max, min);

    //lets try a weighted average
    /*let mut weights = vec![0; (*max + 1) as usize];
    for crab in crabs.iter() {
        weights[*crab as usize] += 1;
    }

    //println!("num of crabs at each location: {:?}", weights);

    let weighted_sum: usize = weights
        .iter()
        .enumerate()
        .map(|(index, weight)| (index + 1) * weight)
        .sum::<usize>();

    let sum_of_weights: usize = weights.iter().sum::<usize>();

    let ave: f32 = weighted_sum as f32 / sum_of_weights as f32;
    println!(
        "weighted ave is: {} this means we should move to index: {}",
        ave,
        ave - 1.0
    );*/

    //lets try a brute force approach
    let mut costs = vec![0; (max + 1) as usize];

    for (index, value) in costs.iter_mut().enumerate() {
        *value = compute_cost(&crabs, index as i32);
    }

    println!("costs: {:?}", costs);

    println!("min cost: {}", costs.iter().min().unwrap());
}

//day one cost
/*
fn compute_cost(pos: &[i32], target: i32) -> i32 {
    let mut total = 0;
    for i in pos {
        total += (target - *i).abs();
    }

    return total;
}*/
//day two cost
//this is a trianglular number
//i.e. the sequence goes 1, 3, 6, 10
fn compute_cost(pos: &[i32], target: i32) -> i32 {
    let mut total = 0;
    for i in pos {
        let distance = (target - i).abs();
        total += ((distance) * (distance + 1)) / 2
    }

    return total;
}
