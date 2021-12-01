fn main() {
    let qualifyfn = |y:&[u32]| y.windows(2).filter(|x| x[0]<x[1]).count();
    let data = String::from_utf8_lossy(include_bytes!("input.txt"));
    let heights : Vec<u32> = data.split("\n").filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap()).collect();
    println!("1: {}\n2: {}", qualifyfn(&heights), qualifyfn(&heights.windows(3).map(|x| x.iter().sum()).collect::<Vec<u32>>()));
}
