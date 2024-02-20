use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("-- Begin processing --");

    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();



    part1(&map);
    part2(&map);

    println!("-- Done processing  --");
}

fn part1(map: &Vec<Vec<char>>) {   
    let start_pos = (20,0); 
     
    println!("Part 1: {:?}", find_distance(map, start_pos));
}   

fn part2(map: &Vec<Vec<char>>) {   
    let mut min_steps = 41*70+1;  

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for i in 0..41 {
        for j in 0..70 {
            if map[i][j] == 'a' {
                positions.push((i,j));
            }
        }
    }

    for pos in positions {
        let distance = find_distance(map, pos);
        if distance < min_steps { 
            min_steps = distance;
        }
    }

    println!("Part 2: {:?}", min_steps);
}   

fn find_distance(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> u32 {
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::from(vec![((start_pos), 0)]);
        let mut visited: Vec<(usize, usize)> = Vec::new();     

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            
            if current.0 == (20, 46) {
                return current.1;
            } 
                
            visited.push(current.0); 

            for dir in [(-1,0), (0,-1), (1,0), (0,1)] {

                let y: i32 = current.0.0 as i32 + dir.0; 
                let x: i32 = current.0.1 as i32 + dir.1;
        
                if y < 0 || y > 40 || x < 0 || x > 69 {
                    continue; 
                } 
        
                if map[y as usize][x as usize] as u32 <= map[current.0.0][current.0.1] as u32 + 1 && 
                    !visited.contains(&(y as usize, x as usize)) && 
                    !queue.contains(&((y as usize, x as usize), current.1 + 1)) {
                    queue.push_back(((y as usize, x as usize), current.1 + 1));
                } 
            }   

        }

        u32::MAX
}