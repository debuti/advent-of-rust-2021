const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

#[derive(Debug)]
struct Board {
    id: usize,
    row_nb: usize,
    column_nb: usize,
    number_lyr: Vec<Vec<u32>>,
    crosses_lyr: Vec<Vec<bool>>,
    won: bool,
}

impl Board {
    fn parse(id: usize, input: &str) -> Board {
        let number_lyr: Vec<Vec<u32>> = input
            .split("\n")
            .map(|x| {
                x.split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|z| z.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
        let row_nb = number_lyr.len();
        let column_nb = number_lyr.iter().nth(0).unwrap().len();
        Board {
            id: id,
            row_nb: row_nb,
            column_nb: column_nb,
            number_lyr: number_lyr,
            crosses_lyr: vec![vec![false; column_nb]; row_nb],
            won: false,
        }
    }
    fn cross(&mut self, draw: &u32) -> bool {
        if self.won {
            return false;
        }
        let mut rowidx = 0;
        for row in &self.number_lyr {
            let mut columnidx = 0;
            for item in row {
                if draw == item {
                    debugln!(
                        "Crossing {} at board {} - {}:{}!",
                        draw,
                        self.id,
                        rowidx,
                        columnidx
                    );
                    self.crosses_lyr[rowidx][columnidx] = true;
                }
                columnidx += 1;
            }
            rowidx += 1;
        }
        self.won = self.checkcolumns() | self.checkrows();
        self.won
    }
    fn checkcolumns(&self) -> bool {
        for idx in 0..self.column_nb {
            if self.checkcolumn(idx) {
                return true;
            }
        }
        false
    }
    fn checkcolumn(&self, idx: usize) -> bool {
        for row in &self.crosses_lyr {
            let mut columnidx = 0;
            for item in row {
                if columnidx == idx && *item == false {
                    return false;
                }
                columnidx += 1;
            }
        }
        true
    }
    fn checkrows(&self) -> bool {
        for idx in 0..self.row_nb {
            if self.checkrow(idx) {
                return true;
            }
        }
        false
    }
    fn checkrow(&self, idx: usize) -> bool {
        let mut rowidx = 0;
        for row in &self.crosses_lyr {
            if rowidx == idx {
                for item in row {
                    if *item == false {
                        return false;
                    }
                }
            }
            rowidx += 1;
        }
        true
    }
    fn score(&self, draw: u32) -> u32 {
        let mut sum = 0;
        let mut rowidx = 0;
        for row in &self.number_lyr {
            let mut columnidx = 0;
            for _ in row {
                if !self.crosses_lyr[rowidx][columnidx] {
                    sum += self.number_lyr[rowidx][columnidx];
                }
                columnidx += 1;
            }
            rowidx += 1;
        }
        sum * draw
    }
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("input.txt"));
    let mut data: Vec<&str> = data.split("\n\n").filter(|x| x.len() > 0).collect();
    let random: Vec<u32> = data
        .remove(0)
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    debugln!("{:?}", random);
    let mut boards: Vec<Board> = data
        .iter()
        .enumerate()
        .map(|(i, x)| Board::parse(i, x))
        .collect();
    debugln!("{:?}", boards);

    for draw in &random {
        for board in &mut boards {
            if board.cross(draw) {
                println!(
                    "Board {} won with score {}!",
                    board.id,
                    board.score(*draw)
                );
            }
        }
    }
    debugln!("{:?}", boards);
}
