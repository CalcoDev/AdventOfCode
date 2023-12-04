use std::fs;
use std::io::{Write, stderr, stdout};

const INPUT: &str = "inputs/d04-scratchcards";

fn get_digit_count(mut num: usize) -> usize {
    if num == 0 {
        return 1;
    }
    
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }

    count
}

fn part1() {
    let input = fs::read_to_string(INPUT)
        .expect("ERROR: Could not parse input.");

    let line_count = input.lines().count();
    let str_begin_idx = 7 + get_digit_count(line_count);
    let split_idx = input.find('|').expect("ERROR: What?") + 1;

    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut chosen_nums: Vec<u32> = Vec::new();

        let mut num_idx = line.len();
        while let Some(idx) = line[split_idx..num_idx].rfind(|c: char| c == ' ') {
            let num_str = line[split_idx + idx..num_idx].trim();
            num_idx = split_idx + idx;
            
            if num_str.len() == 0 {
                continue;
            }

            let num: u32 = num_str.parse()
                .expect("ERROR: Could not parse num!");
            chosen_nums.push(num);
        }

        chosen_nums.sort();

        num_idx = str_begin_idx;
        let mut match_cnt: i32 = 0;
        while let Some(idx) = line[num_idx..split_idx - 1].find(|c: char| c == ' ') {
            let num_str = line[num_idx..num_idx + idx + 1].trim();
            num_idx = num_idx + idx + 1;
            
            if num_str.len() == 0 {
                continue;
            }

            let num: u32 = num_str.parse()
                .expect("ERROR: Could not parse num!");
            if let Ok(_) = chosen_nums.binary_search(&num) {
                match_cnt += 1;
            }
        }

        if match_cnt > 0 {
            sum += 2u32.pow(std::cmp::max(0i32, match_cnt - 1) as u32);
        }
    }

    println!("Sum: {}", sum);
}

    
fn part2() {
    let input = fs::read_to_string(INPUT)
        .expect("ERROR: Could not parse input.");

    let line_count = input.lines().count();
    let str_begin_idx = 7 + get_digit_count(line_count);
    let split_idx = input.find('|').expect("ERROR: What?") + 1;

    let mut cards_won: Vec<(usize, u32)> = Vec::with_capacity(line_count);
    for line in input.lines() {
        let mut chosen_nums: Vec<u32> = Vec::new();

        let mut num_idx = line.len();
        while let Some(idx) = line[split_idx..num_idx].rfind(|c: char| c == ' ') {
            let num_str = line[split_idx + idx..num_idx].trim();
            num_idx = split_idx + idx;
            
            if num_str.len() == 0 {
                continue;
            }

            let num: u32 = num_str.parse()
                .expect("ERROR: Could not parse num!");
            chosen_nums.push(num);
        }

        chosen_nums.sort();

        num_idx = str_begin_idx;
        let mut match_cnt: i32 = 0;
        while let Some(idx) = line[num_idx..split_idx - 1].find(|c: char| c == ' ') {
            let num_str = line[num_idx..num_idx + idx + 1].trim();
            num_idx = num_idx + idx + 1;
            
            if num_str.len() == 0 {
                continue;
            }

            let num: u32 = num_str.parse()
                .expect("ERROR: Could not parse num!");
            if let Ok(_) = chosen_nums.binary_search(&num) {
                match_cnt += 1;
            }
        }

        cards_won.push(
            (match_cnt as usize, if match_cnt > 0 {
                2u32.pow(std::cmp::max(0i32, match_cnt - 1) as u32)
            } else {
                0u32
            })
        );
    }

    let mut count = 0;
    let mut stack: Vec<usize> = (0..line_count).collect();

    while let Some(curr_card) = stack.pop() {
        count += 1;
        stack.extend_from_slice(
            &(curr_card..curr_card+cards_won[curr_card].0).map(|num| num + 1).collect::<Vec<usize>>()
        );
    }

    println!("Sum: {}", count);
}

fn main() {
    part2();
}
