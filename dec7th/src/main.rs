fn main() {
    let data = include_str!("input.txt")
        .split(['\n', ','].as_ref())
        .filter(|x| x.len() > 0)
        .map(|y| y.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut minfuel1 = (i32::MAX, 0i32);
    let mut minfuel2 = (i32::MAX, 0i32);
    for x in *data.iter().min().unwrap()..=*data.iter().max().unwrap() {
        let fuel1: i32 = data.iter().map(|subm| (subm - x).abs()).sum();
        let fuel2: i32 = data
            .iter()
            .map(|subm| {
                let diff = (subm - x).abs();
                diff * (diff + 1) / 2
            })
            .sum();
        if fuel1 < minfuel1.0 {
            minfuel1 = (fuel1, x);
        }
        if fuel2 < minfuel2.0 {
            minfuel2 = (fuel2, x);
        }
    }
    println!(
        "1: position {} will require minimum fuel {}",
        minfuel1.1, minfuel1.0
    );
    println!(
        "2: position {} will require minimum fuel {}",
        minfuel2.1, minfuel2.0
    );
}
