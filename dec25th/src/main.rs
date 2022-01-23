fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|l| l.len() > 0)
        .collect::<Vec<_>>();
    let (h, w) = (lines.len(), lines.iter().nth(0).unwrap().len());
    let cucumbers = lines
        .iter()
        .enumerate()
        .map(|(ridx, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '>' || c == 'v')
                .map(|(cidx, c)| ((ridx, cidx), c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let mut rights = cucumbers
        .iter()
        .filter(|c| c.1 == '>')
        .map(|c| c.0)
        .collect::<Vec<_>>();
    let mut downs = cucumbers
        .iter()
        .filter(|c| c.1 == 'v')
        .map(|c| c.0)
        .collect::<Vec<_>>();

    let mut count = 0;
    loop {
        let mut changes = false;
        let mut right_idxs = Vec::new();
        let mut down_idxs = Vec::new();
        for (i, c) in rights.iter().enumerate() {
            let next = (c.0, (c.1 + 1) % w);
            if !(rights.contains(&next) || downs.contains(&next)) {
                right_idxs.push((i, next));
                changes = true;
            }
        }
        for (i, next) in right_idxs {
            rights[i] = next;
        }
        
        for (i, c) in downs.iter().enumerate() {
            let next = ((c.0 + 1) % h, c.1);
            if !(rights.contains(&next) || downs.contains(&next)) {
                down_idxs.push((i, next));
                changes = true;
            }
        }
        for (i, next) in down_idxs {
            downs[i] = next;
        }

        count += 1;
        if !changes {
            break;
        } 
    }
    println!("1: {}", count);
}