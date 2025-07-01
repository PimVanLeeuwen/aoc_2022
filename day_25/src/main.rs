use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read");

    // total sum of lines 
    let total: i64 = input
        .lines()
        .map(|line| snafu_to_decimal(line))
        .sum();

    // convert total back to snafu 
    let snafu = decimal_to_snafu(total);
    
    // print total 
    println!("Part 1: {}", snafu);
}

// Convert a SNAFU string to a decimal number
fn snafu_to_decimal(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let power = 5_i64.pow(i as u32);
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid SNAFU digit: {}", c),
            };
            digit * power
        })
        .sum()
}

// Convert a decimal number back into SNAFU format
fn decimal_to_snafu(mut n: i64) -> String {
    let mut result = String::new();

    while n != 0 {
        let mut rem = n % 5;
        n /= 5;

        let ch = match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                n += 1;
                '='
            }
            4 => {
                n += 1;
                '-'
            }
            _ => unreachable!(),
        };
        result.insert(0, ch);
    }

    result
}