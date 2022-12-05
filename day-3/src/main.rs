use std::{
    collections::BTreeSet,
    io,
};

/// Stack rearrangement
fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut prioritysum = 0;

    for line in lines {
        let n = line.len() / 2;

        let mut left: BTreeSet<char> = BTreeSet::new();
        let mut right: BTreeSet<char> = BTreeSet::new();

        for x in line[0..n].chars() {
            left.insert(x);
        }
        for x in line[n..line.len()].chars() {
            right.insert(x);
        }

        let item = left.intersection(&right).next().unwrap_or(&' ');

        prioritysum += priority(*item);
    }

    eprintln!("Part A: {prioritysum}");

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
