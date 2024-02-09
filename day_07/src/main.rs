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

fn part_1(input: &str) {

    println!("Part 1: {}", "max_carrying");
}   

fn part_2(input: &str) {


    println!("Part 2: {:?}", "&elves[0..3].iter().sum() as &i32");
}