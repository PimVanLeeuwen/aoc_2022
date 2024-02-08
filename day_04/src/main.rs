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

    let mut redundant = 0;

    for pair in input.lines() {
        let elves: Vec<u32> = pair.split(&[',', '-']).map(|x| x.parse().unwrap()).collect();

        if (elves[0] >= elves[2] && elves[1] <= elves[3]) || 
            (elves[2] >= elves[0] && elves[3] <= elves[1]) {
            redundant += 1;
        }
    }

    println!("Part 1: {}", redundant);
}   

fn part_2(input: &str) {

    let mut overlap = 0;

    for pair in input.lines() {
        let elves: Vec<u32> = pair.split(&[',', '-']).map(|x| x.parse().unwrap()).collect();

        if (elves[0] >= elves[2] && elves[0] <= elves[3]) || 
            (elves[1] >= elves[2] && elves[1] <= elves[3]) ||
            (elves[2] >= elves[0] && elves[2] <= elves[1]) ||
            (elves[3] >= elves[0] && elves[3] <= elves[1]) {
                overlap += 1;
        }
    }

    println!("Part 2: {}", overlap);
}