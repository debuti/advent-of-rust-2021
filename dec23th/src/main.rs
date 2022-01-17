const DEPTH: usize = 4;
type Pos = (usize, usize);
#[derive(Debug, Copy, Clone)]
struct Board {
    pos: [[Pos; DEPTH]; 4],
    strolls: [[u8; DEPTH]; 4],
}
#[rustfmt::skip]
impl Board {
    fn new1(initial: &Vec<Vec<char>>) -> Self {
        let search = |t| {
            let mut inp = initial
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    l.iter()
                     .enumerate()
                     .filter(|(_, c)| **c == t)
                     .map(|(j, _)| (i, j))
                     .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let c = (t as usize - 'A' as usize) * 2 + 3;
            let f = vec![(4usize, c), (5usize, c)];
            inp.extend(f);
            inp.try_into().unwrap()
        };
        Self {
            pos: [search('A'), search('B'), search('C'), search('D')],
            strolls: [[0; DEPTH]; 4],
        }
    }
    fn new2(initial: &Vec<Vec<char>>) -> Self {
        let search = |t| {
            let mut inp = initial
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    l.iter()
                     .enumerate()
                     .filter(|(_, c)| **c == t)
                     .map(|(j, _)| (if i==2 {2} else {5}, j))
                     .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            if t == 'A' { inp.extend(vec![(3,9), (4,7)]); }
            if t == 'B' { inp.extend(vec![(3,7), (4,5)]); }
            if t == 'C' { inp.extend(vec![(3,5), (4,9)]); }
            if t == 'D' { inp.extend(vec![(3,3), (4,3)]); }
            inp.try_into().unwrap()
        };
        Self {
            pos: [search('A'), search('B'), search('C'), search('D')],
            strolls: [[0; DEPTH]; 4],
        }
    }
    fn something(&self, p: Pos) -> Option<usize> {
        for t in 0..=3 {
            for n in 0..DEPTH {
                if self.pos[t][n] == p {return Some(t);}
            }
        }
        None
    }
    fn walkv(&self, from: Pos, to: Pos) -> Option<u32> {
        let mut curr = from;
        for r in if from.0<to.0 {from.0+1..=to.0} else {to.0..=from.0-1} {
            curr.0 = r;
            if self.something(curr).is_some() {
                return None;
            }
        }
        Some((if from.0>to.0 {from.0-to.0} else {to.0-from.0}) as _)
    }
    fn walkh(&self, from: Pos, to: Pos) -> Option<u32> {
        let mut curr = from;
        for c in if from.1<to.1 {from.1+1..=to.1} else {to.1..=from.1-1} {
            curr.1 = c;
            if self.something(curr).is_some() {
                return None;
            }
        }
        Some((if from.1>to.1 {from.1-to.1} else {to.1-from.1} ) as _)
    }
    fn futures(&self, acc: u32, min: u32) -> Vec<(Self, u32)> {
        let mut tmp = Vec::new();
        for t in 0..=3 {
            'next: for n in 0..DEPTH {
                if self.strolls[t][n] == 2 { // No more moves
                    continue;
                } else if self.strolls[t][n] == 1 { // Move only to your room
                    let c_dst = (t*2)+3;

                    // Check that room is empty or only with family
                    for r in (2..2+DEPTH).rev() {
                      if let Some(app) = self.something((r, c_dst)) {
                          if app != t { continue 'next;}
                      }
                      else {
                        let dst = (r, c_dst);
                        // Go to the deepest inside your room, if not blocked by anybody. Only if your room is empty or there are alikes
                        if let Some(costh) = self.walkh(self.pos[t][n], (1, c_dst)) {
                            if let Some(costv) = self.walkv((1, c_dst), dst) {
                                let cost = (costh+costv)*(10u32.pow(t as u32));
                                if acc + cost < min {
                                    let mut wot = self.clone();
                                    wot.pos[t][n] = dst;
                                    wot.strolls[t][n] += 1;
                                    tmp.push((wot, cost));
                                }
                            }
                        }
                        continue 'next;
                      }
                    }

                } else { // Get out of the room
                    // If already in dest room, and not blocking anybody, then stay.
                    if t == (self.pos[t][n].1-3)/2 {
                        let mut allgood = true;
                        for x in self.pos[t][n].0+1..2+DEPTH {
                            if let Some(app) = self.something((x, self.pos[t][n].1)) {
                                if app != t {allgood = false;} 
                            }                          
                        }
                        if allgood { continue 'next; }
                    }
                    // If not, go to hallway, to any reachable spot.
                    if let Some(costv) = self.walkv(self.pos[t][n], (1, self.pos[t][n].1)) {        
                        for c in 1..=11 {
                            if c == 3 || c == 5 || c == 7  || c == 9 {continue;}
                            let dst = (1, c);
                            if let Some(costh) = self.walkh((1, self.pos[t][n].1), dst) {
                                let cost = (costh+costv)*(10u32.pow(t as u32));
                                if acc + cost < min {
                                    let mut wot = self.clone();
                                    wot.pos[t][n] = dst;
                                    wot.strolls[t][n] += 1;
                                    tmp.push((wot, cost));
                                }
                            }
                        }
                    }
                }
            }
        }
        tmp
    }
    fn iswin(&self) -> bool {
        for t in 0..=3 {
            for n in 0..DEPTH {
                if self.pos[t][n].1 != (t*2)+3 {return false;}
            }
        }
        true
    }
    fn print(&self) {
        for row in 0..3+DEPTH {
            'next: for col in 0..=12 {
                if row == 0 || row == 2+DEPTH || col == 0 || col == 12
                    || (row >= 2 && (col < 3 || col % 2 == 0 || 9 < col)) {
                    print!("#");
                } else {
                    for t in 0..=3 {
                        for i in 0..DEPTH {
                            if self.pos[t][i] == (row, col) {
                                print!("{}",
                                    std::char::from_u32(('A' as usize + t).try_into().unwrap())
                                        .unwrap()
                                );
                                continue 'next;
                            }
                        }
                    }
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let positions = include_str!("input.txt")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let board = Board::new1(&positions);
    board.print();
    let mut min = u32::MAX;
    play(board, &mut min, 0);
    println!("1: {}", min);

    let board = Board::new2(&positions);
    board.print();
    let mut min = u32::MAX;
    play(board, &mut min, 0);
    println!("2: {}", min);
}

fn play(board: Board, min: &mut u32, acc: u32) {
    if board.iswin() {
        if acc < *min {
            *min = acc;
        }
        return;
    }
    for (b, cost) in board.futures(acc, *min) {
        play(b, min, acc + cost);
    }
}
