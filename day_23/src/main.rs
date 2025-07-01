use std::fs;
use std::collections::{HashMap, HashSet}; 

type Point = (i32, i32); 

fn main() {
    // read input
    let input = fs::read_to_string("src/input.txt").expect("Failed to read");
    
    // HashSet of all coordinates of the elves (of points) 
    let mut elves: HashSet<Point> = HashSet::new(); 
    
    // insert all the elves from the text files 
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i32, y as i32)); 
            }     
        }
    }

    // all possible directions
    let directions: [[(i32, i32); 3]; 4] = [
        [(-1, -1), (0, -1), (1, -1)], // N
        [(-1, 1), (0, 1), (1, 1)],    // S
        [(-1, -1), (-1, 0), (-1, 1)], // W
        [(1, -1), (1, 0), (1, 1)],    // E
    ];

    // adjacent tiles 
    let all_adjacent = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    
    // start with North
    let mut dir_index = 0; 
    
    // do ten rounds 
    for round in 0..10000 {
        // this will cover all proposed moves 
        let mut proposals: HashMap<Point, Vec<Point>> = HashMap::new(); 
        
        // for each elf
        for &elf in &elves {
            let mut has_neighbour = false; 
            // check all neigbours
            for (dx, dy) in &all_adjacent {
                if elves.contains(&(elf.0 + dx, elf.1 + dy)) {
                    has_neighbour = true; 
                    break; 
                }
            }
            
            if !has_neighbour {
                // if no neighbours we can skip this elf because it will not move
                continue; 
            }
            
            // otherwise we check the directions, starting at the current dir_index
            for i in 0..4 {
                let dir = directions[(dir_index + i) % 4];
                // check if there is an elf in that direction 
                if dir.iter().all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy))) {
                    let (dx, dy) = dir[1];
                    let dest = (elf.0 + dx, elf.1 + dy);
                    // add the proposal to the proposal list 
                    proposals.entry(dest).or_default().push(elf);
                    break;
                }
            }
        }

        // create the new elves HashSet 
        let mut new_elves = elves.clone();
        
        // check for all of the proposals if they are possible, so no move if more would go to a tile
        for (dest, sources) in &proposals {
            if sources.len() == 1 {
                new_elves.remove(&sources[0]);
                new_elves.insert(*dest);
            }
        }
        
        // if no move, stop
        if (elves == new_elves) {
            println!("Part 2: {}", round + 1);
            break; 
        }
        
        // replace the old elves
        elves = new_elves;
        dir_index = (dir_index + 1) % 4;
        
        // after 10 (0 included) rounds do the part 1 thing 
        if (round == 9) {
            let min_x = elves.iter().map(|e| e.0).min().unwrap();
            let max_x = elves.iter().map(|e| e.0).max().unwrap();
            let min_y = elves.iter().map(|e| e.1).min().unwrap();
            let max_y = elves.iter().map(|e| e.1).max().unwrap();

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;

            let part_1 = width * height - elves.len() as i32;
            println!("Part 1: {}", part_1);
        }
    }
    
}