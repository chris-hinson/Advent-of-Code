//Christopher Hinson
//notes: this is incredibly inneficient. brute forcing is not the best solution here

use std::fs;

fn main() {
    let filename = "input5.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input_lines: Vec<&str> = contents.lines().collect();

    //println!("first 5 lines: {:?}", &input_lines[0..5]);

    //vector of vectors of points
    let lines: Vec<Vec<(i32, i32)>> = input_lines
        .iter()
        //split the input line on its ->
        //this leaves some whitespace
        //collect back into a vec of &str, so we now have a vec of vecs
        .map(|x| x.split("->").collect::<Vec<&str>>())
        //parse each point
        .map(|y| {
            y.iter()
                .map(|z| {
                    z.split(",")
                        .into_iter()
                        //gets rid of our whitespace and turns each numver into an i32
                        .map(|a| a.replace(" ", "").parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                //turn each point into a tuple rather than a vec
                .map(|b| (b[0], b[1]))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    println!("first lines: {:?}", &lines[0..5]);

    //this is about to have absolutely terrible time complexity

    //lets first generate every point of every line
    let line_points: Vec<Vec<(i32, i32)>> = lines
        .iter()
        .map(|line| pointsGen(line[0], line[1]))
        .collect::<Vec<Vec<(i32, i32)>>>();

    //println!("size of all points: {}", all_points.len());
    //println!("full points: {:?}", all_points[0]);

    //then we'll make an vector where we keep the point and its number of occurences as a (point,i32) tuple
    let mut occurences: Vec<((i32, i32), i32)> = Vec::new();

    //iterate over every line
    for line in line_points.iter() {
        //iterate over every point of our current line
        for point in line {
            //if occurences already has this point, increase its occurences
            //NOTE: this solution worked, but took ~5 minutes to process the input
            if occurences.iter().find(|x| x.0 == *point) != None {
                let position = occurences.iter().position(|y| y.0 == *point).unwrap();
                occurences[position].1 += 1;
            } else {
                //otherwise push this point initialized to 1 occurences
                occurences.push((*point, 1));
            }
        }
    }

    //finally, scan this vector for any points which occur more than once
    let mut crossings: Vec<((i32, i32), i32)> = Vec::new();
    for point in occurences {
        if point.1 >= 2 {
            println!("found crossing: {:?}", point);
            crossings.push(point);
        }
    }
    println!(
        "there are {} crossings with more than 2 lines",
        crossings.len()
    );
}

fn pointsGen(p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
    println!("calling pointsGen on p1:{:?}, p2:{:?}", p1, p2);

    let mut points: Vec<(i32, i32)> = Vec::new();

    //DAY 1 WE ONLY WANT TO GEN POINTS FOR STRAIGHT LINES
    /*if p1.0 != p2.0 && p1.1 != p2.1 {
        println!("not a straight line");
        return points;
    }*/

    ////hacky because we know we're only dealing with straight lines or lines at a 45 deg angle,but
    //if rise or run is non-zero, normalize it
    let y_delta = if (p2.1 - p1.1) != 0 {
        (p2.1 - p1.1) / (p2.1 - p1.1).abs()
    } else {
        p2.1 - p1.1
    };

    let x_delta = if (p2.0 - p1.0) != 0 {
        (p2.0 - p1.0) / (p2.0 - p1.0).abs()
    } else {
        p2.0 - p1.0
    };

    println!("x_delta: {}, y_delta:{}\n", x_delta, y_delta);

    let mut cur_point = p1;
    while cur_point.0 != p2.0 || cur_point.1 != p2.1 {
        //println!("pushing point: {:?}", cur_point);
        points.push(cur_point);

        cur_point.0 += x_delta;
        cur_point.1 += y_delta;
    }
    points.push(p2);

    //println!("returning these points: {:?}\n", points);
    return points;
}
