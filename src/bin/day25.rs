//Christopher Hinson
//notes

use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fish {
    None,
    East,
    South,
}
impl std::fmt::Display for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Fish::None => write!(f, "."),
            Fish::East => write!(f, ">"),
            Fish::South => write!(f, "V"),
        }
    }
}

struct Board {
    fishies: Vec<Vec<Fish>>,
}

impl Board {
    fn new(input_string: &str) -> Board {
        let fishies: Vec<Vec<Fish>> = input_string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '>' => Fish::East,
                        'v' => Fish::South,
                        '.' => Fish::None,
                        _ => Fish::None,
                    })
                    .collect::<Vec<Fish>>()
            })
            .collect::<Vec<Vec<Fish>>>();

        Board { fishies }
    }

    //move all fish of fish_type
    fn update_board(&mut self, fish_type: Fish) -> bool {
        //we need the initial board state
        let init_board = self.fishies.clone();

        let mut fish_moved = false;
        //let init_board = board.clone();

        for row in 0..self.fishies.len() {
            for col in 0..self.fishies[0].len() {
                let cur_fish = init_board[row][col];
                //dont consider the types of fish we arent considering
                //lol
                if Fish::None == cur_fish || fish_type != cur_fish {
                    continue;
                }

                //update the fish
                match fish_type {
                    Fish::East => {
                        //figure out whats on our right (with wrapping)
                        let right_coords: (usize, usize) = (
                            row,
                            match init_board[row].get(col + 1) {
                                Some(val) => col + 1,
                                None => 0,
                            },
                        );
                        //println!("looking at: {},{}\nE: {:?}", row, col, right_coords);

                        //if east is clear on the initial board, update it on the real board
                        if init_board[right_coords.0][right_coords.1] == Fish::None {
                            self.fishies[right_coords.0][right_coords.1] = Fish::East;
                            self.fishies[row][col] = Fish::None;

                            fish_moved = true;
                        }
                    }
                    Fish::South => {
                        //figure out whats directly down (with wrapping)
                        let down_coords: (usize, usize) = (
                            match init_board.get(row + 1) {
                                Some(val) => row + 1,
                                None => 0,
                            },
                            col,
                        );
                        //println!("looking at: {},{}\nS: {:?}", row, col, down_coords);

                        //if down is clear on the initial board, update it on the real board
                        if init_board[down_coords.0][down_coords.1] == Fish::None {
                            self.fishies[down_coords.0][down_coords.1] = Fish::South;
                            self.fishies[row][col] = Fish::None;

                            fish_moved = true;
                        }
                    }
                    //this should never happen
                    Fish::None => (),
                }
            }
        }

        return fish_moved;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in &self.fishies {
            for fish in line {
                write!(f, "{}", fish)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn main() {
    let filename = "./inputs/input25.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut board = Board::new(&contents);

    println!("initial board state:\n{}", board);

    let mut running = true;
    let mut step = 0;
    while running {
        let east_move = board.update_board(Fish::East);
        let south_move = board.update_board(Fish::South);
        step += 1;
        println!("step: {}\n{}", step, board);

        if !east_move && !south_move {
            running = false;
        }
    }
}
