//Christopher Hinson
//notes:STACKS MOTHERFUCKER, STACKS!!

use std::fs;

fn main() {
    let filename = "input10_KEVIN.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines = contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let indexing_lines = lines.clone();

    //day 1 scoring var
    let mut score: i32 = 0;
    //let mut corrupted: Vec<Vec<char>> = Vec::new();

    println!("lines before filtering: {}", lines.len());

    lines.retain(|line| {
        let mut stack: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for (char_index, char) in line.iter().enumerate() {
            //if we have an opening delimiter, push it on the stack
            if char.eq(&'(') || char.eq(&'{') || char.eq(&'[') || char.eq(&'<') {
                stack.push(*char);
            }
            //otherwise, pop stack and compare
            //if we dont have a matching pair, CORRUPTED LINE
            else {
                let comp = stack.pop().unwrap();
                is_corrupted = match char {
                    ')' => !comp.eq(&'('),
                    '}' => !comp.eq(&'{'),
                    ']' => !comp.eq(&'['),
                    '>' => !comp.eq(&'<'),
                    _ => true,
                };

                //if we have a corrupted line, update our score
                //break the line iteration
                if is_corrupted {
                    //corrupted.push(line.to_vec());
                    print!(
                        "line: {}\n{} ",
                        indexing_lines.iter().position(|ele| ele == line).unwrap() + 1,
                        line.iter().collect::<String>()
                    );

                    match char {
                        ')' => {
                            score += 3;
                            print!("corrupted on char: {} ", char_index + 1);
                            println!(" adding 3, total is now {}", score);
                        }
                        '}' => {
                            score += 1197;
                            print!("corrupted on char: {} ", char_index + 1);
                            println!("adding 1197, total is now {}", score);
                        }
                        ']' => {
                            score += 57;
                            print!("corrupted on char: {} ", char_index + 1);
                            println!("adding 57, total is now {}", score);
                        }
                        '>' => {
                            score += 25137;
                            print!("corrupted on char: {} ", char_index + 1);
                            println!("adding 25137, total is now {}", score);
                        }
                        _ => (),
                    };

                    break;
                }
            }
        }
        !is_corrupted
    });
    println!("lines after filtering: {}", lines.len());

    println!("score: {}\n", score);

    //day 2
    println!("day 2");

    //iterate over the remaining lines (these lines should only be incomplete, not corrupted
    //and turn the line into the stack remainder
    lines = lines
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();

            for char in line.iter() {
                //if we have an opening delimiter, push it on the stack
                if char.eq(&'(') || char.eq(&'{') || char.eq(&'[') || char.eq(&'<') {
                    stack.push(*char);
                } else {
                    //if its a closing delimiter, pop off the stack
                    //we dont have to do anything with this because we know we dont have corrupted lines
                    let _comp = stack.pop().unwrap();
                }
            }

            //println!("remaining stack: {:?}", stack);
            stack
        })
        .collect::<Vec<Vec<char>>>();

    println!("rem stacks: {:?}", lines);

    //now turn each stack remainder into its inverse reverse (match)
    lines = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|char| match char {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    //this should never happen
                    _ => '_',
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    println!("rem stack inv 1: {:?}", lines[0]);

    lines.iter_mut().for_each(|line| line.reverse());

    println!("rem stack inv rev 1: {:?}", lines[0]);

    //now lets calculate scores
    let mut score = lines
        .iter()
        .map(|line| {
            let mut score: u128 = 0;
            line.iter().for_each(|char| {
                score *= 5;
                match char {
                    ')' => score += 1,
                    ']' => score += 2,
                    '}' => score += 3,
                    '>' => score += 4,
                    _ => score += 999,
                }
            });
            score
        })
        .collect::<Vec<u128>>();

    println!("scores: {:?}", score);

    //now sort them
    score.sort();

    println!("sort scores: {:?}", score);

    println!(
        "there are {} scores, middle score is the {}th at {}",
        score.len(),
        score.len() / 2,
        score[score.len() / 2]
    );
}
