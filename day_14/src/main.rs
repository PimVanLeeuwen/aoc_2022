use std::fs::File;
use std::io::{Write, Read};

fn write_array(char_array: [[char; 1200]; 200]) {
    let mut file = File::create("src/output.txt").unwrap();
    for row in &char_array {
        for &ch in row {
            write!(file, "{}", ch).unwrap();
        }
        writeln!(file).unwrap();
    }
}

fn parse_input(input: &str, char_array: &mut [[char; 1200]; 200]) {
    for line in input.lines() {
        let steps: Vec<&str> = line.split("->")
                                   .map(|s| s.trim())
                                   .collect();
        let mut prev_x = 700; 
        let mut prev_y = 700; 
        for s in steps {
            let temp: Vec<&str> = s.split(",").collect();
            let x: usize = temp[0].parse().expect("Int here");
            let y: usize = temp[1].parse().expect("Int here");
            if prev_x == 700 {
                char_array[y][x] = '#'; 
            } else if prev_x == x {
                for y_new in usize::min(prev_y, y)..=usize::max(prev_y, y) {
                    char_array[y_new][x] = '#';
                }
            } else {
                for x_new in usize::min(prev_x, x)..=usize::max(prev_x, x) {
                    char_array[y][x_new] = '#';
                }
            }
            prev_x = x; 
            prev_y = y;
        }
    }

}

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // Declare a 2D array of chars with all elements initialized to the same char
    let mut char_array: [[char; 1200]; 200] = [['.'; 1200]; 200];
    parse_input(&input, &mut char_array);

    println!("-- Begin processing --");

    part_1(&mut char_array);

    char_array = [['.'; 1200]; 200];
    parse_input(&input, &mut char_array);

    part_2(&mut char_array);

    println!("-- Done processing  --");

    write_array(char_array);
}

fn part_1(char_array: &mut [[char; 1200]; 200]) {
    
    let mut pieces = 0; 

    // poor a new sand piece
    loop {
        let mut x = 500; 
        let mut y = 0;
        let pieces_start = pieces; 

        while y < 199 {
            
            // Move down 
            if char_array[y+1][x] == '.' {
                y += 1; 
            } else if char_array[y+1][x-1] == '.' {
                y += 1; 
                x -= 1;
            } else if char_array[y+1][x+1] == '.' {
                y += 1; 
                x += 1; 
            } else {
                char_array[y][x] = '+'; 
                pieces += 1;  
                break;
            }
        }

        if pieces == pieces_start {
            break;
        }
    }

    println!("Part 1: {:?}", pieces);
}   

fn part_2(char_array: &mut [[char; 1200]; 200]) {
    let mut pieces = 0; 

    while char_array[0][500] == '.' {
        let mut x = 500; 
        let mut y = 0;

        while y < 165 {
            // Move down 
            if char_array[y+1][x] == '.' {
                y += 1; 
            } else if char_array[y+1][x-1] == '.' {
                y += 1; 
                x -= 1;
            } else if char_array[y+1][x+1] == '.' {
                y += 1; 
                x += 1; 
            } else {
                break;
            }
        }

        char_array[y][x] = '+'; 
        pieces += 1;  
    }

    println!("Part 2: {:?}", pieces);

}