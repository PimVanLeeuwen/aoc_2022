use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("-- Begin processing --");

    part(&input, 20, 1);
    part(&input, 10000, 2);

    println!("-- Done processing  --");
}

fn part(input: &str, rounds: u64, part: u64) {

    let mut monkeys: Vec<(Vec<u64>, u64)> = Vec::new();

    for line in input.lines().filter(|x| x.trim().starts_with("Starting")) {
        let mut monkey: Vec<u64> = Vec::new();
        for i in line.split(|c| c == ',' || c == ' ').filter(|c| c.trim().parse::<u64>().is_ok()) {
            monkey.push(i.parse::<u64>().unwrap());
        }
        monkeys.push((monkey, 0));
    }

    let rounds = rounds; 

    for _i in 0..rounds {
        for m in 0..8 {
            let items = monkeys[m].0.clone();
            monkeys[m].0 = Vec::new();
            for item in items {
                monkeys[m].1 += 1; 
                // inspect item 
                let new_val: u64; 
                if part == 1 {
                    new_val = monkey_operation(m, item) / 3;
                } else {
                    new_val = monkey_operation(m, item) % (19 * 2 * 13 * 5 * 7 * 11 * 17 * 3);
                }
                
                let target = monkey_test(m, new_val);
                monkeys[target].0.push(new_val);
            }
        }
    }

    let mut throws: Vec<u64> = monkeys.into_iter().map(|x| x.1).collect();
    throws.sort();

    println!("Part: {:?}", throws[6] * throws[7]);
}   

fn monkey_operation(monkey_nr: usize, old: u64) -> u64 {
    match monkey_nr {
        0 => old * 13, 
        1 => old + 3, 
        2 => old + 6, 
        3 => old + 2, 
        4 => u64::pow(old, 2), 
        5 => old + 4, 
        6 => old * 7,
        7 => old + 7, 
        _ => old
    }
}

fn monkey_test(monkey_nr: usize, value: u64) -> usize {
    match monkey_nr {
        0 => if value % 19 == 0 { 6 } else { 7 }, 
        1 => if value % 2  == 0 { 5 } else { 4 }, 
        2 => if value % 13 == 0 { 4 } else { 1 }, 
        3 => if value % 5  == 0 { 6 } else { 0 }, 
        4 => if value % 7  == 0 { 5 } else { 3 }, 
        5 => if value % 11 == 0 { 3 } else { 0 }, 
        6 => if value % 17 == 0 { 2 } else { 7 }, 
        7 => if value % 3  == 0 { 2 } else { 1 },  
        _ => 10
    }
}

