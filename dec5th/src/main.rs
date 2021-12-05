use std::{fmt, env};

const DEBUG: bool = false;

struct Space {
    item: Vec<Vec<u32>>,
}
impl Space {
    fn new(bounds: &((usize, usize), (usize, usize))) -> Self {
        Space {
            item: vec![vec![0u32; bounds.1 .0 - bounds.0 .0 + 1]; bounds.1 .1 - bounds.0 .1 + 1],
        }
    }
    fn count(&self) -> usize {
        self.item
            .iter()
            .map(|x| x.iter().filter(|&&v| v > 1).count())
            .sum::<usize>()
    }
}
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr = String::new();
        for row in &self.item {
            for item in row {
                repr.push_str(&format!("{:3}", item));
            }
            repr.push_str("\n");
        }
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn draw(&self, space: &mut Space) {
        let xdiff = (self.to.x as i32) - (self.from.x as i32);
        let ydiff = (self.to.y as i32) - (self.from.y as i32);
        let maxd = if xdiff.abs() > ydiff.abs() { xdiff.abs() } else { ydiff.abs() };
        let dx = f64::from(xdiff) / f64::from(maxd);
        let dy = f64::from(ydiff) / f64::from(maxd);
        for step in 0..=maxd {
            space.item[(f64::from(self.from.y as u32) + f64::from(step) * dy).round() as usize]
                      [(f64::from(self.from.x as u32) + f64::from(step) * dx).round() as usize] += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = match args.len() {
        2 => {
            match args[1].as_str() {
                "test" => include_str!("test.txt"),
                "input" => include_str!("input.txt"),
                _ => panic!("Only test or input accepted"),
            }
        },
        _ => panic!("Add an argument to the call"),
    };
    let mut bounds = ((0, 0), (0, 0));
    // Do this here (no constructors) and take the loop to calculate the boundaries
    let lines: Vec<Line> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            let mut rawl = x
                .split(" -> ")
                .map(|p| {
                    let rawp: Vec<usize> =
                        p.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
                    if bounds.1 .0 < rawp[0] { bounds.1 .0 = rawp[0]; }
                    if bounds.1 .1 < rawp[1] { bounds.1 .1 = rawp[1]; }
                    Point {
                        x: rawp[0],
                        y: rawp[1],
                    }
                })
                .collect::<Vec<Point>>()
                .into_iter();
            Line {
                from: rawl.next().unwrap(),
                to: rawl.next().unwrap(),
            }
        })
        .collect();

    let mut space = Space::new(&bounds);
    for line in &lines {
        if line.from.x == line.to.x || line.from.y == line.to.y {
            line.draw(&mut space);
        }
    }
    if DEBUG {println!("{}", space);}
    println!("1: {}", space.count());

    for line in &lines {
        if line.from.x != line.to.x && line.from.y != line.to.y {
            line.draw(&mut space);
        }
    }
    if DEBUG {println!("{}", space);}
    println!("2: {}", space.count());
}
