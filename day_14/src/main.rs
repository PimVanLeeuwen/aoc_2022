use std::collections::{HashMap, VecDeque};
use std::fmt::write;
use std::fs::File;
use std::io::{Write, Read};

fn write_array(char_array: [[char; 600]; 200]) {
    let mut file = File::create("src/output.txt").unwrap();
    for row in &char_array {
        for &ch in row {
            write!(file, "{}", ch).unwrap();
        }
        writeln!(file).unwrap();
    }
}

fn parse_input(input: &str, char_array: &mut [[char; 600]; 200]) {
    for line in input.lines() {
        let steps: Vec<&str> = line.split("->")
                                   .map(|s| s.trim())
                                   .collect();
        let mut prev_x = 700; 
        let mut prev_y = 700; 
        for s in steps {
            let temp: Vec<&str> = s.split(",").collect();
            let x: usize = temp[0].parse().expect("Int here");
            let y: usize = temp[1].parse().expect("Int here");
            if prev_x == 700 {
                char_array[y][x] = '#'; 
            } else if prev_x == x {
                for y_new in prev_y..y {
                    char_array[y_new][x] = '#';
                }
            } else {
                for x_new in prev_x..x {
                    char_array[y][x_new] = '#';
                }
            }
            prev_x = x; 
            prev_y = y;
        }
        // println!("{:?}", steps);    
    }

}

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // Declare a 2D array of chars with all elements initialized to the same char
    let mut char_array: [[char; 600]; 200] = [['.'; 600]; 200];
    parse_input(&input, &mut char_array);
    write_array(char_array);

    println!("-- Begin processing --");

    part_1(&input);
    part_2(&input);

    println!("-- Done processing  --");
}

fn part_1(input: &str) {
    
    println!("Part 1: {:?}", "output");
}   

fn part_2(input: &str) {

    println!("Part 2: {:?}", "output");

}