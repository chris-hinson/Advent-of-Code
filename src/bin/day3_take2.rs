//Christopher Hinson
//notes: taking each line as a string instead of a usize should simplify our logic
//what a fucked day. instructions fucked for part 2

use std::fs;

fn main() {
    let filename = "input3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.lines().collect();
    let mut inv_lines: Vec<&str> = Vec::new();

    //println!("lines: {:?}", lines);
    let mut ones: Vec<i32> = vec![0; lines[0].len()];
    let mut zeros: Vec<i32> = vec![0; lines[0].len()];

    //counting
    lines.iter_mut().for_each(|line| {
        line.to_owned()
            .chars()
            .enumerate()
            .for_each(|(index, char)| {
                if char.eq(&'1') {
                    ones[index] += 1
                } else {
                    zeros[index] += 1;
                }
            })
    });

    println!("ones:{:?}\nzero:{:?}\n", ones, zeros);

    let most_common = zeros
        .iter()
        .zip(ones.iter())
        .map(|pair| if pair.0 > pair.1 { "0" } else { "1" })
        .collect::<String>();

    let least_common = zeros
        .iter()
        .zip(ones.iter())
        .map(|pair| if pair.0 > pair.1 { "1" } else { "0" })
        .collect::<String>();

    let epsilon = usize::from_str_radix(&most_common, 2).unwrap()
        * usize::from_str_radix(&least_common, 2).unwrap();

    //day 1 solution
    println!(
        "gamma: {}\ngamma: {}\nepsilon:{}\n",
        most_common, least_common, epsilon
    );

    //day 2
    //first, lets figure out our keep criteria for each positon

    println!(
        "part 2\nmost common: {}\nleast common:{}\n",
        most_common, least_common
    );

    //oxygen and co2 copies of lines
    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();
    //final oxy and c02 values
    let mut oxy_final = "";
    let mut co2_final = "";
    //sentinel values for telling when we're done with co2 and oxy
    let mut oxy_done = false;
    let mut co2_done = false;

    //main loop
    for i in 0..lines[0].len() {
        //first we decide which value to retain based on
        let mut oxy_ones = 0;
        let mut oxy_zeros = 0;

        oxygen.iter().for_each(|line| {
            if line.chars().collect::<Vec<char>>()[i].eq(&'0') {
                oxy_zeros += 1
            } else {
                oxy_ones += 1
            }
        });

        let oxy_keep = if oxy_zeros == oxy_ones {
            '1'
        } else if oxy_zeros > oxy_ones {
            '0'
        } else {
            '1'
        };

        let mut co2_ones = 0;
        let mut co2_zeros = 0;

        co2.iter().for_each(|line| {
            if line.chars().collect::<Vec<char>>()[i].eq(&'0') {
                co2_zeros += 1
            } else {
                co2_ones += 1
            }
        });

        let co2_keep = if co2_zeros == co2_ones {
            '0'
        } else if co2_zeros > co2_ones {
            '1'
        } else {
            '0'
        };

        println!(
            "on digit {}, we have {} oxy_zeros and {} oxy_ones\nwe have {} co2_zeros and {} co2_ones\nWe will keep {}s for oxygen and {}s for co2",
            i, oxy_zeros, oxy_ones,co2_zeros,co2_ones, oxy_keep, co2_keep
        );

        //retain our oxygen
        oxygen.retain(|line| line.chars().collect::<Vec<char>>()[i] == oxy_keep);
        if oxygen.len() == 1 && !oxy_done {
            oxy_final = oxygen[0];
            oxy_done = true;
        }

        println!(
            "on digit {}, after filtering oxygen , we have {:?}",
            i, oxygen
        );

        //retain our co2
        co2.retain(|line| line.chars().collect::<Vec<char>>()[i] == co2_keep);
        if co2.len() == 1 && !co2_done {
            co2_final = co2[0];
            co2_done = true;
        }

        println!("on digit {}, after filtering co2 , we have {:?}\n", i, co2);
    }

    let life_support =
        usize::from_str_radix(oxy_final, 2).unwrap() * usize::from_str_radix(co2_final, 2).unwrap();
    println!(
        "oxy_final:{}\nco2_final:{}\nlife support rating: {:?}",
        oxy_final, co2_final, life_support
    );
}
