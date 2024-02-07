use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    part_1(&input);
    part_2(&input);

    println!("-- Done processing --");
}

fn part_1(input: &str) {
    let mut max_carrying = 0;
    let mut current_elf = 0;

    for meal in input.split("\n") {
        if meal.trim() == "" {
            max_carrying = max_carrying.max(current_elf);
            current_elf = 0;
        } else {
            let meal_int: i32 = meal.parse().unwrap();
            current_elf += meal_int;
        }
    }

    println!("Part 1: {}", max_carrying);
}   

fn part_2(input: &str) {
    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf = 0;

    for meal in input.split("\n") {
        if meal.trim() == "" {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            let meal_int: i32 = meal.parse().unwrap();
            current_elf += meal_int;
        }
    }

    elves.sort_by(|a,b| b.cmp(a));

    println!("Part 2: {:?}", &elves[0..3].iter().sum() as &i32);
}