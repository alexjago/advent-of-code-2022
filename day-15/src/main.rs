use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = parse_input()?;

    // draw_grid(&input);

    println!("Part A: {}", part_a(&input, 2_000_000)); // 2_000_000
    println!("Part B: {}", part_b(&input, [0, 4000000, 0, 4000000]));

    Ok(())
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Manhattan distance
    fn manhattan_dist(&self, other: &Point) -> isize {
        self.x.abs_diff(other.x) as isize + self.y.abs_diff(other.y) as isize
    }
}

fn parse_input() -> Result<HashMap<Point, Point>> {
    let re = Regex::new(r".*x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)")?;
    let mut out = HashMap::new();
    for l in std::io::stdin().lines() {
        let line = l?;
        let (k, v) = re
            .captures(&line)
            .context(anyhow!("Unexpected line in input: {}", &line))?
            .iter()
            .skip(1)
            .flatten()
            .filter_map(|x| x.as_str().parse::<isize>().ok())
            .tuples()
            .map(|(x, y)| Point { x, y })
            .collect_tuple()
            .unwrap();
        out.insert(k, v);
    }
    Ok(out)
}

#[allow(dead_code)]
fn draw_grid(input: &HashMap<Point, Point>) {
    let x_max = input.iter().map(|(a, b)| a.x.max(b.x)).max().unwrap_or(0);
    let x_min = input.iter().map(|(a, b)| a.x.min(b.x)).min().unwrap_or(0);
    let y_max = input.iter().map(|(a, b)| a.y.max(b.y)).max().unwrap_or(0);
    let y_min = input.iter().map(|(a, b)| a.y.min(b.y)).min().unwrap_or(0);

    let sensors = sensor_dist(input);
    let beacons = beacons(input);

    print!("    ");
    for x in x_min..=x_max {
        print!(
            "{}",
            match x.signum() {
                -1 => "-",
                1 => "+",
                _ => " ",
            }
        );
    }
    println!();

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", (x / 100).abs())
    }
    println!();

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", ((x % 100 - x % 10) / 10).abs())
    }
    println!();

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", (x % 10).abs())
    }
    println!();

    for y in y_min..=y_max {
        print!("{y:3} ");
        for x in x_min..=x_max {
            let p = Point { x, y };
            if sensors.contains_key(&p) {
                print!("S");
            } else if beacons.contains(&p) {
                print!("B");
            } else if sensors.iter().any(|(k, d)| k.manhattan_dist(&p) <= *d) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn sensor_dist(input: &HashMap<Point, Point>) -> HashMap<Point, isize> {
    let mut out = HashMap::new();
    for (k, v) in input.iter() {
        out.insert(*k, k.manhattan_dist(v));
    }
    out
}

fn beacons(input: &HashMap<Point, Point>) -> HashSet<Point> {
    input.values().copied().collect()
}

fn part_a(input: &HashMap<Point, Point>, row: isize) -> usize {
    let dists = sensor_dist(input);

    // eprintln!("{dists:?}");

    let coalesced = sweep_row(&dists, row);

    // eprintln!("{coalesced:?}");

    let blocks: usize = coalesced
        .iter()
        .map(|(l, r)| 1 + r.abs_diff(*l))
        // .inspect(|x| eprint!("{x}\t\t"))
        .sum::<usize>();
    // eprintln!();
    let bcns = beacons(input).iter().filter(|p| p.y == row).count();

    blocks - bcns
}

/// Sweep-line algorithm
/// <https://en.wikipedia.org/wiki/Sweep_line_algorithm>
/// We'll take a Y coordinate and calculate the bounding points of exclusion zones
fn sweep_row(dists: &HashMap<Point, isize>, row: isize) -> Vec<(isize, isize)> {
    // for each point, we want to get the left and right bounds (if any) of the excluded part of the row
    let mut exclusions: Vec<(isize, isize)> = dists
        .iter()
        .filter_map(|(p, d)| {
            let r = p.y.abs_diff(row) as isize;
            if r <= *d {
                Some((p.x + r - d, p.x + d - r))
            } else {
                None
            }
        })
        .collect();

    exclusions.sort();

    // eprintln!("{exclusions:?}");

    if exclusions.is_empty() {
        return vec![];
    }

    // now we need to deduplicate the exclusion zones...
    // by constructions our exclusion pairs have (left, right)
    // (i.e. left <= right) and now we have sorted by left so...
    // if b.left < a.right
    //      replace a and b with (a.left.min(b.left), a.right.max(b.right)

    let mut coalesced = vec![];
    coalesced.push(exclusions[0]);

    for (l, r) in exclusions.iter().skip(1) {
        let (p, q) = coalesced.pop().unwrap();

        if *l <= q + 1 {
            coalesced.push((p.min(*l), q.max(*r)))
        } else {
            coalesced.push((p, q));
            coalesced.push((*l, *r));
        }
        // eprintln!("{coalesced:?}");
    }
    coalesced
}

/// Distress beacon somewhere in (0..=4000000, 0..=4000000)
/// Tuning frequency = x*4000000 + y
/// we need to take sensor_dists and the above information
/// to pick the one coordinate in ~16 trillion
/// the sweep-line solution is to extend (a) line by line
/// and then presumably at least one line will have a gap
/// Suppose we take all the exclusion zones and add them in 2D
/// then there should be a hole somewhere in it  
fn part_b(input: &HashMap<Point, Point>, limits: [isize; 4]) -> isize {
    // sweep row...
    let dists = sensor_dist(input);
    let [xmin, xmax, ymin, ymax] = limits;
    for row in ymin..=ymax {
        // if row % (ymax - ymin) / 100 == 0 {
        //     eprint!(".");
        // }
        let coal = sweep_row(&dists, row);
        if coal.len() == 0 {
            // There's... no exclusion zone for this row, so
            // x=0 should be avail
            return row;
        } 
        if coal.len() > 0 {
            // Test for edge cases: is the 
            // leftmost or rightmost coordinate in-bounds?
            let l = coal.first().unwrap().0;
            if l > xmin {
                return (l - 1) * 4_000_000 + row;
            }
            let r = coal.last().unwrap().1;
            if r < xmax {
                return (r + 1) * 4_000_000 + row;
            }
        }
        if coal.len() > 1 {
            // Test for gaps between exclusion zones
            for w in coal[..].windows(2) {
                // Iterate over 2-windows for convenience really
                if let &[left, right] = w {
                    if left.1 >= xmin && left.1 < xmax && right.0 > left.1 + 1 {
                        // There's a gap between our two ranges at `left.1 + 1`
                        // eprintln!("({}, {})", left.1 + 1, row);
                        return (left.1 + 1) * 4_000_000 + row;
                    }
                }
            }
        }
    }
    -1
}
