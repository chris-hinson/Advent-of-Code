//Christopher Hinson
//notes
//canceled at 99999975672193...INVALID searching downwards from 99999999999999
//average time to test a code: 98404.72454268293ns

use rayon::prelude::*;
use std::fs;
use std::sync::{Arc, Mutex};

struct Cpu {
    //w,x,y,z
    regs: [i128; 4],
    input_buffer: Vec<u32>,
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "regs:{:?}\ninput stack:{:?}",
            self.regs, self.input_buffer
        )
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            regs: [0; 4],
            input_buffer: Vec::new(),
        }
    }

    //takes an arg and either returns either the imm or the value of the reg
    fn parse_arg(&mut self, reg: &str) -> Result<i128, &str> {
        match reg.parse::<i128>() {
            Ok(val) => Ok(val),
            Err(_) => match reg {
                "w" => Ok(self.regs[0]),
                "x" => Ok(self.regs[1]),
                "y" => Ok(self.regs[2]),
                "z" => Ok(self.regs[3]),
                _ => Err("not an imm and not a valid reg!"),
            },
        }
    }

    fn execute(&mut self, op: Vec<&str>) {
        match op[0] {
            "inp" => {
                //we know this will always be a variable name
                //println!("inp {}", op[1]);
                let in_val = self.input_buffer.pop().unwrap();

                match op[1] {
                    "w" => self.regs[0] = in_val as i128,
                    "x" => self.regs[1] = in_val as i128,
                    "y" => self.regs[2] = in_val as i128,
                    "z" => self.regs[3] = in_val as i128,
                    _ => println!("BAD REG!"),
                }
            }
            "add" => {
                //println!("add {} {}", op[1], op[2]);

                //we know the first arg will be a register
                match op[1] {
                    "w" => self.regs[0] += self.parse_arg(op[2]).unwrap(),
                    "x" => self.regs[1] += self.parse_arg(op[2]).unwrap(),
                    "y" => self.regs[2] += self.parse_arg(op[2]).unwrap(),
                    "z" => self.regs[3] += self.parse_arg(op[2]).unwrap(),
                    _ => println!("BAD REG"),
                }
            }
            "mul" => {
                //println!("mul {} {}", op[1], op[2]);

                //we know the first arg will be a register
                match op[1] {
                    "w" => self.regs[0] *= self.parse_arg(op[2]).unwrap(),
                    "x" => self.regs[1] *= self.parse_arg(op[2]).unwrap(),
                    "y" => self.regs[2] *= self.parse_arg(op[2]).unwrap(),
                    "z" => self.regs[3] *= self.parse_arg(op[2]).unwrap(),
                    _ => println!("BAD REG"),
                }
            }
            "div" => {
                //println!("div {} {}", op[1], op[2]);

                //we know the first arg will be a register
                match op[1] {
                    "w" => self.regs[0] /= self.parse_arg(op[2]).unwrap(),
                    "x" => self.regs[1] /= self.parse_arg(op[2]).unwrap(),
                    "y" => self.regs[2] /= self.parse_arg(op[2]).unwrap(),
                    "z" => self.regs[3] /= self.parse_arg(op[2]).unwrap(),
                    _ => println!("BAD REG"),
                }
            }
            "mod" => {
                //println!("mod {} {}", op[1], op[2]);

                //we know the first arg will be a register
                match op[1] {
                    "w" => self.regs[0] = self.parse_arg(op[2]).unwrap(),
                    "x" => self.regs[1] = self.parse_arg(op[2]).unwrap(),
                    "y" => self.regs[2] = self.parse_arg(op[2]).unwrap(),
                    "z" => self.regs[3] = self.parse_arg(op[2]).unwrap(),
                    _ => println!("BAD REG"),
                }
            }
            "eql" => {
                //println!("eql {} {}", op[1], op[2]);
                //we know the first arg will be a register
                match op[1] {
                    "w" => {
                        self.regs[0] = if self.regs[0] == self.parse_arg(op[2]).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    "x" => {
                        self.regs[1] = if self.regs[1] == self.parse_arg(op[2]).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    "y" => {
                        self.regs[2] = if self.regs[2] == self.parse_arg(op[2]).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    "z" => {
                        self.regs[3] = if self.regs[3] == self.parse_arg(op[2]).unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => println!("BAD REG"),
                }
            }
            _ => println!("BAD OP"),
        }
    }

    //fill the input buffer with an input digit so we can pop off the top to get the next val
    fn fill_input(&mut self, input: &str) {
        let digits = input
            //.to_string()
            .chars()
            .rev()
            .map(|digit| digit.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        digits
            .iter()
            .for_each(|value| self.input_buffer.push(*value));
    }
}

fn main() {
    let filename = "input24.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let program = contents
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut good_codes: Vec<String> = Vec::new();

    //let codes: Vec<String> = gen_codes();

    //let upper: u128 = 99999999999999;
    //for code in 00000000000000..upper {
    //for code in codes {

    let mut times: Vec<u128> = Vec::new();

    //
    //let low: u128 = 11111111111111;
    let low: u128 = 99999999991111;
    let high: u128 = 99999999999999;
    (low..high).into_par_iter().for_each(|code| {
        let start = std::time::Instant::now();
        if code.to_string().contains('0') {
            continue;
        }
        if code
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .contains(&'0')
        {
            continue;
        } else {
            //print!("testing code: {}...", code);
            let good_code = validate(&code.to_string(), (*program).to_vec());
            if good_code {
                println!("VALID");
                //good_codes.push(code)
                break;
            } else {
                //println!("INVALID");
            }
        }
        let end = std::time::Instant::now();
        let dur = end.duration_since(start);
        times.push(dur.as_nanos());
        //println!("took {}ns", dur.as_nanos());
    });

    let ave_time: f64 = times.iter().sum::<u128>() as f64 / times.len() as f64;
    println!("average time to test a code: {}ns", ave_time);
}

fn validate(code: &str, program: Vec<Vec<&str>>) -> bool {
    let mut cpu = Cpu::new();
    cpu.fill_input(code);

    for opcode in program {
        cpu.execute(opcode);
    }

    if cpu.regs[3] == 0 {
        true
    } else {
        false
    }
}

/*fn gen_codes() -> Vec<String> {
    let mut codes = Vec::new();

    //codes.push("13579246899999".to_owned());
    let low: u128 = 11111111111111;
    let high: u128 = 99999999999999;
    for number in (low..high).into_iter().rev() {
        if number.to_string().contains('0') {
            continue;
        }
        println!("{}", number);
    }

    codes
}*/
