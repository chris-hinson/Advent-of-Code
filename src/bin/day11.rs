//Christopher Hinson
//notes:

use std::fs;

fn print_vec(dvec: &Vec<Vec<u32>>) {
    for row in dvec {
        println!("{:?}", row);
    }
    println!("");
}

fn main() {
    let filename = "input11.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut octo: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    print_vec(&octo);

    step1(&mut octo);
    print_vec(&octo);
    step2(&mut octo);
    print_vec(&octo);
}

//this function simply increases each value by one
fn step1(octo: &mut Vec<Vec<u32>>) {
    for row in octo {
        for col in row {
            *col += 1;
        }
    }
}

fn step2(octo: &mut Vec<Vec<u32>>) {
    //first lets find all the 9s
    let mut flash: Vec<(usize, usize)> = Vec::new();

    let mut num_flash = 0;
    for (i, row) in octo.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col >= 9 {
                num_flash += 1;
                flash.push((i, j))
            }
        }
    }

    println!("flashes: {:?}", flash);

    //increase the surrounding energy level for all
    for octopus in flash {
        let row = octopus.0;
        let col = octopus.1;

        //only do north neighbors if row > 0
        if !(row == 0) {
            //NW
            match match octo.get_mut(row - 1) {
                Some(inner) => inner.get_mut(col - 1),
                None => None,
            } {
                Some(val) => *val += 1,
                None => (),
            }
            //N
            match match octo.get_mut(row - 1) {
                Some(inner) => inner.get_mut(col),
                None => None,
            } {
                Some(val) => *val += 1,
                None => (),
            }
            //NE
            match match octo.get_mut(row - 1) {
                Some(inner) => inner.get_mut(col + 1),
                None => None,
            } {
                Some(val) => *val += 1,
                None => (),
            }
        }

        //only do west neighbors if col is non-zero
        if !(col == 0) {
            //W
            match match octo.get_mut(row) {
                Some(inner) => inner.get_mut(col - 1),
                None => None,
            } {
                Some(val) => *val += 1,
                None => (),
            }
            //SW
            match match octo.get_mut(row + 1) {
                Some(inner) => inner.get_mut(col - 1),
                None => None,
            } {
                Some(val) => *val += 1,
                None => (),
            }
        }

        //E
        match match octo.get_mut(row) {
            Some(inner) => inner.get_mut(col + 1),
            None => None,
        } {
            Some(val) => *val += 1,
            None => (),
        }
        //S
        match match octo.get_mut(row + 1) {
            Some(inner) => inner.get_mut(col),
            None => None,
        } {
            Some(val) => *val += 1,
            None => (),
        }
        //SE
        match match octo.get_mut(row + 1) {
            Some(inner) => inner.get_mut(col + 1),
            None => None,
        } {
            Some(val) => *val += 1,
            None => (),
        }
    }
}
