use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn hamming_dist(x_source: i32, y_source: i32, x_dest: i32, y_dest: i32) -> u32 {
    i32::abs_diff(x_source, x_dest) + i32::abs_diff(y_source, y_dest)
}
 
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

    let mut result: HashSet<i32> = HashSet::new();

    for s in input.lines() {
        let coordinates: Vec<i32> = s.split(|c| c == ',' || c == ' ' || c == '=' || c == ':')
         .filter_map(|s| s.parse().ok())
         .collect();

        let sen_x = coordinates[0];
        let sen_y = coordinates[1];
        let be_x = coordinates[2];
        let be_y = coordinates[3];

        let distance_given = hamming_dist(sen_x, sen_y, be_x, be_y);
        let min_x = sen_x - distance_given as i32; 
        let max_x = sen_x + distance_given as i32; 

        for test_x in min_x..=max_x {
            if hamming_dist(sen_x, sen_y, test_x, 2000000) <= distance_given {
                if test_x != be_x || 2000000 != be_y {
                    result.insert(test_x);
                }
            }
        }

    }
    
    println!("Part 1: {:?}", result.len());
}   

fn part_2(input: &str) {
    let mut sensors: HashSet<(i32, i32, i32)> = HashSet::new();

    for s in input.lines() {
        let coordinates: Vec<i32> = s.split(|c| c == ',' || c == ' ' || c == '=' || c == ':')
         .filter_map(|s| s.parse().ok())
         .collect();

        let sen_x = coordinates[0];
        let sen_y = coordinates[1];
        let be_x = coordinates[2];
        let be_y = coordinates[3];

        let distance_given = hamming_dist(sen_x, sen_y, be_x, be_y) as i32;

        sensors.insert((sen_x, sen_y, distance_given));

    }

    for s1 in sensors.iter() {

        for d in 0..=(s1.2 + 1) {
            let x_diff: i32 = d;
            let y_diff: i32 = s1.2 + 1 - d;

            for c in [(1,1), (1,-1), (-1,1), (-1,-1)] {
                // Candidate point
                let x = s1.0+(c.0*x_diff);
                let y = s1.1+(c.1*y_diff);
                if x < 0 || y < 0 || x > 4000000 || y > 4000000 {
                    continue; 
                }
                assert!(hamming_dist(x, y, s1.0, s1.1) == (s1.2 + 1) as u32);
                let mut correct = true; 
                
                for s2 in sensors.iter() {
                    if hamming_dist(x, y, s2.0, s2.1) <= s2.2 as u32 {
                        correct = false; 
                        break; 
                    }
                }

                if correct {
                    println!("Part 2: {}", (x as u64)*4000000 + (y as u64));
                    return; 
                }
            }

        }   
    }    

}