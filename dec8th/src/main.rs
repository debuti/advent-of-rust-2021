fn main() {
    /*
     * Byte is created as 0gfedcba, so 'be' would be 00010010 = 18 and 'cfbegad' would be 01111111 = 127
     */
    let code2byte = |&x: &&str| {
        (0..7).map(|i|{
            if x.chars().any(|y| y == (97u8 + i) as char) {1 << i}
            else {0}
        }).fold(0u8, |s,v| s|v)
    };
    let data = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|y| {
            let mut parts = y.split(" | ").collect::<Vec<&str>>();
            let first = parts
                .remove(0)
                .split(" ")
                .map(|z| code2byte(&z))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            let second = parts
                .remove(0)
                .split(" ")
                .map(|z| code2byte(&z))
                .collect::<Vec<u8>>()
                .into_iter()
                .rev()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            (first, second)
        })
        .collect::<Vec<([u8; 10], [u8; 4])>>();

    println!(
        "1: {}",
        data.iter()
            .map(|x| {
                x.1.iter()
                    .filter(|y| [2,3,4,7].contains(&y.count_ones()))
                    .count()
            })
            .sum::<usize>()
    );

    fn assert_and_get<F>(input:&[u8], check: F) -> u8 where F : Fn(&&u8)->bool{
        let seq : Vec<&u8> = input.iter().filter(check).collect();
        assert_eq!(1, seq.len());
        **seq.iter().next().unwrap()
    }

    println!("2: {}", data.iter().map(|sample| {
        let mut codes = [0u8;10];
        codes[1] = assert_and_get(&sample.0, |x| x.count_ones() == 2); 
        codes[7] = assert_and_get(&sample.0, |x| x.count_ones() == 3);
        codes[4] = assert_and_get(&sample.0, |x| x.count_ones() == 4);
        codes[8] = assert_and_get(&sample.0, |x| x.count_ones() == 7);
        codes[3] = assert_and_get(&sample.0, |&&x| x.count_ones() == 5 && x|codes[1]==x);
        codes[9] = assert_and_get(&sample.0, |&&x| x.count_ones() == 6 && x|codes[4]==x);
        codes[6] = assert_and_get(&sample.0, |&&x| x.count_ones() == 6 && x|codes[7]!=x);
        codes[0] = assert_and_get(&sample.0, |&&x| x.count_ones() == 6 && (x!=codes[9] && x!=codes[6]));
        codes[5] = assert_and_get(&sample.0, |&&x| x.count_ones() == 5 && x|codes[6]==codes[6]);
        codes[2] = assert_and_get(&sample.0, |&&x| x.count_ones() == 5 && (x!=codes[3] && x!=codes[5]));

        (0..4).map(|i| (codes.iter().position(|&r| r == sample.1[i]).unwrap())*10usize.pow(i as u32)).sum::<usize>()
    }).sum::<usize>());
}
