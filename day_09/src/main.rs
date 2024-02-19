use std::collections::HashSet;
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
    let mut positions_head: HashSet<(i32, i32)> = HashSet::from([(0,0)]);
    let mut positions_tail: HashSet<(i32, i32)> = HashSet::from([(0,0)]);
    let mut tail_pos: Vec<(i32, i32)> = Vec::from(vec![(0,0);10]);
    
    for op in input.lines() {
        let ops: Vec<&str> = op.split_whitespace().collect();
        let direction: &str  = ops[0];
        let amount: u64 = ops[1].parse().unwrap();

        for _i in 0..amount {
            match direction {
                "D" => tail_pos[0] = (tail_pos[0].0, tail_pos[0].1 - 1),
                "L" => tail_pos[0] = (tail_pos[0].0 - 1, tail_pos[0].1),
                "R" => tail_pos[0] = (tail_pos[0].0 + 1, tail_pos[0].1),
                _ => tail_pos[0] = (tail_pos[0].0, tail_pos[0].1 + 1),
            }

            for i in 1..=9 {

                let mut diff = (
                    tail_pos[i-1].0 - tail_pos[i].0,
                    tail_pos[i-1].1 - tail_pos[i].1,
                );

                let abs_diff = (
                    i32::abs_diff(tail_pos[i-1].0, tail_pos[i].0),
                    i32::abs_diff(tail_pos[i-1].1, tail_pos[i].1)
                );
                
                // withing the proper distance 
                if abs_diff.0 < 2 && abs_diff.1 < 2 {
                    diff = (0,0);
                } 
                
                if abs_diff.0 == 2 {
                    diff.0 /= 2;
                } 
                
                if abs_diff.1 == 2 {
                    diff.1 /= 2;
                }
 
                tail_pos[i].0 += diff.0;
                tail_pos[i].1 += diff.1;
            }

            positions_head.insert(tail_pos[1]);
            positions_tail.insert(tail_pos[9]);
        }

        
        
    }

    println!("Part 1: {:?}", positions_head.len());
    println!("Part 2: {:?}", positions_tail.len());
}   