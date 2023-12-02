use std::fs;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32    
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>
}

fn digit_count(mut num: usize) -> usize {
    let mut cnt = 0;
    if num == 0 {
        return 1;
    }

    while num > 0 {
        num /= 10;
        cnt += 1;
    }

    cnt
}

fn parse_input(input: &String) -> Vec<Game> {
    input.as_str().lines().enumerate().map(|(idx, line)| {
        Game {
            id: (idx as u32) + 1,
            sets: if line.len() == 0 {
                    Vec::new()
                } else {
                    line[(6 + digit_count(idx + 1))..].split(";").map(|set| {
                        set.split(",").map(|item| {
                            let s: Vec<&str> = item.trim().split(" ").collect();

                            eprintln!("Set: {:?} | Item: {:?} | S: {:?}", set, item, s);
                        
                            let cnt = s[0].to_string().trim().parse()
                                .expect("ERROR: Could not convert str to u32!");

                            if s[1].starts_with("red") {
                                CubeSet { red: cnt, green: 0, blue: 0 }
                            } else if s[1].starts_with("green") {
                                CubeSet { red: 0, green: cnt, blue: 0 }
                            } else {
                                CubeSet { red: 0, green: 0, blue: cnt }
                            }
                        }).fold(CubeSet { red: 0, green: 0, blue: 0}, |acc, set| {
                            CubeSet {
                                red: acc.red + set.red,
                                green: acc.green + set.green,
                                blue: acc.blue + set.blue
                            }
                        })
                }).collect()
            }
        }
    }).collect()
}

fn part1() {
    let input = fs::read_to_string("inputs/d02-cube-conundrum")
        .expect("ERROR: Couldn't read input!");

    let games = parse_input(&input);
    let sum = games.iter().fold(0, |sum, game| {
        for set in game.sets.iter() {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                return sum;
            }
        }

        sum + game.id
    });
    
    println!("{}", sum);
}

fn part2() {
    let input = fs::read_to_string("inputs/d02-cube-conundrum")
        .expect("ERROR: Couldn't read input!");

    let games = parse_input(&input);
    let power_sum = games.iter().fold(0, |sum, game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        
        for set in game.sets.iter() {
            if set.red > max_red {
                max_red = set.red;
            }
            if set.green > max_green {
                max_green = set.green;
            }
            if set.blue > max_blue {
                max_blue = set.blue;
            }
        }

        sum + (max_red * max_green * max_blue)
    });
    
    println!("{}", power_sum);
}

fn main() {
    part2();
}
