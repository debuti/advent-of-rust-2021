fn main() {
    let mut data = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|y| {
            y.chars()
                .map(|z| z.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let (w, h) = (data.iter().nth(0).unwrap().len(), data.len());

    let mut sum = 0;
    let mut basinseeds: Vec<(usize, usize)> = Vec::new();
    for (rowidx, row) in data.iter().enumerate() {
        for (colidx, item) in row.iter().enumerate() {
            if [
                if colidx < (w - 1) { data[rowidx][colidx + 1] } else { u32::MAX },
                if colidx > 0       { data[rowidx][colidx - 1] } else { u32::MAX },
                if rowidx > 0       { data[rowidx - 1][colidx] } else { u32::MAX },
                if rowidx < (h - 1) { data[rowidx + 1][colidx] } else { u32::MAX },
            ]
            .iter()
            .min()
            .unwrap() > item
            {
                basinseeds.push((rowidx, colidx));
                sum += 1 + item;
            }
        }
    }
    println!("1: {}", sum);

    if (true) {
        println!("\nEnter these commands in octave to render the depthmap:");
        println!("[x,y] = meshgrid(linspace(0, {}, {}), linspace(0, {}, {}));", w-1, w, h-1, h);
        let tmp = format!("{:?}", data).replace("], [","; ").replace("]]","]").replace("[[","[").replace(", "," ");
        println!("z= {};", tmp);
        println!("contour(x,y,z);");
    }

    let mut basins: Vec<usize> = basinseeds
        .iter()
        .map(|b| discover(&mut data, (w, h), *b))
        .collect::<Vec<usize>>();
    basins.sort();
    basins.reverse();
    println!("2: {}", basins.iter().take(3).product::<usize>());
}

fn discover(data: &mut Vec<Vec<u32>>, (w, h): (usize, usize), (y, x): (usize, usize)) -> usize {
    if data[y][x] == 9 {
        return 0;
    }
    data[y][x] = 9;
    1
    + if y > 0     { discover(data, (w, h), (y - 1, x)) } else { 0 }
    + if x > 0     { discover(data, (w, h), (y, x - 1)) } else { 0 }
    + if y < h - 1 { discover(data, (w, h), (y + 1, x)) } else { 0 }
    + if x < w - 1 { discover(data, (w, h), (y, x + 1)) } else { 0 }
}
