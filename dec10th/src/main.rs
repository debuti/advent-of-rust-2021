const OPENINGS : [char; 4] = ['(', '[', '{', '<'];
const CLOSINGS : [char; 4] = [')', ']', '}', '>'];
const POINTS   : [u32;  4] = [3, 57, 1197, 25137];

fn main() {
    let data = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();

    let mut score1 = 0;
    let mut score2s = Vec::<u64>::new();
    for line in data {
        let mut stack = Vec::<char>::new();
        for c in line.chars() {
            match stack.last() {
                None => {
                    if OPENINGS.contains(&c) {
                        stack.push(c);
                    } else { // Nothing to close
                        stack.clear();
                        break;
                    }
                }
                Some(&x) => {
                    if OPENINGS.contains(&c) {
                        stack.push(c);
                    } else {
                        let closing_idx = CLOSINGS.iter().position(|&i| i == c).unwrap();
                        match OPENINGS.iter().position(|&i| i == x) {
                            None => { // This was expected to be a opening
                                stack.clear();
                                break;
                            }
                            Some(opening_idx) => {
                                if opening_idx == closing_idx {
                                    stack.pop();
                                } else { // Opening and closing doesn't match
                                    score1 += POINTS[closing_idx];
                                    stack.clear();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        if !stack.is_empty() { // Incomplete
            score2s.push(0);
            while !stack.is_empty() {
                let item = stack.pop().unwrap();
                *score2s.last_mut().unwrap() *= 5;
                *score2s.last_mut().unwrap() += 1 + OPENINGS.iter().position(|&i| i == item).unwrap() as u64;
            }
        }
    }
    score2s.sort();
    println!("1: {}\n2: {}", score1, score2s[score2s.len() / 2]);
}
