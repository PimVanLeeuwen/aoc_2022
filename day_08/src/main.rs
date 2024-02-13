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

    let mut tree_grid: Vec<Vec<i32>> = Vec::new();
    let mut visible_grid: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut tree_line: Vec<i32> = Vec::new();
        let mut visible_line: Vec<i32> = Vec::new();
        for c in line.chars() {
            tree_line.push(c.to_digit(10).unwrap() as i32);
            visible_line.push(0);
        }
        tree_grid.push(tree_line);
        visible_grid.push(visible_line);
    }

    let height = tree_grid.len();
    let width = tree_grid[0].len();

    for c in 0..height {
        let mut down_max = -1;
        let mut up_max = -1;

        // down and up 
        for r in 0..width {
            if tree_grid[c][r] > down_max {
                down_max = tree_grid[c][r];
                visible_grid[c][r] = 1;
            }
            if tree_grid[c][width-1-r] > up_max {
                up_max = tree_grid[c][width-1-r];
                visible_grid[c][width-1-r] = 1;
            }
        }
    }

    for r in 0..width {
        let mut right_max = -1;
        let mut left_max = -1;

        // right
        for c in 0..height {
            if tree_grid[c][r] > right_max {
                right_max = tree_grid[c][r];
                visible_grid[c][r] = 1;
            }
            if tree_grid[height-1-c][r] > left_max {
                left_max = tree_grid[height-1-c][r];
                visible_grid[height-1-c][r] = 1;
            }
        }
    }

    let mut result = 0;

    for r in visible_grid {
        result += r.iter().filter(|&x| *x == 1).count();
    }

    println!("Part 1: {:?}", result);
}   

fn part_2(input: &str) {

    let mut tree_grid: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut tree_line: Vec<i32> = Vec::new();
        for c in line.chars() {
            tree_line.push(c.to_digit(10).unwrap() as i32);
        }
        tree_grid.push(tree_line);
    }

    let height = tree_grid.len();
    let width = tree_grid[0].len();

    let mut result = 0;

    for c in 0..height {
        for r in 0..width {
            let mut vision: Vec<u32> = Vec::from(vec![0,0,0,0]);

            // check left
            for c_1 in (0..c).rev() {
                if tree_grid[c_1][r] < tree_grid[c][r] {
                    vision[0] += 1;
                } else {
                    vision[0] += 1;
                    break;
                }
            }

            // check rigth
            for c_1 in c+1..width {
                if tree_grid[c_1][r] < tree_grid[c][r] {
                    vision[1] += 1;
                } else {
                    vision[1] += 1;
                    break;
                }
            }

            // check up
            for r_1 in (0..r).rev() {
                if tree_grid[c][r_1] < tree_grid[c][r] {
                    vision[2] += 1;
                } else {
                    vision[2] += 1;
                    break;
                }
            }

            // check down
            for r_1 in r+1..height {
                if tree_grid[c][r_1] < tree_grid[c][r] {
                    vision[3] += 1;
                } else {
                    vision[3] += 1;
                    break;
                }
            }

            if vision[0]*vision[1]*vision[2]*vision[3] > result {
                result = vision[0]*vision[1]*vision[2]*vision[3];
            }


        }
    }

    println!("Part 2: {:?}", result);

}