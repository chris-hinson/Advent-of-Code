//Christopher Hinson
//notes

use std::fs;

struct Point {
    x: usize,
    y: usize,
    val: u32,
    up: Option<u32>,
    right: Option<u32>,
    down: Option<u32>,
    left: Option<u32>,
    marked: bool,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            val: 0,
            up: None,
            right: None,
            down: None,
            left: None,
            marked: false,
        }
    }
}

fn main() {
    let filename = "input9_dummy.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let heightmap: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    //get a collection of the low points
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (i, row) in heightmap.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if test_low(heightmap.clone(), (i, j)) {
                low_points.push((i, j));
            }
        }
    }

    println!("low points are at: {:?}", low_points);

    //iterate over low points to get their rist levels
    let risks = low_points
        .iter_mut()
        .map(|point| heightmap[point.0][point.1] + 1)
        .collect::<Vec<u32>>();
    println!("therefore risks are: {:?}", risks);

    let risk_tot: u32 = risks.iter().sum();
    println!("risk total: {}", risk_tot);
}

fn test_low(map: Vec<Vec<u32>>, test_point: (usize, usize)) -> bool {
    let row = test_point.0;
    let col = test_point.1;
    let point_val = map[row][col];
    let mut low_point = true;

    /*println!(
        "surrounding points are\nup:{:?}\ndown:{:?}\nleft:{:?}\nright:{:?}\n",
        up, down, left, right
    );*/

    let surroundings = get_surrounding(map, test_point);
    let up = surroundings.0;
    let right = surroundings.1;
    let down = surroundings.2;
    let left = surroundings.3;

    //we are at a low point if all non-None values around us are greater than us
    if left != None {
        if left.unwrap() <= &point_val {
            //println!("left is less");
            low_point = false;
        }
    }
    if right != None {
        if right.unwrap() <= &point_val {
            //println!("right is less");
            low_point = false;
        }
    }
    if up != None {
        if up.unwrap() <= &point_val {
            //println!("up is less");
            low_point = false;
        }
    }
    if down != None {
        if down.unwrap() <= &point_val {
            //println!("down is less");
            low_point = false;
        }
    }

    return low_point;
}

fn find_basin(map: Vec<Vec<u32>>, low_point: (usize, usize)) -> i32 {
    let row = low_point.0;
    let col = low_point.1;
    //lets get a version of the map where each point has a bool associated with it so we can mark them
    let mut marking_map = map
        .clone()
        .iter()
        .map(|line| {
            line.iter()
                .map(|col| (*col, false))
                .collect::<Vec<(u32, bool)>>()
        })
        .collect::<Vec<Vec<(u32, bool)>>>();

    let cur_point = low_point;
    marking_map[row][col].1 = true;

    //while

    0
}

fn get_surrounding(
    map: Vec<Vec<u32>>,
    test_point: (usize, usize),
) -> (
    Option<&'static u32>,
    Option<&'static u32>,
    Option<&'static u32>,
    Option<&'static u32>,
) {
    let mut low_point = true;
    let row = test_point.0;
    let col = test_point.1;

    let point_val = map[row][col];
    //println!("testing point: ({},{}), val:{}", row, col, point_val);

    //get our surrounding values, leaving a None if out of bounds

    //left = row,col-1
    let left = if col == 0 {
        None
    } else {
        match &map.get(row) {
            Some(row) => row.get(col - 1),
            None => None,
        }
    };

    //right = row,col+1
    let right = match &map.get(row) {
        Some(row) => row.get(col + 1),
        None => None,
    };

    //up = row-1, col
    let up = if row == 0 {
        None
    } else {
        match &map.get(row - 1) {
            Some(row) => row.get(col),
            None => None,
        }
    };

    //down = row+1, col
    let down = match &map.get(row + 1) {
        Some(row) => row.get(col),
        None => None,
    };

    return (up, right, down, left);
}
