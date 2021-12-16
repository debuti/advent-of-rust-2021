use std::collections::HashMap;

type Coord = (usize, usize);

fn main() {
    let map: Vec<Vec<u32>> = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let (w, h) = (map.iter().nth(0).unwrap().len(), map.len());

    println!("1: {}", dijkstra(&map, (w, h), (0, 0)));

    let mut fullmap: Vec<Vec<u32>> = vec![vec![0; w * 5]; h * 5];
    for row in 0..h {
        for column in 0..5 * w {
            fullmap[row][column] = map[row][column % w] + (column / w) as u32;
            if fullmap[row][column] > 9 {
                fullmap[row][column] -= 9;
            }
        }
    }
    for row in h..5 * h {
        for column in 0..5 * w {
            fullmap[row][column] = fullmap[row % h][column] + (row / h) as u32;
            if fullmap[row][column] > 9 {
                fullmap[row][column] -= 9;
            }
        }
    }
    let (w, h) = (fullmap.iter().nth(0).unwrap().len(), fullmap.len());
    if false {
        println!("{:?}", (w, h));
        for row in 0..5 * h {
            println!("{:?}", fullmap[row]);
        }
    }

    println!("2: {}", dijkstra(&fullmap, (w, h), (0, 0)));
}

fn dijkstra(map: &Vec<Vec<u32>>, (w, h): (usize, usize), start: (usize, usize)) -> u32 {
    let mut pos: Coord = start;
    let mut nodes: HashMap<Coord, (u32, Option<Coord>, bool)> = vec![vec![0u8; w]; h]
        .iter()
        .enumerate()
        .map(|(ri, r)| {
            r.iter()
                .enumerate()
                .map(|(ci, _)| ((ri, ci), (u32::MAX, None, false)))
                .collect()
        })
        .collect::<Vec<Vec<(Coord, (u32, Option<Coord>, bool))>>>()
        .into_iter()
        .flatten()
        .collect();

    if let Some(v) = nodes.get_mut(&pos) {
        *v = (0, None, true);
    }

    loop {
        let current = *nodes.get(&pos).unwrap();
        for dir in directions(&map, (w, h), pos) {
            if let Some(dir) = dir {
                if let Some(v) = nodes.get_mut(&dir.0) {
                    if !v.2 && v.0 > current.0 + dir.1 {
                        *v = (current.0 + dir.1, Some(pos), v.2);
                    }
                }
            }
        }
        let new = nodes
            .iter_mut()
            .filter(|x| !x.1 .2)
            .min_by(|x, y| x.1 .0.cmp(&y.1 .0))
            .unwrap();
        pos = *new.0;
        *new.1 = (new.1 .0, new.1 .1, true);
        if pos == (w - 1, h - 1) {
            break;
        }
    }

    if false {
        while pos != (0, 0) {
            print!("{:?}, ", pos);
            pos = nodes.get(&pos).unwrap().1.unwrap();
        }
        println!();
    }

    nodes.get(&(w - 1, h - 1)).unwrap().0
}

fn directions(map: &Vec<Vec<u32>>, (w, h): Coord, pos: Coord) -> [Option<(Coord, u32)>; 4] {
    let mut directions = [None, None, None, None];
    if pos.0 > 0 {
        let left = (pos.0 - 1, pos.1);
        directions[0] = Some((left, map[left.1][left.0]));
    }
    if pos.1 > 0 {
        let up = (pos.0, pos.1 - 1);
        directions[1] = Some((up, map[up.1][up.0]));
    }
    if pos.0 < h - 1 {
        let right = (pos.0 + 1, pos.1);
        directions[2] = Some((right, map[right.1][right.0]));
    }
    if pos.1 < w - 1 {
        let down = (pos.0, pos.1 + 1);
        directions[3] = Some((down, map[down.1][down.0]));
    }
    directions
}
