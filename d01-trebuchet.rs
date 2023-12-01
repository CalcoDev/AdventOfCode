use std::fs;

fn main() {
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
