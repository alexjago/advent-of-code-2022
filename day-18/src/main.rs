//! AOC Day 18
use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    ops::RangeInclusive,
};

fn main() -> Result<()> {
    let (input, bounds) = read_input()?;

    // input.iter().for_each(|c| eprintln!("{:?}", c));
    eprintln!("Bounds: {bounds:?}");

    let total_surfs = part_a(&input, &bounds);

    println!("Part A: {}", total_surfs);

    println!("Part B: {}", part_b(&input, &bounds));

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
                    if input.contains(&(x, y, z))
                        && p.0.abs_diff(x) + p.1.abs_diff(y) + p.2.abs_diff(z) == 1
                    {
                        // eprintln!("Neighbours: {:?} and {:?}", p, (x, y, z));
                        working.entry(*p).and_modify(|c| *c -= 1);
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Lava,
    Exterior,
    Interior,
    Unknown,
}

/// ... and now we only want those for which it's possible to reach the outside
/// Step 1: what *is* outside anyway?
///     -> do a flood fill from the edges of the sim box
/// Step 2: similar to part A, but now faces only count if they're paired with an "outside"
fn part_b(input: &BTreeSet<Point>, bounds: &Bounds) -> usize {
    let x_min = *bounds[0].start();
    let x_max = *bounds[0].end();
    let y_min = *bounds[1].start();
    let y_max = *bounds[1].end();
    let z_min = *bounds[2].start();
    let z_max = *bounds[2].end();

    let mut grid: Vec<Vec<Vec<Cell>>> =
        vec![
            vec![vec![Cell::Unknown; (1 + z_max - z_min)]; (1 + y_max - y_min)];
            (1 + x_max - x_min)
        ];

    let mut queue: VecDeque<Point> = VecDeque::new();

    // initialise
    for x in bounds[0].clone() {
        for y in bounds[1].clone() {
            for z in bounds[2].clone() {
                let me = if input.contains(&(x, y, z)) {
                    Cell::Lava
                } else if x == x_min
                    || x == x_max
                    || y == y_min
                    || y == y_max
                    || z == z_min
                    || z == z_max
                {
                    Cell::Exterior
                } else if false {
                    todo!()
                } else {
                    Cell::Unknown
                };
                if me == Cell::Exterior {
                    queue.push_back((x, y, z));
                }
                grid[x][y][z] = me;
            }
        }
    }

    // Flood fill to neighbours
    while let Some(me) = queue.pop_front() {
        for x in me.0.saturating_sub(1)..=me.0.saturating_add(1).min(x_max) {
            for y in me.1.saturating_sub(1)..=me.1.saturating_add(1).min(y_max) {
                for z in me.2.saturating_sub(1)..=me.2.saturating_add(1).min(z_max) {
                    if grid[x][y][z] == Cell::Unknown
                        && me.0.abs_diff(x) + me.1.abs_diff(y) + me.2.abs_diff(z) == 1
                    {
                        grid[x][y][z] = Cell::Exterior;
                        queue.push_back((x, y, z));
                    }
                }
            }
        }
    }

    let mut lava_count = 0;
    let mut exterior_count = 0;
    let mut interior_count = 0;
    let mut unknown_count = 0;
    for x in bounds[0].clone() {
        for y in bounds[1].clone() {
            for z in bounds[2].clone() {
                match grid[x][y][z] {
                    Cell::Lava => lava_count += 1,
                    Cell::Exterior => exterior_count += 1,
                    Cell::Interior => interior_count += 1,
                    Cell::Unknown => unknown_count += 1,
                }
            }
        }
    }

    eprintln!("{lava_count} lava; {exterior_count} exterior; {interior_count} interior; {unknown_count} unknown");

    let mut working: BTreeMap<Point, usize> = BTreeMap::new();

    for p in input {
        working.insert(*p, 0);
        for x in p.0.saturating_sub(1)..=(p.0 + 1).min(x_max) {
            for y in p.1.saturating_sub(1)..=(p.1 + 1).min(y_max) {
                for z in p.2.saturating_sub(2)..=(p.2 + 1).min(z_max) {
                    if grid[x][y][z] == Cell::Exterior
                        && p.0.abs_diff(x) + p.1.abs_diff(y) + p.2.abs_diff(z) == 1
                    {
                        working.entry(*p).and_modify(|c| *c += 1);
                    }
                }
            }
        }

        for cond in &[
            p.0 == x_min,
            p.0 == x_max,
            p.1 == y_min,
            p.1 == y_max,
            p.2 == z_min,
            p.2 == z_max,
        ] {
            if *cond {
                eprintln!("External lava at {p:?}");
                working.entry(*p).and_modify(|c| *c += 1);
            }
        }
    }
    working
        .iter()
        // .inspect(|(p, c)| eprintln!("{:?}: {}", p, c))
        .map(|(_, c)| c)
        .sum()
}
