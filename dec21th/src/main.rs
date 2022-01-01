use std::collections::HashMap;

struct DetDice(u32, u32);
#[rustfmt::skip]
impl DetDice {
    fn new() -> Self { Self(0, 0) }
    fn give(&mut self) -> u32 {
        let mut t = 0;
        for _ in 0..3 {
            self.1 += 1;
            t += (self.0 % 100) + 1;
            self.0 += 1;
        }
        t
    }
    fn rolls(&self) -> u32 { self.1 }
}
struct Player(u32, u32);
#[rustfmt::skip]
impl Player {
    fn new(init: u32) -> Self { Self(init, 0) }
    fn roll(&mut self, dice: &mut DetDice) -> bool {
        let v = dice.give();
        self.0 = (v + self.0 - 1) % 10 + 1;
        self.1 += self.0;
        self.1 >= 1000
    }
    fn score(&self) -> u32 { self.1 }
}
fn main() {
    let mut finished = false;
    let mut dice = DetDice::new();
    let mut players = vec![Player::new(2), Player::new(1)];
    'first: loop {
        for player in players.iter_mut() {
            if finished {
                println!("1: {}", player.score() * dice.rolls());
                break 'first;
            }
            if player.roll(&mut dice) {
                finished = true;
            }
        }
    }

    let mut wins = [0u64, 0u64];
    play(&drawfrequencies(), &mut wins, (2, 0), (1, 0), 1, false);
    println!("2: {:?}", wins.iter().max().unwrap());
}
fn drawfrequencies() -> Vec<(u8, u64)> {
    let mut m: HashMap<u8, _> = HashMap::new();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                if let Some(x) = m.get_mut(&(i + j + k)) {
                    *x += 1;
                } else {
                    m.insert(i + j + k, 1);
                }
            }
        }
    }
    Vec::from_iter(m.into_iter())
}
fn play(
    df: &Vec<(u8, u64)>,
    wins: &mut [u64; 2],
    p0: (u8, u8),
    p1: (u8, u8),
    acc: u64,
    turn: bool,
) {
    for (draw, freq) in df {
        if turn {
            let p = (draw + p1.0 - 1) % 10 + 1;
            let p = (p, p1.1 + p);
            if p.1 >= 21 {
                wins[1] += acc * freq;
            } else {
                play(df, wins, p0, p, acc * freq, !turn);
            }
        } else {
            let p = (draw + p0.0 - 1) % 10 + 1;
            let p = (p, p0.1 + p);
            if p.1 >= 21 {
                wins[0] += acc * freq;
            } else {
                play(df, wins, p, p1, acc * freq, !turn);
            }
        }
    }
}
