use std::collections::HashMap;

fn main() {
    let parts = include_str!("input.txt").split_once("\n\n").unwrap();

    let tpl = parts.0.chars().collect::<Vec<char>>();
    let mut items: HashMap<[char; 2], isize> = HashMap::new();
    for ann in tpl
        .windows(2)
        .map(|t| [t[0], t[1]])
        .collect::<Vec<[char; 2]>>()
    {
        if let Some(i) = items.get_mut(&ann) {
            *i += 1;
        } else {
            items.insert(ann, 1);
        }
    }

    let instructions = parts
        .1
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|s| {
            let t = s.split_once(" -> ").unwrap();
            (
                [
                    t.0.chars().collect::<Vec<char>>()[0],
                    t.0.chars().collect::<Vec<char>>()[1],
                ],
                t.1.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<[char; 2], char>>();

    for step in 1..=40 {
        let mut tmp: Vec<([char; 2], isize)> = Vec::new();

        for (k, v) in items.iter() {
            tmp.push((*k, *v));
            if let Some(inst) = instructions.get(k) {
                tmp.push((*k, -*v));
                let lk = &[k[0], *inst];
                tmp.push((*lk, *v));
                let rk = &[*inst, k[1]];
                tmp.push((*rk, *v));
            }
        }
        
        items.clear();
        for ann in tmp {
            if let Some(i) = items.get_mut(&ann.0) {
                *i += ann.1;
            } else {
                items.insert(ann.0, ann.1);
            }
        }
        
        if step == 10 || step == 40 {
            let mut summary: HashMap<char, isize> = HashMap::new();
            summary.insert(*tpl.iter().last().unwrap(), 1);
            for tuple in &items {
                if let Some(x) = summary.get_mut(&tuple.0[0]) {
                    *x += tuple.1;
                } else {
                    summary.insert(tuple.0[0], *tuple.1);
                }
            }
            let max = summary.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
            let min = summary.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
            println!("-> {}", max.1 - min.1);
        }
    }
}
