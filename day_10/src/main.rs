use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("-- Begin processing --");

    part_1(&input);

    println!("-- Done processing  --");
}

fn part_1(input: &str) {

    let mut cycle_nr = 0; 
    let mut x = 1;
    let mut output = 0;
    let mut register: Vec<String> = Vec::new(); 
    let mut reg_line: String = String::new();

    for line in input.lines() {
        let arguments: Vec<&str> = line.split_whitespace().collect();
        
        if arguments[0] == "noop" {

            cycle_nr += 1; 

            if x-1 <= (cycle_nr - 1) % 40 && (cycle_nr - 1) % 40 <= x+1 {
                reg_line.push('#');
            } else {
                reg_line.push(' ');
            }

            if cycle_nr % 40 == 20 {
                output += cycle_nr*x;
            } else if reg_line.len() == 40 {
                register.push(reg_line);
                reg_line = String::new();
            }
        } else {
            cycle_nr += 1; 

            if x-1 <= (cycle_nr - 1) % 40 && (cycle_nr - 1) % 40 <= x+1 {
                reg_line.push('#');
            } else {
                reg_line.push(' ');
            }

            if cycle_nr % 40 == 20 {
                output += cycle_nr*x;
            } else if reg_line.len() == 40 {
                register.push(reg_line);
                reg_line = String::new();
            }

            cycle_nr += 1; 

            if x-1 <= (cycle_nr - 1) % 40 && (cycle_nr - 1) % 40 <= x+1 {
                reg_line.push('#');
            } else {
                reg_line.push(' ');
            }

            let addx: i32 = arguments[1].parse().unwrap();

            x += addx;

            if cycle_nr % 40 == 20 {
                output += cycle_nr*x;
            } else if reg_line.len() == 40 {
                register.push(reg_line);
                reg_line = String::new();
            }
        }

    }

    println!("Part 2: {:?}", output);
    println!("Part 2:");

    for i in register {
        println!("{i}");
    }
}   
