use std::collections::HashMap;
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

    let mut file_stack: Vec<String> = Vec::new();
    let mut folder_sizes: HashMap<String, u64> = HashMap::new();

    for l in input.lines() {
        if l.starts_with("$ ls") || l.starts_with("dir") {
            continue; 
        } else if l.starts_with("$ cd") {
            let dest: Vec<&str> = l.split_whitespace().collect();
            let dest = dest[2];

            if dest == ".." {
                file_stack.pop();
            } else {
                if file_stack.is_empty() {
                    file_stack.push(dest.to_string());
                } else {
                    let mut dest_path = file_stack.last().unwrap().to_string();
                    dest_path.push_str("/");
                    dest_path.push_str(dest);

                    file_stack.push(dest_path);
                }
            }
        } else {
            let size: Vec<&str> = l.split_whitespace().collect();
            let size: u64 = size[0].parse().unwrap();
            for c in file_stack.clone().into_iter() {
                let entry = folder_sizes.entry(c).or_insert(0);
                *entry += size; 
            }
        }

    }   

    let mut result: u64 = 0;

    let used_space_now = folder_sizes.get("/").unwrap().clone();
    let free_space_now = 70000000 - used_space_now;
    let space_needed = 30000000 - free_space_now;
    let mut smallest: u64 = 70000000; 

    for (_folder, size) in folder_sizes {
        
        if size <= 100000 {
            result += size; 
        }

        if size > space_needed && size < smallest {
            smallest = size;
        }
    }
    
    println!("Part 1: {:?}", result);

    println!("Part 2: {:?}", smallest);

}
    