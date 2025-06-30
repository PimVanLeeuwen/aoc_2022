use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Reading the input file
    let file = File::open("src/input.txt").unwrap();
    let reader = io::BufReader::new(file); 
    
    // parse the input
    let input: Vec<i64> = reader
        .lines()
        .filter_map(|line| line.ok()?.parse::<i64>().ok())
        .collect();

    println!("-- Begin processing --");
    
    part_1(&input, 1);
    
    // multiply with decryption key
    let multiplied: Vec<i64> = input
        .iter()
        .map(|&v| 811589153*v)
        .collect(); 
    
    part_1(&multiplied, 10);

    println!("-- Done processing  --");
}

fn part_1(input: &Vec<i64>, times: u32) {
    // create items with their indices 
    let mut items: Vec<(usize, i64)> = input
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect();
    
    for _ in 0..times {
        // permute
        for og_i in 0..input.len() {
            // the next item to move
            let current_pos = items.iter().position(|&(i, _)| i == og_i).unwrap();
            // remove the item from the array 
            let (idx, val) = items.remove(current_pos);
            // compute the new position 
            let new_pos = (current_pos as i64 + val).rem_euclid((input.len() - 1) as i64) as usize;
            // insert the item again
            items.insert(new_pos, (idx, val));
        }
    }
    
    // convert back to array without indices
    let permuted: Vec<i64> = items
        .iter()
        .map(|&(_, v)| v)
        .collect();
    
    let pos_of_zero = permuted
        .iter()
        .position(|&i| i == 0)
        .unwrap(); 
    
    let grove_coords: i64 = [1000, 2000, 3000]
        .iter()
        .map(|&i| permuted[(pos_of_zero + i) % input.len()])
        .sum(); 

    println!("Part 1: {:?}", grove_coords);
}

