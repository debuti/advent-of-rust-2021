use std::collections::HashSet;

const COINCIDENCES_NB: usize = 12;


type Op = [[i32; 3]; 3];

//const RX   : Op = [[1, 0, 0], [0, cos x, -sen x], [0, sen x, cos x]];
const R000X: Op = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
const R090X: Op = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
const R180X: Op = [[1, 0, 0], [0, -1, 0], [0, 0, -1]];
//const R270X: Op = [[1, 0, 0], [0, 0, 1], [0, -1, 0]];

//const RY   : Op = [[cos x, 0, sen x], [0, 1, 0], [-sen x, 0, cos x]];
//const R000Y: Op = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
const R090Y: Op = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
//const R180Y: Op = [[-1, 0, 0], [0, 1, 0], [0, 0, -1]];
//const R270Y: Op = [[0, 0, -1], [0, 1, 0], [1, 0, 0]];

//const RZ   : Op = [[cos x, -sen x, 0], [sen x, cos x, 0], [0, 0, 1]];
//const R000Z: Op = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
const R090Z: Op = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];
const R180Z: Op = [[-1, 0, 0], [0, -1, 0], [0, 0, 1]];
//const R270Z: Op = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]];

const OPS: [Op; 24] = [
    R000X, R090X, R090X, R090X, R180Z, R090X, R090X, R090X, R090Z, R090Y, R090Y, R090Y, R180Z,
    R090Y, R090Y, R090Y, R090X, R090Z, R090Z, R090Z, R180X, R090Z, R090Z, R090Z,
];


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}
impl Coord {
    fn inv(&self) -> Self {
        Coord {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    fn add(&self, other: &Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    fn apply_op(&mut self, op: &Op) {
        // Y = M * X
        let (x, y, z) = (
            op[0][0] * self.x + op[0][1] * self.y + op[0][2] * self.z,
            op[1][0] * self.x + op[1][1] * self.y + op[1][2] * self.z,
            op[2][0] * self.x + op[2][1] * self.y + op[2][2] * self.z,
        );
        self.x = x;
        self.y = y;
        self.z = z;
    }
    fn inc(&mut self, t: &Self) {
        self.x += t.x;
        self.y += t.y;
        self.z += t.z;
    }
    fn taxicab(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}
impl TryFrom<Vec<i32>> for Coord {
    type Error = &'static str;

    fn try_from(v: Vec<i32>) -> Result<Self, Self::Error> {
        Ok(Coord {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
}


#[derive(Clone)]
struct Scanner {
    checked:bool,
    sidx: u32,
    location: Option<Coord>,
    beacons: Vec<Coord>,
}
impl std::fmt::Debug for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(idx:{},\nlocation:{:?},\nbeacons({}): {:?})",
            self.sidx,
            self.location,
            self.beacons.len(),
            self.beacons,
        )
    }
}
impl Scanner {
    fn new(sidx: u32, beacons: Vec<Coord>) -> Self {
        Scanner {
            checked: false,
            sidx,
            location: if sidx == 0 {
                Some(Coord { x: 0, y: 0, z: 0 })
            } else {
                None
            },
            beacons,
        }
    }
    fn apply_op(&mut self, op: &Op) {
        for beacon in self.beacons.iter_mut() {
            beacon.apply_op(op);
        }
    }
}

fn main() {
    let mut scanners = include_str!("input.txt")
        .split("\n\n")
        .filter(|l| l.len() > 0)
        .map(|s| {
            let parts = s.split_once("\n").unwrap();
            let sidx = parts.0.split(" ").nth(2).unwrap().parse().unwrap();
            let beacons = parts
                .1
                .split("\n")
                .map(|c| {
                    c.split(",")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            Scanner::new(sidx, beacons)
        })
        .collect::<Vec<_>>();

    let l = scanners.len();
    let mut lastcount = usize::MAX;
    /* Loop while there are unsolved scanners */
    while scanners.iter().filter(|s| s.location.is_none()).count() > 0 {
        if lastcount == scanners.iter().filter(|s| s.location.is_none()).count() {
            println!("Couldn't find solution");
            return;
        }
        lastcount = scanners.iter().filter(|s| s.location.is_none()).count();
        for s in scanners
            .iter()
            .enumerate()
            .filter(|(_, s)| s.location.is_some())
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
        {
            if scanners[s].checked { continue;}
            for u in 0..l {
                if scanners[u].location.is_some() {
                    continue;
                }
                versus(&mut scanners, s, u);
            }
            scanners[s].checked = true;
        }
    }
    //println!("\nScanners {:?}", scanners);
    println!(
        "1: {}",
        scanners
            .iter()
            .fold(HashSet::<Coord>::new(), |mut acc, s| {
                acc.extend(HashSet::<Coord>::from_iter(s.beacons.iter().cloned()));
                acc
            }).iter()
            .count()
    );

    let mut max = 0;
    for i in 0..l {
        for j in i..l {
            let d = scanners[i].location.as_ref().unwrap().taxicab(&scanners[j].location.as_ref().unwrap());
            if d > max {max = d}
        }
    }
    println!("2 {}", max);
}

fn versus(scanners: &mut Vec<Scanner>, fixed: usize, free: usize) {
    for op in OPS {
        scanners[free].apply_op(&op);
        if let Ok(coord) = findfit(&scanners[fixed], &scanners[free]) {
            scanners[free].location = Some(Coord { x: 0, y: 0, z: 0 }.add(&coord).clone());
            for beacon in scanners[free].beacons.iter_mut() {
                beacon.inc(&coord);
            }
            return;
        }
    }
}

fn findfit(fixed: &Scanner, free: &Scanner) -> Result<Coord, ()> {
    for fixedbeacon in &fixed.beacons {
        for freebeacon in &free.beacons {
            let mut count = 1;
            let freescanner = fixedbeacon.add(&freebeacon.inv());
            for (idx, freebeacon) in free.beacons.iter().enumerate() {
                if idx > 0 {
                    if fixed.beacons.contains(&freescanner.add(&freebeacon)) {
                        count += 1;
                        if count >= COINCIDENCES_NB {
                            return Ok(freescanner);
                        }
                    }
                }
            }
        }
    }
    Err(())
}
