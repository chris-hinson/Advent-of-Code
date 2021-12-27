//Christopher Hinson
//notes:only 2 axes? and no sin/cos calculations?? damn, missed out on a fun puzzle

use std::fs;

fn main() {
    let filename = "input2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let commands: Vec<(&str, i32)> = contents
        .lines()
        .map(|command| command.split(" ").collect::<Vec<_>>())
        .map(|command| (command[0], command[1].parse::<i32>().unwrap()))
        .collect();

    //println!("commands 1-5 are: {:?}", &commands[0..6]);

    //part 1///////////////////////////////////////////////////////////////////////////////////////
    let mut horizontal = 0;
    let mut vertical = 0;
    //parse commands
    for command in &commands {
        match command.0 {
            "up" => vertical -= command.1,
            "down" => vertical += command.1,
            "forward" => horizontal += command.1,
            _ => println!("bad command??"),
        }
    }

    println!(
        "part 1 final values:\nhorizontal:{}\nvertical:{}\nmultiplied together:{}\n\n",
        horizontal,
        vertical,
        horizontal * vertical
    );
    ///////////////////////////////////////////////////////////////////////////////////////////////

    //part 2///////////////////////////////////////////////////////////////////////////////////////
    horizontal = 0;
    vertical = 0;
    let mut aim = 0;

    //parse commands
    for command in &commands {
        match command.0 {
            "up" => aim -= command.1,
            "down" => aim += command.1,
            "forward" => {
                horizontal += command.1;
                vertical += aim * command.1
            }
            _ => println!("bad command??"),
        }
    }

    println!(
        "part 2 final values:\nhorizontal:{}\nvertical:{}\nmultiplied together:{}",
        horizontal,
        vertical,
        horizontal * vertical
    );

    ///////////////////////////////////////////////////////////////////////////////////////////////
}
