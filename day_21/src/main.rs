use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use z3::ast::Ast;
use z3::*; 

fn main() {
    // Reading the input file
    let mut file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("-- Begin processing --");

    part_2(&input);

    println!("-- Done processing  --");
} 

fn part_2(input: &str) {

    // Create a Z3 context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // here we store the variables and there names
    let mut variables: HashMap<String, ast::Int<'_>> = HashMap::new();

    let solver_part_1 = Solver::new(&ctx);
    let solver_part_2 = Solver::new(&ctx);

    for l in input.lines() {
        let monkey: Vec<&str> = l.split(": ").collect();

        variables.insert(String::from(monkey[0]), ast::Int::new_const(&ctx, String::from(monkey[0])));
 
    }

    for l in input.lines() {

        let monkey: Vec<&str> = l.split(": ").collect();
        let m_source = variables.get(&String::from(monkey[0])).expect("expect value here");

        if monkey[1].parse::<i64>().is_ok() {
            // variable is equal 
            if monkey[0] != "humn" {
                solver_part_2.assert(&m_source._eq(&ast::Int::from_i64(&ctx, monkey[1].parse::<i64>().unwrap())));
            } 

            solver_part_1.assert(&m_source._eq(&ast::Int::from_i64(&ctx, monkey[1].parse::<i64>().unwrap())));
        } else {

            let m1 = variables.get(&monkey[1][..4]).expect("expect value here");
            let m2 = variables.get(&monkey[1][7..]).expect("expect value here");

            if monkey[0] == "root" {
                solver_part_1.assert(&m_source._eq(&ast::Int::add(&ctx, &[&m1, &m2])));
                solver_part_2.assert(&m1._eq(&m2));
            } else {
                match monkey[1].chars().nth(5).unwrap() {
                    '+' => {
                        solver_part_1.assert(&m_source._eq(&ast::Int::add(&ctx, &[&m1, &m2])));
                        solver_part_2.assert(&m_source._eq(&ast::Int::add(&ctx, &[&m1, &m2])));
                    },
                    '-' => {
                        solver_part_1.assert(&m_source._eq(&ast::Int::sub(&ctx, &[&m1, &m2])));
                        solver_part_2.assert(&m_source._eq(&ast::Int::sub(&ctx, &[&m1, &m2])));
                    },
                    '/' => {
                        solver_part_1.assert(&m_source._eq(&m1.div(&m2)));
                        solver_part_2.assert(&m_source._eq(&m1.div(&m2)));
                    },
                    _ => {
                        solver_part_1.assert(&m_source._eq(&ast::Int::mul(&ctx, &[&m1, &m2])));
                        solver_part_2.assert(&m_source._eq(&ast::Int::mul(&ctx, &[&m1, &m2])));
                    },
                }    
            }
                   
        }
 
    }

    assert_eq!(solver_part_1.check(), SatResult::Sat);

    let model_part_1 = solver_part_1.get_model().unwrap();
    let part_1 = model_part_1.eval(variables.get("root").expect("variable under this name"), true).unwrap().as_i64().unwrap();

    println!("Part 1: {:?}", part_1);

    assert_eq!(solver_part_2.check(), SatResult::Sat);

    let model_part_2 = solver_part_2.get_model().unwrap();
    let part_2 = model_part_2.eval(variables.get("humn").expect("variable under this name"), true).unwrap().as_i64().unwrap();

    println!("Part 2: {:?}", part_2);

}
  

