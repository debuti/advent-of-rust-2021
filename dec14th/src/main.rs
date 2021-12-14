use std::collections::HashMap;

fn main() {
    let parts = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut tpl = parts.0.chars().collect::<Vec<char>>();

    let instructions = parts
        .1
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|s| {
            let t = s.split_once(" -> ").unwrap();
            (
                t.0.chars().collect::<Vec<char>>(),
                t.1.chars().next().unwrap(),
            )
        })
        .collect::<Vec<(Vec<char>, char)>>();

    for step in 1..=40 {
        println!("{}",step);
        let mut temp = Vec::new();
        for window in tpl.windows(2) {
            let mut flag = false;
            for inst in &instructions {
                if inst.0 == window {
                    temp.push(inst.0[0]);
                    temp.push(inst.1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                temp.push(window[0]);
            }
        }
        temp.push(*tpl.last().unwrap());
        tpl = temp;
        if step == 10 || step == 40 {
        println!("-> {:?}", max(histogram(&tpl))-min(histogram(&tpl)));}
    }
}

fn histogram<T: Eq + std::hash::Hash + Copy>(v: &Vec<T>) -> HashMap<T, usize> {
    let mut r: HashMap<T, usize> = HashMap::new();
    for x in v {
        *r.entry(*x).or_default() += 1;
    }
    r
}

fn max<T>(h: HashMap<T, usize>) -> usize {
    *h.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(_, v)| v).unwrap()
}
fn min<T>(h: HashMap<T, usize>) -> usize {
    *h.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(_, v)| v).unwrap()
}
