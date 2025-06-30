use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn(&self, rotation: char) -> Self {
        use Direction::*;
        let all = [Right, Down, Left, Up];
        let idx = *self as usize;
        let new_idx = match rotation {
            'R' => (idx + 1) % 4,
            'L' => (idx + 3) % 4,
            _ => idx,
        };
        all[new_idx]
    }

    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}




fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let (grid_str, path_str) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let max_width = grid.iter().map(Vec::len).max().unwrap();
    for row in &mut grid {
        while row.len() < max_width {
            row.push(' ');
        }
    }

    let mut instructions = vec![];
    let mut num = String::new();
    
    // make sure that instruction contains either the full concatenated digits 
    // or the turning instructions
    for c in path_str.trim().chars() {
        if c.is_digit(10) {
            num.push(c);
        } else {
            if !num.is_empty() {
                instructions.push(num.clone());
                num.clear();
            }
            instructions.push(c.to_string());
        }
    }
    if !num.is_empty() {
        instructions.push(num);
    }
    
    // starting coords
    let mut row = 0; // top row 
    let mut col = grid[0].iter().position(|&c| c == '.').unwrap(); // first non-0 spot
    let mut dir = Direction::Right; // start to the right

    for instr in instructions {
        if let Ok(steps) = instr.parse::<usize>() {
            // if this is a walking instruction 
            for _ in 0..steps {
                // current pos and distance that we have to walk
                let (dr, dc) = dir.vector();
                let mut r = row as i32;
                let mut c = col as i32;
    
                // go walking
                loop {
                    r = (r + dr + grid.len() as i32) % grid.len() as i32;
                    c = (c + dc + grid[0].len() as i32) % grid[0].len() as i32;
                    let tile = grid[r as usize][c as usize];
    
                    if tile == ' ' {
                        // keep walking in looping around
                        continue;
                    } else if tile == '#' {
                        // hit a wall, stop walking
                        break;
                    } else if tile == '.' {
                        // found a spot, continue there
                        row = r as usize;
                        col = c as usize;
                        break;
                    }
                }
            }
        } else {
            // turning instruction 
            dir = dir.turn(instr.chars().next().unwrap());
        }
    }
    
    let part_1 = 1000 * (row + 1) + 4 * (col + 1) + dir as usize;
    println!("Part 1: {}", part_1);
}