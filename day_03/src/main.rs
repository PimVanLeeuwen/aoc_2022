use std::collections::HashSet;
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

    let mut priority = 0;

    for rucksack in input.split("\n") {
        let common: HashSet<char> = rucksack[0..rucksack.len()/2]
                        .chars()
                        .filter(|c| rucksack[rucksack.len()/2..rucksack.len()].contains(*c))
                        .collect();
        let common_char = common.into_iter().next().unwrap();

        let mut points = 0;

        match common_char {
            'a'..='z' => points = common_char as u32 - 'a' as u32 + 1,
            'A'..='Z' => points = common_char as u32 - 'A' as u32 + 27,
            _ => println!("This should not happen")
        }

        priority += points; 
    }

    println!("Part 1: {}", priority);
}   

fn part_2(input: &str) {
    let mut priority = 0;

    let lines: Vec<&str> = input.split("\n").collect();

    for i in 0..lines.len()/3 {
        let common: HashSet<char> = lines[i*3]
                        .chars()
                        .filter(|c| lines[(i*3)+1].contains(*c) && lines[(i*3)+2].contains(*c))
                        .collect();
        let badge = common.into_iter().next().unwrap();

        

        let mut points = 0;

        match badge {
            'a'..='z' => points = badge as u32 - 'a' as u32 + 1,
            'A'..='Z' => points = badge as u32 - 'A' as u32 + 27,
            _ => println!("This should not happen")
        }

        priority += points; 
    }

    println!("Part 2: {}", priority);
}