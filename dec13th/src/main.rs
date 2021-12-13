use std::collections::HashSet;

fn main() {
    let parts: [&str; 2] = include_str!("input.txt")
        .split("\n\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>()
        .as_slice()
        .try_into()
        .unwrap();

    let mut coords = parts[0]
        .split("\n")
        .map(|x| {
            let mut t = x.split(",").map(|c| c.parse::<u32>().unwrap());
            (t.next().unwrap(), t.next().unwrap())
        })
        .collect::<HashSet<(u32, u32)>>();

    let instructions = parts[1]
        .split("\n")
        .map(|s| {
            let mut t = s.split(" ").nth(2).unwrap().split("=");
            (
                t.next().unwrap().chars().nth(0).unwrap(),
                t.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(char, u32)>>();

    for (instidx, inst) in instructions.iter().enumerate() {
        coords = fold(&coords, *inst);
        //print(&coords);
        if instidx == 0 {
            println!("1: {}", coords.len());
        }
    }
    println!("2:");
    print(&coords);
}

fn fold(coords: &HashSet<(u32, u32)>, inst: (char, u32)) -> HashSet<(u32, u32)> {
    let mut new: HashSet<(u32, u32)> = HashSet::new();
    if inst.0 == 'y' {
        // Fold horizontally
        for c in coords.iter() {
            if c.1 < inst.1 {
                new.insert(*c);
            } else {
                new.insert((c.0, (inst.1 - (c.1 - inst.1))));
            }
        }
    }
    if inst.0 == 'x' {
        // Fold vertically
        for c in coords.iter() {
            if c.0 < inst.1 {
                new.insert(*c);
            } else {
                new.insert(((inst.1 - (c.0 - inst.1)), c.1));
            }
        }
    }
    new
}

fn print(coords: &HashSet<(u32, u32)>) {
    let (w, h) = coords.iter().fold((0, 0), |acc, elem| {
        (
            if elem.0 > acc.0 { elem.0 } else { acc.0 },
            if elem.1 > acc.1 { elem.1 } else { acc.1 },
        )
    });
    for y in 0..=h {
        for x in 0..=w {
            print!("{}", if coords.contains(&(x, y)) { "#" } else { "." });
        }
        println!("");
    }
    println!("");
}
