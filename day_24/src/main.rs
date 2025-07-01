use std::collections::{HashSet, VecDeque};
use std::fs;

// direction for readability sake
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

// these are the individual blizzard things on the map that we can keep track of like this
#[derive(Clone, Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    dir: Dir,
}

fn main() {
    // text file
    let input = fs::read_to_string("src/input.txt").unwrap();
    // the blizzards in the map
    let mut blizzards: Vec<Blizzard> = Vec::new();
    // these are the input lines
    let lines: Vec<&str> = input.lines().collect();
    
    let height = lines.len();
    let width = lines[0].len();
    let mut walls = HashSet::new();

    // go over all the spots
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // if its a wall insert there, if it is a blizzard, insert there. 
            match c {
                '#' => {
                    walls.insert((x, y));
                }
                '^' => blizzards.push(Blizzard { x, y, dir: Dir::Up }),
                'v' => blizzards.push(Blizzard { x, y, dir: Dir::Down }),
                '<' => blizzards.push(Blizzard { x, y, dir: Dir::Left }),
                '>' => blizzards.push(Blizzard { x, y, dir: Dir::Right }),
                _ => {}
            }
        }
    }
    
    // start and finish positions
    let start = (lines[0].find('.').unwrap(), 0usize);
    let goal = (lines[height - 1].find('.').unwrap(), height - 1);
    
    // period is computed over the area without the walls (hence the -2) 
    let period = lcm(width - 2, height - 2);

    // Precompute blizzard maps for each minute up to period
    // this way we can afterward just loop through the map of obstacles and not care about the 
    // blizzard any more
    let mut blizzard_maps = vec![HashSet::new(); period];
    for minute in 0..period {
        let mut blizzard_positions = HashSet::new();
        for b in &blizzards {
            let (mut x, mut y) = (b.x, b.y);
            match b.dir {
                Dir::Right => x = 1 + ((b.x - 1 + minute) % (width - 2)),
                Dir::Left => x = 1 + ((b.x - 1 + period - (minute % (width - 2))) % (width - 2)),
                Dir::Down => y = 1 + ((b.y - 1 + minute) % (height - 2)),
                Dir::Up => y = 1 + ((b.y - 1 + period - (minute % (height - 2))) % (height - 2)),
            }
            blizzard_positions.insert((x, y));
        }
        blizzard_maps[minute] = blizzard_positions;
    }
    
    

    // BFS
    // to the goal 
    let there = bfs(&blizzard_maps, start, goal, 0, &walls, width, height, period);
    // to get back 
    let back = bfs(&blizzard_maps, goal, start, there, &walls, width, height, period);
    // get back to the goal 
    let again = bfs(&blizzard_maps, start, goal, back, &walls, width, height, period);

    // print both 
    println!("Part 1: {}", there);
    println!("Part 2: {}", again);
    
}

fn bfs(
    blizzard_maps: &Vec<HashSet<(usize, usize)>>,
    start: (usize, usize),
    goal: (usize, usize),
    start_time: usize,
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    period: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start.0, start.1, start_time)); // starting position
    visited.insert((start.0, start.1, start_time)); // which we also visit

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)]; // all directions including wait

    while let Some((x, y, minute)) = queue.pop_front() {
        let next_minute = minute + 1;

        // new blizzard after taking a step 
        let blizzards = &blizzard_maps[next_minute % period];

        // check for all directions
        for (dx, dy) in &directions {
            let (nx, ny) = (x as isize + dx, y as isize + dy);

            // OUTSIDE MAP 
            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
    
            // Goal reached
            if (nx, ny) == goal {
                return next_minute;
            }

            // WALLS & BLIZZARD
            if ((nx, ny) != start && walls.contains(&(nx, ny)) ) || blizzards.contains(&(nx, ny)) {
                continue;
            }
            
            // Otherwise we can go here, so add to queue
            if visited.insert((nx, ny, next_minute % period)) {
                queue.push_back((nx, ny, next_minute));
            }
        }
    }

    panic!("No path found");
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
    
