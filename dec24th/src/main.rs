// #[macro_use]
// extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::thread;

type Tuning = [i64; 3];

/* Replace this const with the static to get the values from input (less performance) */
const TUNINGS: [Tuning; 14] = [
    [1, 10, 2],
    [1, 10, 4],
    [1, 14, 8],
    [1, 11, 7],
    [1, 14, 12],
    [26, -14, 7],
    [26, 0, 10],
    [1, 10, 14],
    [26, -10, 2],
    [1, 13, 6],
    [26, -12, 8],
    [26, -3, 11],
    [26, -11, 5],
    [26, -2, 11],
];

// lazy_static! {
//     static ref TUNINGS: Vec<Tuning> = include_str!("input.txt")
//     .split("inp")
//     .filter(|c| c.len() > 0)
//     .map(|c| {
//         c.split('\n')
//             .enumerate()
//             .filter(|(i, _)| *i == 4 || *i == 5 || *i == 15)
//             .map(|(_, l)| l.split(' ').last().unwrap().parse::<i64>().unwrap())
//             .collect::<Vec<_>>()
//             .try_into()
//             .unwrap()
//     })
//     .collect::<Vec<_>>();
// }

fn algorithm(mut z: i64, tune: &Tuning, input: i64) -> i64 {
    // Code decompiled by hand
    // inp w     ## Input
    // mul x 0   ## Reset x
    // add x z   ## ..
    // mod x 26  ## x = z % 26
    // div z 1   .# z = z / 1
    // add x 10  .# x = 10 + (z%26)
    // eql x w   ## ..
    // eql x 0   ## x = (x!=w)
    // mul y 0   ## Reset y
    // add y 25  ## ..
    // mul y x   ## ..
    // add y 1   ## ..
    // mul z y   ## z = z * ((25 * x) + 1)
    // mul y 0   ## Reset y
    // add y w   ## ..
    // add y 2   .# ..
    // mul y x   ## ..
    // add z y   ## z = z + (x * (w + 2))
    let x = if input != (tune[1] + (z % 26)) { 1 } else { 0 };
    z /= tune[0];
    z *= (25 * x) + 1;
    z += x * (input + tune[2]);
    z
}

fn rec(result: &mut i64, z: i64, idx: usize, range: &[i64; 9]) -> bool {
    for n in range {
        let newz = algorithm(z, &TUNINGS[idx], *n);
        if idx < 13 {
            if rec(result, newz, idx + 1, range) {
                *result += 10i64.pow(13 - idx as u32) * n;
                return true;
            }
        } else if newz == 0 {
            *result = *n;
            return true;
        }
    }
    false
}

fn x_est(result: &mut i64, range: [i64; 9]) {
    rec(result, 0, 0, &range);
    println!();
}

fn x_est_multithreaded<'a>(result: &mut i64, range: [i64; 9]) {
    assert_eq!(range[0], 1);
    for n in range {
        let mut thrds = Vec::new();
        let results = Arc::new(Mutex::new([0; 9]));
        for m in range {
            let results = Arc::clone(&results);
            /*
             * This can be further optimized by killing all the threads with
             * higher m once something is found
             */
            thrds.push(thread::spawn(move || {
                let mut r = 0;
                if rec(
                    &mut r,
                    algorithm(algorithm(0, &TUNINGS[0], n), &TUNINGS[1], m),
                    2,
                    &range,
                ) {
                    results.lock().unwrap()[m as usize - 1] = r;
                }
            }));
        }
        for thr in thrds {
            let _ = thr.join();
        }
        if results.lock().unwrap().iter().filter(|&x| *x > 0).count() > 0 {
            let (m, r) = results
                .lock()
                .unwrap()
                .into_iter()
                .enumerate()
                .filter(|(_, x)| *x > 0)
                .nth(0)
                .unwrap();
            *result = n * 10i64.pow(13) + (1 + m as i64) * 10i64.pow(12) + r;
            return;
        }
    }
}

fn main() {
    let range: [i64; 9] = (1..=9).rev().collect::<Vec<_>>().try_into().unwrap();
    let mut result = 0;
    x_est(&mut result, range);
    println!("1: {:#?}", result);

    let range: [i64; 9] = (1..=9).collect::<Vec<_>>().try_into().unwrap();
    let mut result = 0;
    x_est_multithreaded(&mut result, range);
    println!("2: {:#?}", result);
}