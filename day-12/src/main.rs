use anyhow::{bail, Result};
use std::collections::{HashMap, VecDeque};

fn main() -> Result<()> {
    let grid = make_grid()?;

    println!("Part A: {}", part_a(&grid)?);

    println!("Part B: {}", part_b(&grid)?);

    Ok(())
}

type Row = usize;
type Col = usize;
type Distance = usize;

struct Grid {
    grid: HashMap<(Row, Col), u32>,
    start: (Row, Col),
    end: (Row, Col),
}

fn make_grid() -> Result<Grid> {
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, l) in std::io::stdin().lines().enumerate() {
        let line = l?;
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    grid.insert((i, j), 0);
                    start = (i, j);
                }
                'E' => {
                    grid.insert((i, j), 'z' as u32 - 'a' as u32);
                    end = (i, j);
                }
                _ => {
                    grid.insert((i, j), c as u32 - 'a' as u32);
                }
            }
        }
    }
    Ok(Grid { grid, start, end })
}

/// Input: area heightmap (lowest is `a`; highest `z`)
/// POIs are `S`: start location and `E`: end location
/// Want to get there in fewest steps
/// Can move from `src` to `dst` if `src + 1 <= dst`
/// This all sounds like Breadth-First-Search to me
fn part_a(input: &Grid) -> Result<usize> {
    // Grid should have a width and height
    // Index by row, column (top left is (0,0))

    // Breadth First Search: we have a queue
    let mut queue: VecDeque<(Row, Col)> = VecDeque::new();
    let mut done: HashMap<(Row, Col), Distance> = HashMap::new();
    let mut parents: HashMap<(Row, Col), (Row, Col)> = HashMap::new();
    queue.push_back(input.start);

    while let Some(spot) = queue.pop_front() {
        let now = input.grid.get(&spot).unwrap();
        // eprintln!("{spot:?}: {now}");
        // get neighbours
        // can ONLY make rook-moves
        for r in spot.0.saturating_sub(1)..=spot.0.saturating_add(1) {
            for c in spot.1.saturating_sub(1)..=spot.1.saturating_add(1) {
                // skip self
                if r == spot.0 && c == spot.1 {
                    continue;
                }
                // skip diagonals
                if r != spot.0 && c != spot.1 {
                    continue;
                }
                // ensure neighbour exists and is not already visited
                if let Some(other) = input.grid.get(&(r, c)) {
                    if !done.contains_key(&(r, c)) && !queue.contains(&(r, c)) && *other <= now + 1
                    {
                        // add to queue and traceback
                        parents.insert((r, c), spot);
                        queue.push_back((r, c));
                        // eprintln!("\tqueued ({r}, {c}) [{other}]");
                    } else {
                        // eprintln!("\tskipped ({r}, {c}) [{other}]");
                    }
                }
            }
        }

        // eprintln!("{parents:?}");

        done.insert(
            spot,
            *parents.get(&spot).and_then(|p| done.get(p)).unwrap_or(&0) + 1,
        );

        if spot == input.end {
            // let mut p = spot;
            // while let Some(n) = parents.get(&p) {
            //     eprint!("{p:?} <- ");
            //     p = *n;
            // }
            // eprintln!("{:?}", input.start);

            return Ok(*done.get(&spot).unwrap() - 1);
        }
    }

    bail!("No path found");
}

/// Now we want to find the shortest path from ANY 0-height cell
fn part_b(input: &Grid) -> Result<usize> {
    let zeroes: Vec<(Row, Col)> = input
        .grid
        .iter()
        .filter(|(_k, v)| v == &&0)
        .map(|(k, _v)| *k)
        .collect();

    let mut m = usize::MAX;
    for z in zeroes {
        let g = Grid {
            grid: input.grid.clone(),
            start: z,
            end: input.end,
        };
        let t = part_a(&g).unwrap_or(usize::MAX);
        // eprintln!("{z:?}: {t}");
        m = m.min(t);
    }

    Ok(m)
}
