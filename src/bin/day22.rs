//Christopher Hinson
//notes:
//command regex: (on|off)\sx=(-*\d+)..(-*\d+),y=(-*\d+)..(-*\d+),z=(-*\d+)..(-*\d+)
//https://regex101.com/r/J1HTtz/1

use regex::Regex;
use std::fs;

fn main() {
    let filename = "./inputs/input22.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let command_regex =
        Regex::new(r"(on|off)\sx=(-*\d+)..(-*\d+),y=(-*\d+)..(-*\d+),z=(-*\d+)..(-*\d+)").unwrap();

    let commands = contents
        .lines()
        .map(|line| {
            let mut caps = command_regex
                .captures(line)
                .unwrap()
                .iter()
                .map(|cap| cap.unwrap().as_str())
                .collect::<Vec<&str>>();
            //remove the full string capture group
            caps.remove(0);
            return caps;
        })
        .map(|mut command| {
            let command_type = command.remove(0);
            (
                command_type,
                command
                    .iter()
                    .map(|val| i32::from_str_radix(val, 10).unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect::<Vec<(&str, Vec<i32>)>>();

    //initialize our 100x100x100 reactor to all false;
    let reactor: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 100]; 100]; 100];

    for command in commands {
        //dont consider this command if its not within our reactor
        if !in_bounds(&command) {
            continue;
        }

        println!("processing: {:?}", command);
    }

    //println!("points in on x=10..12,y=10..12,z=10..12 ");
    //let points = gen_points((10, 12, 10, 12, 10, 12));

    //println!("{:?}", points);
}

fn in_bounds(command: &(&str, Vec<i32>)) -> bool {
    let mut overlapping = true;

    //x plane overlap
    if 50 > command.1[0] && -50 < command.1[1] {
        overlapping = false;
    }
    //y plane overlap
    if 50 > command.1[2] && -50 < command.1[3] {
        overlapping = false;
    }
    //z plane overlap
    if 50 > command.1[4] && -50 < command.1[5] {
        overlapping = false;
    }

    overlapping
}

fn gen_points(bounds: (i32, i32, i32, i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    //these are our cubit's bounds
    let x1 = bounds.0;
    let x2 = bounds.1;
    let y1 = bounds.2;
    let y2 = bounds.3;
    let z1 = bounds.4;
    let z2 = bounds.5;
    let mut ret_vec = Vec::new();

    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            for z in z1..z2 + 1 {
                println!("    {x},{y},{z}");
                ret_vec.push((x, y, z));
            }
        }
    }

    return ret_vec;
}
