//! AOC Day 18
use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

fn main() -> Result<()> {
    let (input, bounds) = read_input()?;

    // input.iter().for_each(|c| eprintln!("{:?}", c));
    eprintln!("Bounds: {bounds:?}");

    let total_surfs = part_a(&input, &bounds);

    println!("Part A: {}", total_surfs);

    Ok(())
}
type Point = (usize, usize, usize);
type Bounds = [RangeInclusive<usize>; 3];

fn read_input() -> Result<(BTreeSet<Point>, Bounds)> {
    let input = std::io::stdin().lines();
    let mut out = BTreeSet::new();

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut z_min = 0;
    let mut z_max = 0;

    for l in input {
        let line = l?;

        for p in line.split(',').tuples() {
            let (xs, ys, zs) = p;
            let x = xs.parse()?;
            let y = ys.parse()?;
            let z = zs.parse()?;

            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
            z_min = z_min.min(z);
            z_max = z_max.max(z);

            out.insert((x, y, z));
        }
    }
    Ok((out, [x_min..=x_max, y_min..=y_max, z_min..=z_max]))
}

/// We're approximating surface area by counting non-touching faces of unit cubes
fn part_a(input: &BTreeSet<Point>, bounds: &Bounds) -> usize {
    let mut working: BTreeMap<Point, usize> = BTreeMap::new();

    for p in input {
        working.insert(*p, 6);
        for x in p.0.saturating_sub(1)..=(p.0 + 1) {
            for y in p.1.saturating_sub(1)..=(p.1 + 1) {
                for z in p.2.saturating_sub(2)..=(p.2 + 1) {
                    if input.contains(p)
                        && input.contains(&(x, y, z))
                        && p.0.abs_diff(x) + p.1.abs_diff(y) + p.2.abs_diff(z) == 1
                    {
                        // eprintln!("Neighbours: {:?} and {:?}", p, (x, y, z));
                        working.entry(*p).and_modify(|c| *c -= 1);

                        // working
                        //     .entry((x, y, z))
                        //     .and_modify(|c| *c += 1)
                        //     .or_insert(1);
                    }
                }
            }
        }
    }

    working
        .iter()
        // .inspect(|(p, c)| eprintln!("{:?}: {}", p, c))
        .map(|(_, c)| c)
        .sum()
}

/// ... and now we only want those for which it's possible to reach the outside
/// Step 1: what *is* outside anyway?
///     -> do a flood fill from the edges of the sim box
/// Step 2: similar to part A, but now faces only count if they're paired with an "outside"
fn part_b() {}
