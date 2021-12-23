//Christopher Hinson
//notes: damn i really wanted the parallelization solution to work, much cooler code
//cool how to simplify this algorithm though. Thanks cs1510

extern crate rayon;
use rayon::prelude::*;
use std::fs;
use std::sync::{Arc, Mutex};

fn main() {
    let filename = "input6.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut fish: Vec<i32> = contents
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|fish| fish.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("initial fish: {:?}", fish);

    let mut ages: Vec<u128> = vec![0; 9];

    for i in fish {
        ages[i as usize] += 1;
    }

    println!("initial fish by age: {:?}", ages);

    for day in 1..257 {
        ages = proc_day(&mut ages);
        println!("on day {} there are {:?} fish by age", day, ages);
    }

    println!("fish sum is {}", ages.iter().sum::<u128>());

    /*for day in 1..19 {
        fish = proc_day(&mut fish);
        //println!("day: {:?}, fish is: {:?}", day, fish);
        println!("after day: {:?} there are {:?} fish", day, fish.len())
    }*/
}

/*fn proc_day(fish: &mut [i32]) -> Vec<i32> {
    //let mut new_fish = Arc::new(Mutex::new(Vec::new()));
    //let mut new_fish = fish.to_vec().clone();
    let mut new_fish = Arc::new(Mutex::new(0));

    //let mut test_vec = fish
    let mut fish = fish
        .par_iter_mut()
        .enumerate()
        .map(|(_index, f)| {
            if *f == 0 {
                //new_fish.lock().unwrap()[index] = 6;
                //new_fish.lock().unwrap().push(8);
                *new_fish.lock().unwrap() += 1;
                6
            } else {
                //new_fish.lock().unwrap()[index] -= 1;
                *f - 1
            }
        })
        .collect::<Vec<i32>>();

    /*for (index, cur_fish) in fish.par_iter().enumerate() {
        if *cur_fish == 0 {
            new_fish[index] = 6;
            new_fish.push(8);
        } else {
            new_fish[index] -= 1;
        }
    }*/

    //return new_fish.try_unwrap().unwrap().into_inner().unwrap();
    //let val = new_fish.lock().unwrap();
    //return val.to_vec();

    //return new_fish.lock().unwrap().to_vec();

    let mut new_fish_vec = Vec::new();
    for i in 0..*new_fish.lock().unwrap() {
        new_fish_vec.push(8);
    }

    fish.append(&mut new_fish_vec);

    return fish;
}*/

fn proc_day(fish: &mut [u128]) -> Vec<u128> {
    let mut new_fish = vec![0; 9];

    new_fish[0] = fish[1];
    new_fish[1] = fish[2];
    new_fish[2] = fish[3];
    new_fish[3] = fish[4];
    new_fish[4] = fish[5];
    new_fish[5] = fish[6];
    new_fish[6] = fish[0] + fish[7];
    new_fish[7] = fish[8];
    new_fish[8] = fish[0];

    return new_fish;
}
