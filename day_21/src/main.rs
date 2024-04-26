use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut monkeys: HashMap<String, String> = HashMap::new(); 

    for l in input.lines() {
        let monkey: Vec<&str> = l.split(": ").collect();

        monkeys.insert(String::from(monkey[0]), String::from(monkey[1]));
    }

    println!("-- Begin processing --");

    part_1(&monkeys);
    part_2(&monkeys);

    println!("-- Done processing  --");
}

fn part_1(monkeys: &HashMap<String, String>) {
    
    let root = find_monkey("root", monkeys);

    println!("Part 1: {:?}", root);
}   

fn part_2(monkeys: &HashMap<String, String>) {

    // dunno yet maybe with sat should be easy. 

    println!("Part 2: {:?}", "solution");

}

fn find_monkey(monkey: &str, monkeys: &HashMap<String, String>) -> i64 {
    let monkey = monkeys.get(monkey).expect("should be a monkey with this value");

    // singular shout
    if monkey.parse::<i64>().is_ok() {
        monkey.parse().unwrap()
    } else {
        let m1: i64  = find_monkey(&monkey[..4], monkeys);
        let m2: i64  = find_monkey(&monkey[7..], monkeys);

        match monkey.chars().nth(5).unwrap() {
            '+' => m1 + m2,
            '-' => m1 - m2,
            '/' => m1 / m2,
            _ => m1 * m2,
        }
    }

}   

