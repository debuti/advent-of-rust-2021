const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Part {
    v: u32,
    d: u32,
}

fn main() {
    let lines = include_str!("input.txt")
        .split("\n")
        .filter(|l| l.len() > 0 && l.chars().next().unwrap() != '#')
        .collect::<Vec<&str>>();

    let numbers = parsein(&lines);
    let mut sum = Vec::<Part>::new();
    for (idx, number) in numbers.into_iter().enumerate() {
        if idx == 0 {
            sum = number;
        } else {
            sum = nadd(sum, number);
        }
    }
    debugln!("\nsum:\t\t{:?}", sum);
    println!("1:\t{:?}", magnitude(sum));

    let numbers = parsein(&lines);
    let mut largest = 0u32;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let m = magnitude(nadd(numbers[i].clone(), numbers[j].clone()));
                if m > largest {
                    largest = m;
                }
            }
            let m = magnitude(nadd(numbers[j].clone(), numbers[i].clone()));
            if m > largest {
                largest = m;
            }
        }
    }
    println!("2:\t{:?}", largest);
}

fn parsein(lines: &Vec<&str>) -> Vec<Vec<Part>> {
    let mut result = Vec::<_>::new();
    for line in lines {
        let mut depth = 0;
        result.push(line.chars().fold(Vec::<Part>::new(), |mut acc, c| {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {}
                d => acc.push(Part {
                    v: d.to_digit(10).unwrap(),
                    d: depth,
                }),
            };
            acc
        }));
    }
    result
}

fn magnitude(mut n: Vec<Part>) -> u32 {
    while n.len() > 1 {
        for idx in 0..n.len() {
            if n[idx].d == n[idx + 1].d {
                n[idx] = Part {
                    v: 3 * n[idx].v + 2 * n[idx + 1].v,
                    d: n[idx].d - 1,
                };
                n.remove(idx + 1);
                break;
            }
        }
    }
    n[0].v
}

fn nadd(mut l: Vec<Part>, mut r: Vec<Part>) -> Vec<Part> {
    l.iter_mut().for_each(|p| p.d += 1);
    r.iter_mut().for_each(|p| p.d += 1);
    l.append(&mut r);
    debugln!("after addition\t{:?}", l);
    reduce(l)
}

fn reduce(mut a: Vec<Part>) -> Vec<Part> {
    'outer: loop {
        for idx in 0..a.len() {
            // Explode
            if a[idx].d > 4 {
                a[idx.saturating_sub(1)].v += a[idx].v;                
                if idx + 1 < a.len() - 1 {
                    a[idx + 2].v += a[idx + 1].v;
                }
                a[idx] = Part {
                    v: 0,
                    d: a[idx].d - 1,
                };
                a.remove(idx + 1);
                debugln!("after explode\t{:?}", a);
                continue 'outer;
            }
        }
        for idx in 0..a.len() {
            // Split
            if a[idx].v > 9 {
                a.insert(
                    idx + 1,
                    Part {
                        v: if a[idx].v % 2 == 0 {
                            a[idx].v >> 1
                        } else {
                            (a[idx].v >> 1) + 1
                        },
                        d: a[idx].d + 1,
                    },
                );
                a[idx] = Part {
                    v: a[idx].v >> 1,
                    d: a[idx].d + 1,
                };
                debugln!("after split\t{:?}", a);
                continue 'outer;
            }
        }
        debugln!();
        break;
    }
    a
}
