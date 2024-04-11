use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};

fn write_array(char_array: &[[char; 7]; 2022*4]) {
    let mut file = File::create("src/output.txt").unwrap();
    for row in char_array {
        for &ch in row {
            write!(file, "{}", ch).unwrap();
        }
        writeln!(file).unwrap();
    }
}

fn _write_stone(char_array: &[[char; 4]; 4]) {
    for row in char_array {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn place_rock(playground: &mut [[char; 7]; 2022*4], rock: &[[char; 4]; 4], pos: (usize, usize)) {
    for x in 0..4 {
        for y in 0..4 {
            if rock[3 - y][x] == '#' {
                playground[pos.1 - y][pos.0 + x] = rock[3 - y][x]; 
            }
        }
    }    
}

fn check_rock(playground: &[[char; 7]; 2022*4], rock: &[[char; 4]; 4], pos: (usize, usize)) -> bool {
    for x in 0..4 {
        for y in 0..4 {
            if rock[3 - y][x] == '#' && playground[pos.1 - y][pos.0 + x] == '#' {
                return false;
            }  
        }
    }    

    true
}

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut air: VecDeque<char> = VecDeque::new();

    for c in input.chars() {
        air.push_back(c);
    }

    println!("-- Begin processing --");

    part_1(&mut air);

    // part_2(&input);

    println!("-- Done processing  --");
}

fn part_1(air: &mut VecDeque<char>) {

    let mut playground: [[char; 7]; 2022*4] = [['.'; 7]; 2022*4];
    let mut floor = 8087 - 3;
    let mut air_iter = air.iter();
    // println!("{:?}", playground[8][2]);

    // place this many rocks 
    for i in 0..2022 {
        
        let mut shape: [[char; 4]; 4] = [['.'; 4]; 4]; 
        let height;
        let width;
        
        // Define shapes in a 4x4 grid, keep track of their width so that we can match that against left and right side
        match i % 5 {
            0 => {
                shape[3][0] = '#';
                shape[3][1] = '#';
                shape[3][2] = '#';
                shape[3][3] = '#';
                height = 1; 
                width = 4; 
            },
            1 => {
                shape[3][1] = '#';
                shape[2][0] = '#';
                shape[2][1] = '#';
                shape[2][2] = '#';
                shape[1][1] = '#';
                height = 3; 
                width = 3; 
            },
            2 => {
                shape[3][0] = '#';
                shape[3][1] = '#';
                shape[3][2] = '#';
                shape[2][2] = '#';
                shape[1][2] = '#';
                height = 3; 
                width = 3; 
            },
            3 => {
                shape[3][0] = '#';
                shape[2][0] = '#';
                shape[1][0] = '#';
                shape[0][0] = '#';
                height = 4; 
                width = 1; 
            },
            4 => {
                shape[3][0] = '#';
                shape[3][1] = '#';
                shape[2][0] = '#';
                shape[2][1] = '#';
                height = 2; 
                width = 2; 
            },
            _ => break, 
        }

        // begin pos of this rock
        let mut rock_x = 2; 
        let mut rock_y = floor; 
        assert!(check_rock(&playground, &shape, (rock_x,rock_y)));

        // falling 
        loop {
            let d; 
            // first the rush of air if possible 
            match air_iter.next() {
                Some(x) => d = x, 
                None => {
                    air_iter = air.iter();
                    d = air_iter.next().expect("first element should be fine");
                }
            }
            
            if *d == '<' && rock_x > 0 && check_rock(&playground, &shape, (rock_x - 1,rock_y)) {
                rock_x -= 1; 
            } else if *d == '>' && rock_x + width < 7 && check_rock(&playground, &shape, (rock_x + 1,rock_y)) {
                rock_x += 1; 
            }
            
            // then the falling down  
            if rock_y < 8087 && check_rock(&playground, &shape, (rock_x,rock_y+1)) {
                rock_y += 1; 
            } else {
                // It can be the case that it falls lower than an existing block 
                place_rock(&mut playground, &shape, (rock_x,rock_y));
                floor = floor.min(rock_y - height - 3);

                if i == 2021 {
                    println!("Part 1: {:?}", 8088 - (rock_y - height + 1));
                }
                break;
            } 


        }


    }

    write_array(&playground);    
}   

fn _part_2(_input: &str) {
    // For this some cycle detection is needed, might do this later if I feel like it. 
    todo!();
}