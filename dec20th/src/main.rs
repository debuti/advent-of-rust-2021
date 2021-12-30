type Coord = (i32, i32);
type Image = (Vec<Coord>, bool);

fn main() {
    let parts = include_str!("input.txt").split_once("\n\n").unwrap();
    let iea: Vec<_> = parts.0.chars().map(|c| c == '#').collect();
    let mut image: Image = (
        parts
            .1
            .split("\n")
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(j, _)| (i as _, j as _))
                    .collect::<Vec<Coord>>()
            })
            .collect::<Vec<Vec<_>>>()
            .into_iter()
            .flatten()
            .collect(),
        false,
    );
    let will_play_inverts = iea[0];
    for i in 0..50 {
        image = update(&iea, image, will_play_inverts);
        if (i == 1) || (i == 49) {
            println!("{:?}", image.0.len());
        }
    }
}

#[rustfmt::skip]
fn imagedims(image: &Image) -> (Coord, Coord) {
    image.0.iter().fold(
        ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
        |mut acc, d| {
            if acc.0.0 > d.0 {acc.0.0=d.0;}
            if acc.0.1 > d.1 {acc.0.1=d.1;}
            if acc.1.0 < d.0 {acc.1.0=d.0;}
            if acc.1.1 < d.1 {acc.1.1=d.1;}
            acc
        },
    )
}

fn _imagedims_slightly_worse_performance(image: &Image) -> (Coord, Coord) {
    (
        (
            image.0.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
            image.0.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
        ),
        (
            image.0.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
            image.0.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
        ),
    )
}

#[rustfmt::skip]
fn value(image: &Image, (i, j): Coord, wpi: bool) -> usize {
    let t = if image.0.contains(&(i - 1, j - 1)) {1 << 8} else {0} + 
            if image.0.contains(&(i - 1, j    )) {1 << 7} else {0} + 
            if image.0.contains(&(i - 1, j + 1)) {1 << 6} else {0} +
            if image.0.contains(&(i    , j - 1)) {1 << 5} else {0} +
            if image.0.contains(&(i    , j    )) {1 << 4} else {0} + 
            if image.0.contains(&(i    , j + 1)) {1 << 3} else {0} + 
            if image.0.contains(&(i + 1, j - 1)) {1 << 2} else {0} + 
            if image.0.contains(&(i + 1, j    )) {1 << 1} else {0} + 
            if image.0.contains(&(i + 1, j + 1)) {1 << 0} else {0};
    (if !wpi || !image.1 { // Image saves #
        t
    } else {               // Image saves .
        !t & 0x1FF
    }) as _
}

fn update(iea: &Vec<bool>, image: Image, wpi: bool) -> Image {
    let ((lx, ly), (hx, hy)) = imagedims(&image);
    let mut t = Vec::new();
    for i in lx - 1..=hx + 1 {
        for j in ly - 1..=hy + 1 {
            let v = value(&image, (i, j), wpi);
            if !wpi || image.1 {
                if iea[v] {
                    t.push((i, j));
                }
            } else {
                if !iea[v] {
                    t.push((i, j));
                }
            }
        }
    }
    (t, !image.1)
}
