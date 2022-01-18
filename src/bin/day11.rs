//Christopher Hinson
//notes:

use std::fs;

fn print_vec(dvec: &Vec<Vec<u32>>) {
    for row in dvec {
        for cell in row {
            print!("{cell}");
        }
        println!("");
    }
    println!("");
}

fn main() {
    let filename = "./inputs/input11.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut octo: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("Before any steps:");
    print_vec(&octo);

    //need to keep track of both our steps and total flashes explicitly
    let mut steps = 0;
    let mut total_flashes = 0;
    //so long as the sum of the board is not 0, keep iterating
    while octo
        .iter_mut()
        .map(|line| line.iter().sum::<u32>())
        .sum::<u32>()
        != 0
    {
        steps += 1;
        step_a(&mut octo);

        total_flashes += step_b(&mut octo);
        println!("After step {steps}:");
        print_vec(&octo);
    }
    println!("total flashes: {total_flashes}");
}

//this function simply increases each value by one
fn step_a(octo: &mut Vec<Vec<u32>>) {
    for row in octo {
        for col in row {
            *col += 1;
        }
    }
}

fn step_b(octo: &mut Vec<Vec<u32>>) -> i32 {
    //first lets find all the 9s
    let mut flash: Vec<(usize, usize)> = Vec::new();

    for (i, row) in octo.iter().enumerate().rev() {
        for (j, col) in row.iter().enumerate().rev() {
            if *col > 9 {
                //num_flash += 1;
                flash.push((i, j))
            }
        }
    }

    //println!("flashes: {:?}", flash);

    let mut visited: Vec<(usize, usize)> = Vec::new();

    //increase the surrounding energy level for all that flashed
    while !flash.is_empty() {
        //get our next flasher and add it to the visited vec
        let cur = flash.pop().unwrap();
        visited.push(cur.clone());
        //println!("flashing: {:?}", cur);

        //get the moore neighbors of the current octo
        let neighbors = moore_neighbors(octo, cur);

        //for every neighbor, increment it and if this increment puts it above 9 and we havent flashed it yet this step, add it to the flash list
        let mut neighbors_flash: Vec<(usize, usize)> = Vec::new();
        for cell in neighbors {
            //print!("\tincr neigh: {:?}", cell);
            octo[cell.0][cell.1] += 1;
            //println!("it's now:{}", octo[cell.0][cell.1]);
            if octo[cell.0][cell.1] > 9
                && !flash.contains(&(cell.0, cell.1))
                && !visited.contains(&(cell.0, cell.1))
            {
                neighbors_flash.push((cell.0, cell.1));
                //println!("\t\tADDING {:?} TO FLASH LIST", (cell.0, cell.1));
            }
        }
        flash.append(&mut neighbors_flash);
    }

    //finally, reset any flashers to 0
    for (i, row) in octo.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if *col > 9 {
                *col = 0;
            }
        }
    }
    return visited.len() as i32;
}

//takes a field and (x,y) coords and returns a vec of all its valid moore neighborhood neighbors (i.e. deals with board edges)
fn moore_neighbors(octo: &mut Vec<Vec<u32>>, cell: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    let row = cell.0;
    let col = cell.1;

    let height = octo.len();
    let width = octo[0].len();

    //NW
    if row > 0 && col > 0 {
        neighbors.push((row - 1, col - 1))
    }
    //N
    if row > 0 {
        neighbors.push((row - 1, col))
    }
    //NE
    if row > 0 && col < width - 1 {
        neighbors.push((row - 1, col + 1))
    }
    //E
    if col < width - 1 {
        neighbors.push((row, col + 1))
    }
    //SE
    if row < height - 1 && col < width - 1 {
        neighbors.push((row + 1, col + 1))
    }
    //S
    if row < height - 1 {
        neighbors.push((row + 1, col))
    }
    //SW
    if row < height - 1 && col > 0 {
        neighbors.push((row + 1, col - 1))
    }
    //W
    if col > 0 {
        neighbors.push((row, col - 1))
    }

    return neighbors;
}
