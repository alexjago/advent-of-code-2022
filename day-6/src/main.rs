use std::collections::BTreeSet;

/// 2022 Advent of Code Day 6

use std::io::Read;

fn main() -> std::io::Result<()> {
    
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

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
fn find_marker(input: &String, window_size: usize) -> usize {
    for (i, c) in input.chars().collect::<Vec<char>>().windows(window_size).enumerate() {
        // test that all `window_size` elements are different
        let t = BTreeSet::from_iter(c.iter());
        if t.len() == window_size {
            return i + window_size;
        }
    }
    return 0;
}
