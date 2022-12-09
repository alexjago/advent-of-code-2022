//! AOC Day 9: Rope Bridge
//!
//!

use std::collections::HashSet;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let moves = read_input()?;

    // 6243
    println!("Part A: {}", part_a(&moves));
    println!("Part A with B: {}", part_b(&moves, 2));
    println!("Part B: {}", part_b(&moves, 10));

    Ok(())
}

/// Convention: Right is +X, Up is +Y
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

/// Straight from the docs
impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Straight from the docs
impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Move {
    axis: Axis,
    dist: isize,
}

fn read_input() -> Result<Vec<Move>> {
    let mut moves = Vec::new();
    for s in std::io::stdin().lines() {
        let line = s?;
        let (d, n) = line.split_once(' ').unwrap();
        let k: isize = n.parse().unwrap();
        moves.push(match d {
            "U" => Move {
                axis: Axis::Y,
                dist: k,
            },
            "D" => Move {
                axis: Axis::Y,
                dist: -k,
            },
            "L" => Move {
                axis: Axis::X,
                dist: -k,
            },
            "R" => Move {
                axis: Axis::X,
                dist: k,
            },
            _ => {
                bail!("Unknown move type");
            }
        })
    }
    Ok(moves)
}

/// The head and tail must always be a kings' move apart or overlapping
/// If the head is ever two steps away NESW the tail can move one step to keep up
/// If the head is ever a knight's move away, the tail moves one step diagonally
fn part_a(moves: &Vec<Move>) -> usize {
    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut tail = Position { x: 0, y: 0 };
    let mut head = Position { x: 0, y: 0 };

    tail_positions.insert(tail);

    for m in moves {
        for _ in 0..m.dist.abs() {
            // eprintln!("{:?} {}", m.axis, m.dist.signum());
            match m.axis {
                Axis::X => {
                    head.x += m.dist.signum();
                }
                Axis::Y => {
                    head.y += m.dist.signum();
                }
            };
            tail = catch_up(&head, &tail).unwrap();
            tail_positions.insert(tail);
            // eprintln!("\thead: {:?}\ttail: {:?}", head, tail);
        }
    }
    // eprintln!("{:?}", tail_positions);
    // visualise_positions(&tail_positions);
    tail_positions.len()
}

/// Implement the tail chase
/// Expectation: `head` and `tail` will never be more than two steps away in any direction
/// In Part A the head will never move diagonally, but in B it can
fn catch_up(head: &Position, tail: &Position) -> Result<Position> {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    let mut x_new = tail.x;
    let mut y_new = tail.y;

    if x_diff.abs() > 1 && y_diff.abs() > 1 {
        // head moved diagonally away from tail
        x_new = tail.x + x_diff.signum();
        y_new = tail.y + y_diff.signum();
    } else if x_diff.abs() > 1 {
        // 2 left/right and either same column or 1 up/down
        // Either way we take the same Y to move diagonally if need be
        x_new = tail.x + x_diff.signum();
        y_new = head.y 
    } else if y_diff.abs() > 1 {
        // transpose of other case
        x_new = head.x;
        y_new = tail.y + y_diff.signum();
    } 
    if x_diff.abs() > 2 || y_diff.abs() > 2 {
        bail!("Error: head too far ahead of tail:\tHead: {:?}\tTail: {:?}", head, tail);
    }
    Ok(Position{x: x_new, y: y_new})
}

/// Rather than two knots, you now must simulate a rope consisting of ten knots. 
/// One knot is still the head of the rope and moves according to the series of motions. 
/// Each knot further down the rope follows the knot in front of it using the same rules as before.
// 2594: too low; 2622: also too low; 2649: too high
fn part_b(moves: &Vec<Move>, knot_count: usize) -> usize {
    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut knots = vec![Position {x: 0, y: 0}; knot_count.max(2)];

    tail_positions.insert(knots[knots.len()-1]);

    for (i, m) in moves.iter().enumerate() {
        for _ in 0..m.dist.abs() {
            // eprintln!("{:?} {}", m.axis, m.dist.signum());
            match m.axis {
                Axis::X => {
                    knots[0].x += m.dist.signum();
                }
                Axis::Y => {
                    knots[0].y += m.dist.signum();
                }
            };
            // Iterate over remaining knots to propagate change
            for k in 1..knots.len() {
                let old = knots[k].clone();
                let rez = catch_up(&knots[k-1], &knots[k]);
                if let Ok(new) = rez {
                    knots[k] = new;
                    if knots[k] == old {
                        // this knot didn't move, so the others won't have to either
                        break;
                    }
                } else {
                    eprintln!("Input: {i}\tMove: {m:?}\t Old: {old:?}, Knot: {k}");
                    for knot in &knots {
                        eprint!("{knot} ");
                    }
                    eprintln!("");
                    visualise_positions(&tail_positions);
                    rez.unwrap();
                }
            }
            tail_positions.insert(knots[knots.len()-1]);
        }
        // eprintln!("Input: {i}; Knots:");
        // for knot in &knots {
        //     eprint!("{knot} ");
        // }
        // eprintln!("");
    }
    // visualise_positions(&tail_positions);
    tail_positions.len()
}

fn visualise_positions(grid: &HashSet<Position>){
    let x_max = grid.iter().map(|p| p.x).max().unwrap_or_default();
    let x_min = grid.iter().map(|p| p.x).min().unwrap_or_default();
    let y_max = grid.iter().map(|p| p.y).max().unwrap_or_default();
    let y_min = grid.iter().map(|p| p.y).min().unwrap_or_default();

    eprintln!("Part B; x: {x_min} to {x_max}; y: {y_min} to {y_max}");

    for yd in 0..=(y_max - y_min) {
        let y = y_max - yd;
        for x in x_min..=x_max {
            if x == 0 && y == 0 {
                eprint!("s");
            } else if grid.contains(&Position {x: x, y: y}) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!("");
    }
}