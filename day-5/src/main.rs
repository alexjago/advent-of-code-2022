use std::{
    collections::{BTreeMap, VecDeque},
    io,
};

/// Stack rearrangement
fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lines().filter_map(|x| x.ok()).collect();
    // step 1: garner input
    // three line typologies:
    // * stack drawing
    // * stack labels
    // [blank line]
    // * move instructions

    let mut stacks_a: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();
    let mut stacks_b: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();

    for line in lines {
        // A somewhat fragile parse but it will do
        if line.contains('[') {
            for (i, v) in line.chars().enumerate() {
                if i % 4 == 1 && !v.is_whitespace() {
                    stacks_a
                        .entry(i / 4 + 1)
                        .or_default()
                        .push_front(String::from(v));
                    stacks_b
                        .entry(i / 4 + 1)
                        .or_default()
                        .push_front(String::from(v));
                }
            }
        } else if line.contains("move") {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            let qty: usize = tokens[1].parse().unwrap();
            let orig: usize = tokens[3].parse().unwrap();
            let dest: usize = tokens[5].parse().unwrap();

            // Part A
            // recursively reorganise
            for _ in 0..qty {
                let v = stacks_a.get_mut(&orig).unwrap().pop_back().unwrap();
                stacks_a.get_mut(&dest).unwrap().push_back(v);
            }

            // Part B
            let from = stacks_b.get_mut(&orig).unwrap();
            let mut moved = from.split_off(from.len() - qty);
            stacks_b.get_mut(&dest).unwrap().append(&mut moved);
        }
    }

    // need top (last) from each
    eprint!("Part A: ");
    for (_, v) in stacks_a {
        eprint!("{}", v.back().unwrap_or(&String::from(" ")));
    }
    eprintln!();

    eprint!("Part B: ");
    for (_, v) in stacks_b {
        eprint!("{}", v.back().unwrap_or(&String::from(" ")));
    }
    eprintln!();

    Ok(())
}
