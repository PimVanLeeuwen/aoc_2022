use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obisidian_cost: (u32,u32),
    geode_cost: (u32, u32)
   }

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut bps: Vec<Blueprint> = Vec::new();
    for l in input.lines() {
        let parts: Vec<u32> = l.split(|c| c == ' ' || c == ':')
            .filter_map(|x| x.parse().ok())
            .collect();
        bps.push(Blueprint { id: parts[0], ore_cost: parts[1], clay_cost: parts[2], 
            obisidian_cost: (parts[3], parts[4]), geode_cost: (parts[5], parts[6]) });
    }

    println!("-- Begin processing --");

    part_1(&bps);
    part_2(&bps);

    println!("-- Done processing  --");
}

fn find_max_geode(bp: &Blueprint, minute: u32, machines: [u32; 4], ores: [u32; 4], part2: bool) -> u32 {

    let max_minutes = match part2 {
        true => 32,
        false => 24,
    };

    if minute == max_minutes {
        ores[3] + machines[3]
    } else {
        // we are processing minute number 'minute'
        let mut options: Vec<u32> = Vec::new();

        // now we check for any of the machines
        if machines[0] > 0 {
            // minutes needed to gather the resources
            let t_needed: u32; 
            if ores[0] < bp.ore_cost {
                t_needed = ((bp.ore_cost - ores[0]) as f32 / machines[0] as f32).ceil() as u32;
            } else {
                t_needed = 0;
            }
            
            // If gathering and building the machine leads to us being over time, then it is no use
            if minute + t_needed + 1 < max_minutes + 1 {
                options.push(find_max_geode(bp, minute + t_needed + 1,
                    [machines[0] + 1, machines[1], machines[2], machines[3]], 
                    [ores[0] + ((t_needed + 1)*machines[0]) - bp.ore_cost, ores[1] + ((t_needed + 1)*machines[1]), ores[2] + ((t_needed + 1)*machines[2]), ores[3] + ((t_needed + 1)*machines[3])], part2));
            }
            
            let t_needed: u32; 
            if ores[0] < bp.clay_cost {
                t_needed = ((bp.clay_cost - ores[0]) as f32 / machines[0] as f32).ceil() as u32;
            } else {
                t_needed = 0;
            }
            // If gathering and building the machine leads to us being over time, then it is no use
            if minute + t_needed + 1 < max_minutes + 1 {
                options.push(find_max_geode(bp, minute + t_needed + 1,
                    [machines[0], machines[1] + 1, machines[2], machines[3]], 
                    [ores[0] + ((t_needed + 1)*machines[0]) - bp.clay_cost, ores[1] + ((t_needed + 1)*machines[1]), ores[2] + ((t_needed + 1)*machines[2]), ores[3] + ((t_needed + 1)*machines[3])], part2));
            }
        }

        if machines[0] > 0 && machines[1] > 0 {
            // minutes needed to gather the resources
            let t_needed_ore: u32; 
            if ores[0] < bp.obisidian_cost.0 {
                t_needed_ore = ((bp.obisidian_cost.0 - ores[0]) as f32 / machines[0] as f32).ceil() as u32;
            } else {
                t_needed_ore = 0;
            }

            let t_needed_clay: u32; 
            if ores[1] < bp.obisidian_cost.1 {
                t_needed_clay = ((bp.obisidian_cost.1 - ores[1]) as f32 / machines[1] as f32).ceil() as u32;
            } else {
                t_needed_clay = 0;
            }

            let t_needed = t_needed_clay.max(t_needed_ore);
            // If gathering and building the machine leads to us being over time, then it is no use
            if minute + t_needed + 1 < max_minutes + 1 {
                options.push(find_max_geode(bp, minute + t_needed + 1,
                    [machines[0], machines[1], machines[2] + 1, machines[3]], 
                    [ores[0] + ((t_needed + 1)*machines[0]) - bp.obisidian_cost.0, ores[1] + ((t_needed + 1)*machines[1]) - bp.obisidian_cost.1, ores[2] + ((t_needed + 1)*machines[2]), ores[3] + ((t_needed + 1)*machines[3])], part2));
            }
        }

        if machines[0] > 0 && machines[2] > 0 {
            // minutes needed to gather the resources
            let t_needed_ore: u32; 
            if ores[0] < bp.geode_cost.0 {
                t_needed_ore = ((bp.geode_cost.0 - ores[0]) as f32 / machines[0] as f32).ceil() as u32;
            } else {
                t_needed_ore = 0;
            }

            let t_needed_obs: u32; 
            if ores[2] < bp.geode_cost.1 {
                t_needed_obs = ((bp.geode_cost.1 - ores[2]) as f32 / machines[2] as f32).ceil() as u32;
            } else {
                t_needed_obs = 0;
            }


            let t_needed = t_needed_obs.max(t_needed_ore);
            // If gathering and building the machine leads to us being over time, then it is no use
            if minute + t_needed + 1 < max_minutes + 1 {
                options.push(find_max_geode(bp, minute + t_needed + 1,
                    [machines[0], machines[1], machines[2], machines[3] + 1], 
                    [ores[0] + ((t_needed + 1)*machines[0]) - bp.geode_cost.0, ores[1] + ((t_needed + 1)*machines[1]), ores[2] + ((t_needed + 1)*machines[2]) - bp.geode_cost.1, ores[3] + ((t_needed + 1)*machines[3])], part2));
            }
        }
        // if we can no longer build, just collect
        if options.is_empty() {
            ores[3] + ((max_minutes - minute + 1)*machines[3])
        } else {
            *options.iter().max().expect("There are numbers in here.")
        }        
    }
}

fn part_1(blueprints: &Vec<Blueprint>) {

    let mut score = 0; 

    for b in blueprints {
        score += find_max_geode(b, 1, [1,0,0,0], [0,0,0,0], false)*b.id;
    }
    
    println!("Part 1: {:?}", score);
}   

fn part_2(blueprints: &Vec<Blueprint>) {

    let mut score = 1; 

    for b in 0..3 {
        score *= find_max_geode(&blueprints[b], 1, [1,0,0,0], [0,0,0,0], true);
        println!("score after iteration: {:?}", score);
    }

    println!("Part 2: {:?}", score);

}