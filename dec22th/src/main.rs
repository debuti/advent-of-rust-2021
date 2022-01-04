use std::collections::HashSet;

type Range = [i32; 2];
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}
#[rustfmt::skip]
impl Cuboid {
    fn new(ranges: [Range; 3]) -> Self {
        Cuboid { x: ranges[0], y: ranges[1], z: ranges[2], }
    }
    fn cubes(&self) -> u64 {
        (self.x[1]+1-self.x[0]).abs() as u64
      * (self.y[1]+1-self.y[0]).abs() as u64
      * (self.z[1]+1-self.z[0]).abs() as u64
    }
    fn isinit(&self) -> bool {
        (((-50 <= self.x[1]) && (self.x[1] <= 50)) || ((-50 <= self.x[0]) && (self.x[0] <= 50)))
     && (((-50 <= self.y[1]) && (self.y[1] <= 50)) || ((-50 <= self.y[0]) && (self.y[0] <= 50)))
     && (((-50 <= self.z[1]) && (self.z[1] <= 50)) || ((-50 <= self.z[0]) && (self.z[0] <= 50)))
    }
    fn mid(&self) -> [i32;3] {
        [(self.x[0] + self.x[1])/2,
         (self.y[0] + self.y[1])/2,
         (self.z[0] + self.z[1])/2,]
    }
    fn contains(&self, cube: [i32;3]) -> bool {
        self.x[0]<=cube[0] && cube[0]<=self.x[1] 
     && self.y[0]<=cube[1] && cube[1]<=self.y[1] 
     && self.z[0]<=cube[2] && cube[2]<=self.z[1] 
    }
    fn overlaps(&self, other: &Self) -> bool {
        ((self.x[0] <= other.x[0] && other.x[0] <= self.x[1]) || (self.x[0] <= other.x[1] && other.x[1] <= self.x[1]) || (other.x[0] <= self.x[0] && self.x[1] <= other.x[1])) 
     && ((self.y[0] <= other.y[0] && other.y[0] <= self.y[1]) || (self.y[0] <= other.y[1] && other.y[1] <= self.y[1]) || (other.y[0] <= self.y[0] && self.y[1] <= other.y[1])) 
     && ((self.z[0] <= other.z[0] && other.z[0] <= self.z[1]) || (self.z[0] <= other.z[1] && other.z[1] <= self.z[1]) || (other.z[0] <= self.z[0] && self.z[1] <= other.z[1])) 
    }
    fn split(self, other:&Self) -> HashSet<Self> {
        let mut t = HashSet::new();

        let mut xcuts = vec![self.x[0]];
        if self.x[0]  <  other.x[0] && other.x[0] <= self.x[1] { xcuts.push(other.x[0]-1); xcuts.push(other.x[0]); }
        if self.x[0]  <=  other.x[1] && other.x[1] < self.x[1] { xcuts.push(other.x[1]); xcuts.push(other.x[1]+1); }
        xcuts.push(self.x[1]);

        let mut ycuts = vec![self.y[0]];
        if self.y[0] <  other.y[0] && other.y[0] <= self.y[1] { ycuts.push(other.y[0]-1); ycuts.push(other.y[0]); }
        if self.y[0] <= other.y[1] && other.y[1] <  self.y[1] { ycuts.push(other.y[1]); ycuts.push(other.y[1]+1); }
        ycuts.push(self.y[1]);

        let mut zcuts = vec![self.z[0]];
        if self.z[0] <  other.z[0] && other.z[0] <= self.z[1] { zcuts.push(other.z[0]-1); zcuts.push(other.z[0]); }
        if self.z[0] <= other.z[1] && other.z[1] <  self.z[1] { zcuts.push(other.z[1]); zcuts.push(other.z[1]+1); }
        zcuts.push(self.z[1]);

        for xcutidx in (0..xcuts.len()).step_by(2) {
            for ycutidx in (0..ycuts.len()).step_by(2) {
                for zcutidx in (0..zcuts.len()).step_by(2) {
                    let c = Cuboid{x:[xcuts[xcutidx+0], xcuts[xcutidx+1]],
                                   y:[ycuts[ycutidx+0], ycuts[ycutidx+1]],
                                   z:[zcuts[zcutidx+0], zcuts[zcutidx+1]]};
                    if !other.contains(c.mid()) {
                        t.insert(c);
                    }
                }
            }
        }
        t
    }
}
impl std::fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x={}..{}, y={}..{}, z={}..{})",
            self.x[0], self.x[1], self.y[0], self.y[1], self.z[0], self.z[1],
        )
    }
}
type Instruction = (bool, Cuboid);
fn main() {
    let data = include_str!("input.txt")
        .split("\n")
        .filter(|l| l.len() > 0 && l.chars().nth(0).unwrap() != '#');
    let instructions = data
        .map(|l| {
            let parts = l.split_once(" ").unwrap();
            (
                parts.0 == "on",
                Cuboid::new(
                    parts
                        .1
                        .split(',')
                        .map(|c| {
                            c.split_once('=')
                                .unwrap()
                                .1
                                .split("..")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect::<Vec<_>>()
                                .as_slice()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<Range>>()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                ),
            )
        })
        .collect::<Vec<Instruction>>();

    println!("1: {}", process(&instructions, true));
    println!("2: {}", process(&instructions, false));
}

fn process(instructions: &Vec<Instruction>, initseq: bool) -> u64 {
    let mut core: HashSet<Cuboid> = HashSet::new();
    for instruction in instructions {
        if !initseq || (initseq && instruction.1.isinit()) {
            let mut tmp: HashSet<Cuboid> = HashSet::new();
            for cuboid in core.into_iter() {
                if cuboid.overlaps(&instruction.1) {
                    tmp.extend(cuboid.split(&instruction.1));
                } else {
                    tmp.insert(cuboid);
                }
            }
            if instruction.0 {
                tmp.insert(instruction.1);
            }
            core = tmp;
        }
    }
    core.iter().fold(0, |acc, x| x.cubes() + acc)
}