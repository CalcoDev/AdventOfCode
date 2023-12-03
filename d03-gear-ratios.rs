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

fn read_num_backward(mut str: &str, idx: usize) -> u32 {
    str = &str[..idx+1];
    let prev = str.rfind(|c: char| !c.is_numeric()).unwrap_or(0);
    
    str[prev+1..].parse().expect("ERROR: Could not parse number to u32!")
}

fn read_num_forward(mut str: &str, idx: usize) -> u32 {
    str = &str[idx..];
    let next = str.find(|c: char| !c.is_numeric()).unwrap_or(0);
    
    str[..next].parse().expect("ERROR: Could not parse number to u32!")
}

fn read_num(str: &str, idx: usize) -> u32 {
    let prev = str[..idx+1].rfind(|c: char| !c.is_numeric()).unwrap_or(0);
    let next = str[idx+1..].find(|c: char| !c.is_numeric()).unwrap_or(0);

    str[prev+1..idx+next+1].parse().expect("ERROR: Could not parse number to u32!")
}

fn part1() {
    let input = fs::read_to_string("inputs/own/d03-gear-ratios")
        .expect("ERROR: Could not read input!");

    let width = if let Some(idx) = input.find("\n") {
        idx + 1
    } else {
        input.len()
    };
    let height = input.lines().count();

    let mut s_idx = 0;
    let mut sum = 0;
    while let Some(mut idx) = input.as_str()[s_idx..].find(is_valid_char) {
        idx += s_idx;
        s_idx = idx + 1;
        let (x, y) = get_coords_idx(idx, width);

        // println!("STRING: \n{}", &input.as_str()[s_idx..]);
        // println!("X, Y: {} {}", x + 1, y + 1);
        // continue;

        for y_off in -1i32..1i32 {
            for x_off in -1i32..1i32 {
                if x as i32 + x_off < 0 ||
                    x as i32 + x_off > width as i32 ||
                    y as i32 + y_off < 0 ||
                    y as i32 + y_off > height as i32 {
                    continue;
                }
                
                let idx = get_idx_coords(
                    ((x as i32) + x_off) as usize, 
                    ((y as i32) + y_off) as usize,
                    width
                );

                sum += if (input.as_bytes()[idx] as char).is_numeric() {
                    if x_off < 0 {
                        eprintln!("XOFF < 0");
                        read_num_backward(input.as_str(), idx)
                    } else if x_off > 0 {
                        eprintln!("XOFF > 0");
                        read_num_forward(input.as_str(), idx)
                    } else {
                        eprintln!("XOFF == 0");
                        read_num(input.as_str(), idx)
                    }
                } else {
                    0u32
                }
            }
        }
    }
    
    println!("{}", sum);
}

fn main() {
    part1();
}
