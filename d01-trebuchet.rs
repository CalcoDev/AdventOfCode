use std::fs;

fn part1() {
    let input = fs::read_to_string("inputs/d01-trebuchet")
        .expect("ERROR: Could not read input file!");

    let sum = input.as_str().split("\n").fold(0, |sum, line| {
        let f_idx = line.find(char::is_numeric);
        
        sum + if let Some(f_idx) = f_idx {
            let l_idx = line.rfind(char::is_numeric).unwrap();
            
            (line.as_bytes()[f_idx] as char).to_digit(10).unwrap() * 10 + 
            (line.as_bytes()[l_idx] as char).to_digit(10).unwrap()
        } else {
            0
        }
    });
        
    println!("{}", sum);
}

const NUMBER_IDS: [&str; 9] = 
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn part2() {
    let input = fs::read_to_string("inputs/d01-trebuchet")
        .expect("ERROR: Could not read input file!");

    let sum = input.as_str().split("\n").fold(0, |sum, line| {
        let mut first_num = 0;
        let mut last_num = 0;
        
        'outer: for (idx, c) in line.char_indices() {
            if c.is_numeric() {
                first_num = c.to_digit(10).unwrap();
                break;
            }

            for (nidx, num) in NUMBER_IDS.iter().enumerate() {
                if line[idx..].starts_with(num) {
                    first_num = (nidx as u32) + 1;
                    break 'outer;
                }
            }
        }
        'outer: for (idx, c) in line.char_indices().rev() {
            if c.is_numeric() {
                last_num = c.to_digit(10).unwrap();
                break;
            }

            for (nidx, num) in NUMBER_IDS.iter().enumerate() {
                if line[idx..].starts_with(num) {
                    last_num = (nidx as u32) + 1;
                    break 'outer;
                }
            }
        }
        sum + (first_num * 10 + last_num)
    });

    println!("{}", sum);
}

fn main() {
    part2();
}
