use std::{collections::BTreeSet, io};

/// Stack rearrangement
fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut prioritysum_a = 0;
    let mut prioritysum_b = 0;

    for line in &lines {
        let n = line.len() / 2;

        let left: BTreeSet<char> = line[0..n].chars().collect();
        let right: BTreeSet<char> = line[n..line.len()].chars().collect();

        let item = left.intersection(&right).next().unwrap_or(&' ');

        prioritysum_a += priority(*item);
    }

    eprintln!("Part A: {prioritysum_a}");

    // For Part B we simply need the item contained in 3 consecutive sacks
    for g in lines.chunks_exact(3) {
        let a: BTreeSet<char> = g[0].chars().collect();
        let b: BTreeSet<char> = g[1].chars().collect();
        let c: BTreeSet<char> = g[2].chars().collect();

        let ab: BTreeSet<char> = a.intersection(&b).map(|x| *x).collect();

        let item = ab.intersection(&c).next().unwrap_or(&' ');

        prioritysum_b += priority(*item);
    }

    eprintln!("Part B: {prioritysum_b}");

    Ok(())
}

fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        1 + (item as u32) - ('a' as u32)
    } else if item.is_ascii_uppercase() {
        27 + (item as u32) - ('A' as u32)
    } else {
        0
    }
}
