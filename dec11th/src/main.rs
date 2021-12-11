use std::{thread, time};

const VISUALIZE_RATE: u64 = 200;

fn main() {
    let mut data = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let (w, h) = (data[0].len(), data.len());

    let mut flashes = 0;
    let mut step = 0;
    let mut not_yet_in_sync = true;
    while not_yet_in_sync {
        step += 1;
        for rowidx in 0..h {
            for colidx in 0..w {
                data[rowidx][colidx] += 1;
                if data[rowidx][colidx] == 10 {
                    flashes += flash(&mut data, (w, h), (colidx, rowidx));
                }
            }
        }
        for row in &mut data {
            for octopus in row {
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
        if VISUALIZE_RATE > 0 {
            print!("\x1B[2J");
            for rowidx in 0..h {
                for colidx in 0..w {
                    print!("{}{}{}", if data[rowidx][colidx]==0 {"\x1b[0;31m"}else{""}, data[rowidx][colidx], if data[rowidx][colidx]==0 {"\x1b[0m"}else{""});
                }
                println!("");
            }
            thread::sleep(time::Duration::from_millis(VISUALIZE_RATE));
        }
        if step == 100 {
            println!("1: {}", flashes);
        }
        not_yet_in_sync = data.iter().map(|r| r.iter().map(|&i| i as u32).sum::<u32>()).sum::<u32>() != 0;
    }
    println!("2: {}", step);
}

fn flash(data: &mut Vec<Vec<u8>>, (w, h): (usize, usize), (x, y): (usize, usize)) -> u32 {
    if data[y][x] < 9 {
        data[y][x] += 1;
        return 0;
    } else if data[y][x] == 9 || data[y][x] == 10 {
        data[y][x] = 11;
        return 1
            + if x > 0     && y > 0     { flash(data, (w, h), (x - 1, y - 1)) } else { 0 }
            + if              y > 0     { flash(data, (w, h), (x    , y - 1)) } else { 0 }
            + if x < w - 1 && y > 0     { flash(data, (w, h), (x + 1, y - 1)) } else { 0 }
            + if x > 0                  { flash(data, (w, h), (x - 1, y))     } else { 0 }
            + if x < w - 1              { flash(data, (w, h), (x + 1, y))     } else { 0 }
            + if x > 0     && y < h - 1 { flash(data, (w, h), (x - 1, y + 1)) } else { 0 }
            + if              y < h - 1 { flash(data, (w, h), (x    , y + 1)) } else { 0 }            
            + if x < w - 1 && y < h - 1 { flash(data, (w, h), (x + 1, y + 1)) } else { 0 };
    } else {
        return 0;
    }
}
