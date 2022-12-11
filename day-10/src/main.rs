//! AOC Day 10: basic CPU simulation

use std::{collections::HashSet, io::Read};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    };

    println!("Part A: {}", part_a(&input));

    Ok(())
}

/// Trace the value of X "during" the cycle
/// Two instructions, one register
///     addx p
///     noop
/// `addx p` adds p (integer, which may be negative) to register X;
/// it takes *two* cycles to complete: X does not change until one cycle later
/// (this doesn't mean data race hell, it means it takes two cycles)
/// X changes "after" the cycle rather than "during"
/// `noop` does nothing and takes one cycle to complete
fn trace_x_during(input: &str, startval: isize) -> Vec<isize> {
    let mut x = startval;
    let mut out = Vec::new();
    for line in input.lines() {
        // X traced "during" while addition happens "after"
        out.push(x);
        if line.starts_with("addx") {
            // simulate two cycles with extra push for first one before X changed
            out.push(x);
            x += line.split_once(' ').unwrap().1.parse::<isize>().unwrap();
        } // else noop
    }
    out
}

/// For part A we're told that X starts with the value 1
/// and asked to get the value of it at cycles 20, 60, 100, 140, 180 and 220,
/// multiplied by those values and summed
fn part_a(input: &str) -> isize {
    trace_x_during(input, 1)
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .take(220)
        .skip(19)
        .step_by(40)
        // .inspect(|t| eprintln!("{t:?}"))
        .map(|(i, x)| *x * (i as isize))
        // .inspect(|t| eprintln!("{t:?}"))
        .sum()
}
