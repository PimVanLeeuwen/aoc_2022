use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("-- Begin processing --");

    part_1(&input);
    part_2(&input);

    println!("-- Done processing  --");
}

struct Node {
    file: bool, 
    size: u64, 
    children: Vec<Node>
}

fn part_1(input: &str) {
    
    let mut last_four: VecDeque<char> = VecDeque::new(); 

    for (i, c) in input.trim().chars().enumerate() {
        last_four.push_back(c);
        
        if last_four.len() > 4 {
            last_four.pop_front();    
        }

        if last_four.len() == 4 {
            let string: String = last_four.iter().collect();
            if string.chars().all(|c| string.matches(c).count() == 1) {
                println!("Part 1: {:?}", i+1);
                break;
            }
        }
    }


    // println!("Part 1: {:?}", "output");
}   

fn part_2(input: &str) {


    let mut last_fourteen: VecDeque<char> = VecDeque::new(); 

    for (i, c) in input.trim().chars().enumerate() {
        last_fourteen.push_back(c);
        
        if last_fourteen.len() > 14 {
            last_fourteen.pop_front();    
        }

        if last_fourteen.len() == 14 {
            let string: String = last_fourteen.iter().collect();
            if string.chars().all(|c| string.matches(c).count() == 1) {
                println!("Part 2: {:?}", i+1);
                break;
            }
        }
    }
}