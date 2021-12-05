use std::env;
use std::fmt;
use std::fs;

pub struct Board {
    board: Vec<Vec<(i32, bool)>>,
    score: i32,
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //write!(f, "{}", self.board)

        let mut out_string: String = "".to_owned();

        /*let out_board = self
        .board
        .iter()
        .flat_map(|x| x.iter().map(|y| y.0))
        .collect::<Vec<i32>>();*/

        for row in self.board.iter() {
            for col in row {
                out_string.push_str(&(col.0).to_string());
                out_string.push_str(",");
                out_string.push_str(&(col.1).to_string());
                out_string.push_str(" ");
            }
            out_string.push_str("\n");
        }

        write!(f, "{}", out_string)
    }
}

impl Board {
    pub fn new(board: Vec<Vec<i32>>) -> Board {
        //we want to add a marked bool to each number in the board
        let mut marked_board: Vec<Vec<(i32, bool)>> =
            vec![vec![(0, false); board[0].len()]; board.len()];

        for (i, line) in board.iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                marked_board[i][j] = (*col, false);
            }
        }

        Board {
            board: marked_board,
            score: 0,
        }
    }

    //take a value and if the board contains it, mark it and return true, otherwise reutn false
    pub fn mark(&mut self, value: i32) -> bool {
        for (i, line) in self.board.iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                if col.0 == value {
                    self.board[i as usize][j as usize] = (col.0, true);
                    return true;
                }
            }
        }

        return false;
    }

    pub fn done(&mut self) -> bool {
        let new_board = self.board.clone();

        let col_exists = new_board[0]
            .iter()
            .enumerate()
            //we are taking an iterator over the first row
            //and turning it into an iterator of board columns
            //NOTE: THIS ONLY WORKS ON SQUARE MATRICES
            .map(|col| {
                new_board
                    //flatten our board
                    .iter()
                    .flat_map(|x| x.iter().map(|y| y.1))
                    //skip the first col items in the flattened iter
                    .skip(col.0)
                    //step by the size of a row to get a full col
                    .step_by(new_board[0].len())
                    .collect::<Vec<bool>>()
            })
            //now that we have our column vectors
            //try and find one containing only trues
            .find(|y| y.iter().all(|z| *z));
        let col_found = match col_exists {
            Some(_v) => true,
            None => false,
        };

        let row_exists = new_board.iter().find(|x| x.iter().all(|y| y.1));
        let row_found = match row_exists {
            Some(_v) => true,
            None => false,
        };

        return row_found || col_found;
    }

    pub fn tabulate_score(&mut self, last_num: i32) -> i32 {
        //score is defined as the sum of all unmarked numbers, multiplied by the last number called

        let unmarked_total = self.board.iter().fold(0, |total, row| {
            total
                + row
                    .iter()
                    .filter(|item| !(item.1))
                    .fold(0, |row_tot, col| row_tot + (col.0))
        });

        return unmarked_total * last_num;
    }
}

fn main() {
    let filename = "input4.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //lines of the input file
    let mut lines: Vec<&str> = contents.lines().collect();

    //THIS HAS BAD TIME COMPLEXITY
    let bingo_string = lines.remove(0);
    lines.remove(0);
    println!("BINGO STRING: {}", bingo_string);
    let bingo_vec = bingo_string
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();

    //split into individual boards
    for (_board_num, board_contents) in lines.chunks(6).enumerate() {
        //board contents comes in as a slice of str slices
        //println!("board is: {:?}", board_contents);

        let mut cur_board: Vec<Vec<i32>> = Vec::new();

        //iterate over lines of the board, ignoring the empty line
        for (line_num, line) in board_contents.iter().filter(|x| !x.is_empty()).enumerate() {
            //line is a string, lets split it on spaces,
            //println!("line {} is {:?}", line_num, line);
            let line_contents: Vec<&str> = line.split(" ").collect();
            let mut cur_line: Vec<i32> = Vec::new();

            //iterate over the values of the split, ignoring any that cannot be parsed into an i32
            for value in line_contents {
                let num = value.parse::<i32>();

                match num {
                    Ok(v) => {
                        //println!("{:?}", num);
                        cur_line.push(num.unwrap());
                    }
                    Err(e) => {}
                }
            }

            cur_board.push(cur_line);
        }

        //println!("pushing board: {:?}", cur_board);
        boards.push(Board::new(cur_board));
    }

    /*println!(
        "There are {} boards, board 0 looks like {:?}",
        boards.len(),
        boards[0].board
    );

    let check_val = boards[0].board[0][0].0;

    println!(
        "does board 0 contain its 0,0th char? {}",
        boards[0].mark(check_val)
    );

    println!("board 0 now looks like: {:?}", boards[0].board);

    println!("testing done conditions");
    println!("testing row");
    for i in 0..5 {
        println!("marking board 0's 0,{}th char", i);
        let check_val = boards[0].board[0][i].0;
        boards[0].mark(check_val);
    }

    println!(
        "marked board 0's 0th row, it now looks like {:?}",
        boards[0].board
    );

    println!(
        "board 0 should now be a winner. is it? {}",
        boards[0].done()
    );

    println!("testing col");
    println!("board 1 looks like:\n {}", boards[1]);

    for i in 0..5 {
        println!("marking board 1's {},0th char", i);
        let check_val = boards[1].board[i][0].0;
        boards[1].mark(check_val);
    }

    println!(
        "marked board 1's 0th col, it now looks like \n{}",
        boards[1]
    );

    println!(
        "board 1 should now be a winner. is it? {}",
        boards[1].done()
    );*/

    for number in bingo_vec {
        println!("calling {}", number);
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(number);

            if board.done() {
                print!(
                    "board {} wins, it looks like \n{}\n with a score of ",
                    i, board
                );
                println!("{}", board.tabulate_score(number));
                boards.remove();
                //return;
            }
        }
    }
}
