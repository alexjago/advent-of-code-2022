/// 2022 Advent of Code Day 6
use std::collections::{BTreeMap, BTreeSet};
use std::io::{stdin, Read};

fn main() -> std::io::Result<()> {
    let input = {
        let mut i = String::new();
        stdin().read_to_string(&mut i)?;
        i.chars().collect::<Vec<char>>()
    };

    println!("Part A: {}", find_marker(&input, 4));
    println!("Part B: {}", find_marker(&input, 14));

    Ok(())
}

/// Both markers are a sequence of `n` distinct characters
/// For the start-of-packet marker it's where the four most
/// recently received characters were *all* different
/// So we need a sliding last-4-chars window
/// Example: start of packet is at position 7
///     mjqjpqmgbljsphdztnvjfqwrcgsmlb
/// Similarly, the start-of-message marker is window size 14
fn find_marker_simple(input: &[char], window_size: usize) -> usize {
    for (i, c) in input.windows(window_size).enumerate() {
        // test that all `window_size` elements are different
        let t = BTreeSet::from_iter(c.iter());
        if t.len() == window_size {
            return i + window_size;
        }
    }
    return 0;
}

/// Both markers are a sequence of `n` distinct characters
/// For the start-of-packet marker it's where the four most
/// recently received characters were *all* different
/// So we need a sliding last-4-chars window
/// Example: start of packet is at position 7
///     mjqjpqmgbljsphdztnvjfqwrcgsmlb
/// Similarly, the start-of-message marker is window size 14
fn find_marker(input: &[char], window_size: usize) -> usize {
    // don't recreate a btree every iteration
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();

    // Fill map with initial window (minus one)
    // the final char is added as part of the main loop
    for c in &input[..window_size - 1] {
        *counts.entry(*c).or_insert(0) += 1;
    }

    for (i, c) in input.windows(window_size).enumerate() {
        let oldest = c[0];
        let newest = *c.last().unwrap();

        // update newest
        *counts.entry(newest).or_insert(0) += 1;

        // test for condition
        if (counts.len() == window_size) && counts.values().all(|x| x == &1) {
            return i + window_size;
        }

        // decrement oldest and remove if needed
        if let Some(n) = counts.get(&oldest) {
            if n <= &1 {
                counts.remove(&oldest);
            } else {
                counts.insert(oldest, n - 1);
            }
        }
    }

    return 0;
}
