//Christopher Hinson
//Notes

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input14_dummy.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut contents = contents.lines().collect::<Vec<&str>>();

    let mut polymer = contents.remove(0);
    let mut polymer = polymer.chars().collect::<Vec<char>>();
    contents.remove(0);

    println!("polymer is: {:?}", polymer);

    let insert_regex = Regex::new(r"(\w+)\s->\s(\w)").unwrap();
    let insertions: Vec<(Regex, String)> = contents
        .iter()
        .map(|insertion| {
            let caps = insert_regex.captures(insertion).unwrap();
            let find = caps.get(1).unwrap().as_str().chars().collect::<Vec<char>>();
            (
                Regex::new(&find.iter().collect::<String>()).unwrap(),
                //format!("{}{}{}", find[0], caps.get(2).unwrap().as_str(), find[1]),
                format!("{}", caps.get(2).unwrap().as_str()),
            )
        })
        .collect::<Vec<(Regex, String)>>();

    println!("insertions: {:?}", insertions);

    println!(
        "this is what a know-good regex looks like: {:?}",
        insert_regex
    );

    //we now have a vector of regexs and their replacement.
    //this means we can repeatedly apply every regex and replace accordingly
    //hopefully this has decent time complexity

    /*println!("initial polymer: {}", polymer);

    let mut new_polymer = "".to_owned();
    for index in 0..polymer.chars().collect::<Vec<char>>().len() - 1 {
        let char = polymer.chars().collect::<Vec<char>>()[index];
        let check_string = &polymer[index..index + 2 as usize];
        println!("checking: {}", check_string);

        for (regex, replacement_string) in &insertions {
            let rep = regex.replace(&check_string, replacement_string);
            //check_string = &rep;
            rep.chars().for_each(|c| new_polymer.push(c));
        }
    }

    println!("new polymer: {}", new_polymer);*/

    println!(
        "initial polymer is: {:?}",
        polymer.iter().collect::<String>()
    );

    for step in 1..5 {
        let mut new_polymer = "".to_owned();
        for index in 0..polymer.len() - 1 {
            let comp = format!("{}{}", polymer[index], polymer[index + 1]);
            //println!("checking: {}", comp);

            new_polymer.push(polymer[index]);
            for (regex, replacement) in &insertions {
                if regex.is_match(&comp) {
                    //println!("match on {:?}", regex);
                    //new_polymer.push(polymer[index]);
                    new_polymer.push(replacement.chars().collect::<Vec<char>>()[0]);
                }
            }
        }
        new_polymer.push(polymer.pop().unwrap());
        polymer = new_polymer.chars().collect::<Vec<char>>();

        /*println!(
            "polymer on step {} is: {:?}",
            step,
            polymer.iter().collect::<String>()
        );*/
        println!("step {} done", step);
    }

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut freqs: HashMap<char, usize> = HashMap::new();

    for char in alphabet.chars() {
        let freq = polymer.iter().filter(|a| *a == &char).count();
        freqs.insert(char, freq);
    }

    let max_freq = freqs.iter().map(|entry| entry.1).max().unwrap();
    let min_freq = freqs
        .iter()
        .filter(|entry| *entry.1 != 0)
        .map(|entry| entry.1)
        .min()
        .unwrap();

    println!("freq table of final polymer: \n{:?}", freqs);

    println!(
        "max_freq: {:?}\nmin_freq: {:?}\ndiff: {}",
        max_freq,
        min_freq,
        max_freq - min_freq
    );
}
