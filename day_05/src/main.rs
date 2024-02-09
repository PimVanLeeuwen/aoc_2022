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
    
    let mut loading = true;
    let mut cargo: Vec<Vec<char>> = Vec::new();
    
    for line in input.lines() {
        if line.trim() == "" {
            loading = false;
        } else if loading {
            let stack: Vec<char> = line.trim().chars().collect();
            cargo.push(stack);
        } else {    
            let parts: Vec<usize> = line.split_whitespace().filter(|x| x.parse::<usize>().is_ok()).map(|x| x.parse().unwrap()).collect();
            for _i in 0..parts[0] {
                let item = cargo[parts[1]-1].pop().unwrap();
                cargo[parts[2]-1].push(item);
            }

        }
    }

    let output: String = cargo.iter().map(|x| x.last().unwrap()).collect();

    println!("Part 1: {:?}", output);
}   

fn part_2(input: &str) {

    let mut loading = true;
    let mut cargo: Vec<Vec<char>> = Vec::new();
    
    for line in input.lines() {
        if line.trim() == "" {
            loading = false;
        } else if loading {
            let stack: Vec<char> = line.trim().chars().collect();
            cargo.push(stack);
        } else {    
            let parts: Vec<usize> = line.split_whitespace().filter(|x| x.parse::<usize>().is_ok()).map(|x| x.parse().unwrap()).collect();
            let mut payload = String::new();
            for _i in 0..parts[0] {
                let item = cargo[parts[1]-1].pop().unwrap();
                payload.push(item);
            }

            for c in payload.chars().rev() {
                cargo[parts[2]-1].push(c);
            }

        }
    }

    let output: String = cargo.iter().map(|x| x.last().unwrap()).collect();

    println!("Part 1: {:?}", output);
}