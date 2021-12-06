fn main() {
    let data = include_str!("input.txt");
    let mut population_by_day = vec![0u64; 9];
    for item in data.
        split(['\n', ','].as_ref())
        .filter(|x| x.len() > 0)
        .map(|y| y.parse().unwrap())
        .collect::<Vec<usize>>()
    {
        population_by_day[item] += 1;
    }

    for day in 1..=256 {
        let natality = population_by_day[0];
        for idx in 0..=8 {
            population_by_day[idx] = if idx < 8 {
                population_by_day[idx + 1]
            } else {
                0
            } + if idx == 6 || idx == 8 { natality } else { 0 }
        }
        println!("Day {:3}: {}", day, population_by_day.iter().sum::<u64>());
    }
}

