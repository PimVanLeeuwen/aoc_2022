use std::collections::VecDeque;
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

    let mut coordinates: Vec<(i32, i32, i32,)> = Vec::new(); 

    for line in input.lines() {
        let parts: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().expect("should be an int") + 1).collect();
        coordinates.push((parts[0], parts[1], parts[2])); 
    }

    let mut sides = 0; 

    for c in &coordinates {
        for d in [(-1,0,0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)] {
            if !coordinates.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)) {
                sides += 1; 
            }
        } 
    }
    
    println!("Part 1: {:?}", sides);
}   

fn part_2(input: &str) {

    let mut coordinates: Vec<(i32, i32, i32,)> = Vec::new(); 
    let mut dfs_queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut water_reachable: Vec<(i32, i32, i32)> = Vec::new();  

    for line in input.lines() {
        let parts: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().expect("should be an int") + 1).collect();
        coordinates.push((parts[0], parts[1], parts[2])); 
    }

    dfs_queue.push_back((-1,-1,-1));

    while !dfs_queue.is_empty() {
        let c = dfs_queue.pop_front().expect("I mean !is_empty()");

        water_reachable.push(c);

        for d in [(-1,0,0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)] {
            if c.0 + d.0 < 23 && c.0 + d.0 > -2 && c.1 + d.1 < 23 && c.1 + d.1 > -2 && c.2 + d.2 < 23 && c.2 + d.2 > -2 &&
                !water_reachable.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)) &&
                !coordinates.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)) &&
                !dfs_queue.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)) {
                    dfs_queue.push_back((c.0 + d.0, c.1 + d.1, c.2 + d.2));
            }
        }
    }

    let mut sides = 0; 

    for c in &coordinates {
        for d in [(-1,0,0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)] {
            if water_reachable.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)){
                sides += 1; 
            }
        } 
    }
    
    println!("Part 2: {:?}", sides);

}