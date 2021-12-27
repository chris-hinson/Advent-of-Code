//Christopher Hinson
//Notes
use regex::Regex;
use std::fs;

fn main() {
    let filename = "input13.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //parse our regex
    let point_regex = Regex::new(r"(\d+),(\d+)").unwrap();
    //clone our input since we are droppping some lines we want back later
    let points = contents.clone();
    //turn the single long string into a vec of strings
    let mut points: Vec<&str> = points.lines().collect::<Vec<&str>>();
    //only keep lines that match our regex
    points.retain(|line| point_regex.is_match(line));
    //turn the vec of strings (where each line is a valid point string) into a vec of u32 tuples
    let points: Vec<(u32, u32)> = points
        .iter()
        .map(|line| {
            let caps = point_regex.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();

    //println!("{:?}", points);

    //do the same shit for folds lol
    let fold_regex = Regex::new(r"\w+\s([x|y])=(\d+)").unwrap();
    let folds = contents.clone();
    let mut folds: Vec<&str> = folds.lines().collect::<Vec<&str>>();
    folds.retain(|line| fold_regex.is_match(line));
    let mut folds: Vec<(char, u32)> = folds
        .iter()
        .map(|fold| {
            let caps = fold_regex.captures(fold).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<char>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(char, u32)>>();
    println!("folds:{:?}", folds);

    let row_max = points.iter().map(|point| point.1).max().unwrap() + 1;
    let col_max = points.iter().map(|point| point.0).max().unwrap() + 1;

    println!("row_max:{},col_max:{}", row_max, col_max);

    let mut board: Vec<Vec<char>> = vec![vec!['.'; col_max as usize]; row_max as usize];

    for point in points {
        board[point.1 as usize][point.0 as usize] = '#';
    }
    print_board(&board);

    for (index, fold) in folds.iter_mut().enumerate() {
        board = fold_board(*fold, &board);
        println!("board after {} fold:", index);
        print_board(&board);
    }

    //day 1 solution
    /*let mut vis_dots = 0;
    board.iter().for_each(|row| {
        row.iter().for_each(|val| {
            if *val == '#' {
                vis_dots += 1;
            }
        })
    });
    println!("after first fold: {} dots visible", vis_dots);*/
}

fn print_board(board: &[Vec<char>]) {
    for line in board {
        println!("{}", line.iter().collect::<String>())
    }
    println!("");
}

fn fold_board(fold: (char, u32), board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    println!("folding along:{} = {}", fold.0, fold.1);
    match fold.0 {
        'x' => {
            println!("horiz fold");
            let mut left: Vec<Vec<char>> = board
                .iter()
                .map(|line| line.clone()[0..fold.1 as usize].to_vec())
                .collect::<Vec<Vec<char>>>();

            let mut right: Vec<Vec<char>> = board
                .iter()
                .map(|line| line.clone()[((fold.1) + 1) as usize..line.len() as usize].to_vec())
                .collect::<Vec<Vec<char>>>();

            print_board(&left);
            print_board(&right);

            //right half gets folded left
            let right = right
                .iter_mut()
                .map(|line| {
                    line.iter()
                        .map(|val| val.clone())
                        .rev()
                        .collect::<Vec<char>>()
                        .to_vec()
                })
                .collect::<Vec<Vec<char>>>();
            println!("folding left:");
            print_board(&left);
            print_board(&right);

            //interlace the two
            for (row, line) in left.iter_mut().enumerate() {
                for (col, char) in line.iter_mut().enumerate() {
                    let right_corr = right[row][col];
                    if *char == '.' && right_corr.eq(&'#') {
                        *char = '#';
                    }
                }
            }

            println!("new left: ");
            print_board(&left);

            left
        }
        'y' => {
            println!("vertical fold");
            let mut upper: Vec<Vec<char>> = board[0..(fold.1 as usize)].to_vec();
            let mut lower: Vec<Vec<char>> =
                board[((fold.1) + 1) as usize..(board.len()) as usize].to_vec();

            print_board(&upper);
            print_board(&lower);
            //bottom half gets folded up
            let lower: Vec<Vec<char>> = lower
                .iter_mut()
                .map(|val| val.clone())
                .rev()
                .collect::<Vec<Vec<char>>>();
            println!("folding up");
            print_board(&upper);
            print_board(&lower);

            //overlap the two
            for (row, line) in upper.iter_mut().enumerate() {
                for (col, char) in line.iter_mut().enumerate() {
                    let lower_corr = lower[row][col];

                    if *char == '.' && lower_corr.eq(&'#') {
                        *char = '#';
                    }
                }
            }
            println!("new upper:");
            print_board(&upper);
            upper
        }
        _ => board.clone(),
    }
    //board.to_vec().clone()
}
