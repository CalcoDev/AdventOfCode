use std::fs;

fn is_valid_char(c: char) -> bool {
    c != '.' && !c.is_numeric() && !c.is_whitespace()
}

fn get_idx_coords(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn get_coords_idx(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

fn read_num(str: &str, idx: usize) -> (u32, usize, usize) {
    let prev = if let Some(idx) = str[..idx+1].rfind(|c: char| !c.is_numeric()) {
        idx as i32
    } else {
        -1
    };
    
    let next = if let Some(idx) = str[idx+1..].find(|c: char| !c.is_numeric()) {
        idx as i32
    } else {
        str.len() as i32 - 1
    };

    let prev = (prev+1) as usize;
    let next = (idx as i32+next+1) as usize;
    let num = str[prev..next].parse()
        .expect("ERROR: Could not parse number to u32!");
    
    (num, prev, next)
}

fn part1() {
    let input = fs::read_to_string("inputs/d03-gear-ratios")
        .expect("ERROR: Could not read input!");

    let width = if let Some(idx) = input.find("\n") {
        idx + 1
    } else {
        input.len()
    };
    let height = input.lines().count();

    let mut nums: Vec<u32> = Vec::new();
    let mut s_idx = 0;
    while let Some(mut idx) = input.as_str()[s_idx..].find(|c: char| c.is_numeric()) {
        idx += s_idx;
        s_idx = idx;

        let next = input.as_str()[s_idx..].find(|c: char| !c.is_numeric()).unwrap_or(0);
        let num: u32 = input.as_str()[s_idx..s_idx+next].parse()
            .expect("ERROR: Could not parse number to u32!");

        s_idx += next;
        nums.push(num);
    }
    nums.sort();
    let all_sum: u32 = nums.iter().sum();
    let mut ratio_sum: u32 = 0;


    let mut s_idx = 0;
    while let Some(mut idx) = input.as_str()[s_idx..].find(is_valid_char) {
        idx += s_idx;
        s_idx = idx + 1;

        let mut check: Vec<bool> = vec![false; width * height];
        let (x, y) = get_coords_idx(idx, width);

        let mut gear_nums: Vec<u32> = Vec::new();
        for y_off in -1i32..2i32 {
            for x_off in -1i32..2i32 {
                if x as i32 + x_off < 0 ||
                    x as i32 + x_off >= width as i32 ||
                    y as i32 + y_off < 0 ||
                    y as i32 + y_off >= height as i32 {
                    continue;
                }
                
                let idx = get_idx_coords(
                    ((x as i32) + x_off) as usize, 
                    ((y as i32) + y_off) as usize,
                    width
                );
                
                if check[idx] {
                    continue;
                }

                if (input.as_bytes()[idx] as char).is_numeric() {
                    let (num, l, r) = read_num(input.as_str(), idx);
                    gear_nums.push(num);
                    if let Ok(idx) = nums.binary_search(&num) {
                        nums.remove(idx);
                        for i in l..r {
                            check[i] = true;
                        }
                    }
                }
            }
        }

        println!("CHAR: {}:\n{:?}", input.as_bytes()[idx] as char, gear_nums);
        if input.as_bytes()[idx] == b'*' && gear_nums.len() == 2 {
            println!("{:?}", gear_nums);
            ratio_sum += gear_nums[0] * gear_nums[1];
        }
    }

    let diff: u32 = nums.iter().sum();
    let sum = all_sum - diff;
    println!("Sum: {}", sum);
    println!("Ratio Sum: {}", ratio_sum);

    eprintln!("all {} sum {} vec {:?}", all_sum, sum, nums);
    // panic!();
}

fn main() {
    part1();
}
