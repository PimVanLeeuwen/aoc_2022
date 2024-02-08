use std::collections::HashMap;
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
    let mut score = 0;
    let hand_points = HashMap::from([
        ('X', 1),
        ('Y', 2), 
        ('Z', 3)
    ]);

    let match_points = HashMap::from([
        (('A', 'X'), 3),
        (('A', 'Y'), 6),
        (('A', 'Z'), 0),
        (('B', 'X'), 0),
        (('B', 'Y'), 3),
        (('B', 'Z'), 6),
        (('C', 'X'), 6),
        (('C', 'Y'), 0),
        (('C', 'Z'), 3),
    ]);

    let mut rounds = 0; 

    for round in input.split("\n") {
        rounds += 1;

        let opponent = round.chars().nth(0).unwrap();
        let me = round.chars().nth(2).unwrap();

        match hand_points.get(&me) {
            Some(points) => score += points,
            None => println!("None value found for: {:?}", me)
        } 
        
        match match_points.get(&(opponent, me)) {
            Some(points) => score += points,
            None => println!("None value found for: {:?}", (opponent, me))
        }           
    }

    println!("Part 1: {} points over {} lines", score, rounds);
}   

fn part_2(input: &str) {
    let mut score = 0;
    let hand_points = HashMap::from([
        ('X', 0),
        ('Y', 3), 
        ('Z', 6)
    ]);

    let match_points = HashMap::from([
        (('A', 'X'), 3),
        (('A', 'Y'), 1),
        (('A', 'Z'), 2),
        (('B', 'X'), 1),
        (('B', 'Y'), 2),
        (('B', 'Z'), 3),
        (('C', 'X'), 2),
        (('C', 'Y'), 3),
        (('C', 'Z'), 1)
    ]);

    let mut rounds = 0; 

    for round in input.split("\n") {
        rounds += 1;

        let opponent = round.chars().nth(0).unwrap();
        let me = round.chars().nth(2).unwrap();

        match hand_points.get(&me) {
            Some(points) => score += points,
            None => println!("None value found for: {:?}", me)
        } 
        
        match match_points.get(&(opponent, me)) {
            Some(points) => score += points,
            None => println!("None value found for: {:?}", (opponent, me))
        }           
    }

    println!("Part 2: {} points over {} lines", score, rounds);

}