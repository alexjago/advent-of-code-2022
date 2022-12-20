//! Day 17: Almost tetris!
use anyhow::Result;
use std::collections::HashMap;

const CAVE_WIDTH: u8 = 7_u8;
const SPAWNS_A: usize = 2022;
const SPAWNS_B: usize = 1000000000000;
const WINDOW_DEPTH: usize = (CAVE_WIDTH as usize) * 4;

fn main() -> Result<()> {
    let input = read_input()?;

    /*
        // testing that the rocks are laid out correctly
        use Rock::*;
        for r in [Minus, Plus, Angle, Bar, Square] {
            println!("{}", print_pile(&r.to_u8s()));
        }
    */
    println!("Part A: {}", part_a(&input, SPAWNS_A));
    println!("Part B: {}", part_b(&input, SPAWNS_B));

    Ok(())
}

/// There are five rock patterns:
/// * 4-wide bar ####
/// * plus sign (3 rows, 3 cols, corners missing)
/// * left angle (3 high and wide)
/// * 4-high bar
/// * 2x2 square
/// They also come in this order
/// Format: we know the vertical chamber is exactly 7 units wide
/// we can comfortably fit each of these into a u16 bitmap
/// (top left == high bit, bottom right = low bit)
/// the bottom left is the reference corner
#[repr(u16)]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Rock {
    Minus = 0x000F_u16,
    Plus = 0x04E4_u16,
    Angle = 0x022E_u16,
    Bar = 0x8888_u16,
    Square = 0x00CC_u16,
}

impl Rock {
    /// Convert to an array of u8s
    /// high bit of [0] is bottom left
    fn to_u8s(self: &Self) -> [u8; 4] {
        let rock = *self as u16;
        // bottom layer
        let a = ((rock & 0x000f) << 4) as u8;
        // second-bottom layer
        let b = ((rock & 0x00f0) >> 0) as u8;
        let c = ((rock & 0x0f00) >> 4) as u8;
        let d = ((rock & 0xf000) >> 8) as u8;

        [a, b, c, d]
    }

    fn width(self: &Self) -> u8 {
        use crate::Rock::*;
        match &self {
            Bar => 1,
            Square => 2,
            Angle | Plus => 3,
            Minus => 4,
        }
    }
}

/// The rock automatically falls each tick.
/// It will also have a gust of air pushing it left/right
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[repr(i8)]
enum Move {
    Left = -1,
    Null = 0,
    Right = 1,
}
/// Collisions with the Floor cause this rock to come to rest
/// and instantly spawn a new rock
/// Collisions with anything else simply prevent the L/R movement
enum Collision {
    Floor,
    Other,
}

fn read_input() -> Result<Vec<Move>> {
    let buf = std::io::read_to_string(std::io::stdin())?;

    use crate::Move::*;
    Ok(buf
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Left),
            '>' => Some(Right),
            _ => None,
        })
        .collect())
}

/// General structure: nested loop over spawns and then single rock
fn part_a(input: &[Move], spawns: usize) -> usize {
    use Rock::*;
    let mut moves = input.iter().cycle();
    // stopped rocks.
    let mut pile: Vec<u8> = Vec::new();

    // For each rock...
    for rock in [Minus, Plus, Angle, Bar, Square]
        .iter()
        .cycle()
        .take(spawns)
    {
        let mut bottom = pile.len() + 3;
        let mut left = 2;

        // ... For each move
        while let Some(m) = moves.next() {
            //            eprintln!(
            //                "{:?} at (row {} col {}), moving {:?}",
            //                rock, bottom, left, m
            //            );
            // Attempt to push the thing left/right
            if !would_collide(*m, *rock, bottom, left, &pile) {
                // if a L/R movement would cause a collision then it just
                // doesn't take effect
                left = (left as i8).wrapping_add(*m as i8) as u8;
            }
            // Attempt to move it down
            if bottom == 0 || would_collide(Move::Null, *rock, bottom - 1, left, &pile) {
                add_to_pile(*rock, bottom, left, &mut pile);
                break;
            } else {
                bottom -= 1;
            }
        }
        // println!("{}", print_pile(&pile));
        //        if let Some(b) = is_blocked(&pile) {
        //            println!("-{}\n{}", b, print_pile(&pile));
        //            break;
        //        }
    }

    //  println!("{}", print_pile(&pile));
    pile.len()
}

fn would_collide(push: Move, rock: Rock, bottom: usize, left: u8, pile: &Vec<u8>) -> bool {
    use Move::*;
    // test for left wall
    if push == Left && left == 0 {
        return true;
    }
    // test for right wall
    if push == Right && left + rock.width() == CAVE_WIDTH {
        return true;
    }

    if bottom > pile.len() {
        return false;
    }

    // the harder collision detection is with the pile
    // the last element of the pile is the topmost row
    // We have to do a convolution, basically

    let testleft = (left as i8).wrapping_add(push as i8) as u8;
    for ((i, layer), r) in pile
        .iter()
        .enumerate()
        .skip(bottom)
        .zip(rock.to_u8s().iter())
        .take(4)
    {
        //        eprintln!("\t{i}: {layer:02x} | {r:02x}");
        if (r >> testleft) & layer != 0 {
            //            eprintln!(
            //                "Intersection on layer {i}:{layer:02x} with {r:02x} of {:?}",
            //                rock
            //            );
            return true;
        }
    }
    false
}

fn add_to_pile(rock: Rock, bottom: usize, left: u8, pile: &mut Vec<u8>) {
    //eprintln!("Adding {rock:?} at ({bottom}, {left})");

    if bottom + 4 > pile.len() {
        pile.extend([0_u8].iter().cycle().take(4 + bottom - pile.len()));
    }

    for (i, r) in rock.to_u8s().iter().enumerate() {
        pile[bottom + i] |= r >> left;
    }

    // Trim pile to maintain invariant that top layer is occupied
    while let Some(t) = pile.pop() {
        if t != 0 {
            pile.push(t);
            break;
        }
    }
}

fn print_pile(pile: &[u8]) -> String {
    let mut out = String::new();

    for (i, r) in pile.iter().enumerate().rev() {
        out.push_str(&format!("{i:6}\t"));
        for k in 0..7 {
            out.push(match r & (0x80 >> k) {
                0 => '.',
                _ => '#',
            });
        }
        out.push('\n');
    }
    out.push_str("\t0123456\n");
    out
}

/// With much reading of hints.
/// The general concept here is that since the inputs are cyclic.
/// If we can ever show that the rocks can't get below a certain level,
/// then that effectively resets the floor.
/// We can hash:
///     (everything above, current rock, current move) : (that level, number of rocks spawned)
/// having done that if we ever get a *repeated* key then we have found the cycle length
/// and from there we can skip cycles and need only play through one more cycle
fn part_b(input: &[Move], total_spawns: usize) -> usize {
    // (hash([remaining pile]), rock, jet_id) : (total_height, rock_id)
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut cycle_cache: HashMap<(u64, Rock, usize), (usize, usize)> = HashMap::new();

    use Rock::*;
    let mut moves = input.iter().enumerate().cycle();
    let rocks = [Minus, Plus, Angle, Bar, Square];
    let mut rock_id = 0;
    // stopped rocks.
    let mut pile: Vec<u8> = Vec::new();

    let mut total_height = 0;
    let mut finish_up = false;
    let mut cycle_count = 0;
    let mut cycle_height = 0;
    let mut pre_cycle_height = 0;

    // For each rock...
    while rock_id < total_spawns {
        let rock = rocks[rock_id % 5];
        let mut bottom = pile.len() + 3;
        let mut left = 2;
        let mut jet_id = 0;
        // ... For each move
        while let Some((jid, m)) = moves.next() {
            //            eprintln!(
            //                "{:?} at (row {} col {}), moving {:?}",
            //                rock, bottom, left, m
            //            );
            // Attempt to push the thing left/right
            if !would_collide(*m, rock, bottom, left, &pile) {
                // if a L/R movement would cause a collision then it just
                // doesn't take effect
                left = (left as i8).wrapping_add(*m as i8) as u8;
            }
            // Attempt to move it down
            if bottom == 0 || would_collide(Move::Null, rock, bottom - 1, left, &pile) {
                add_to_pile(rock, bottom, left, &mut pile);
                jet_id = jid;
                break;
            } else {
                bottom -= 1;
            }
        }
        if !finish_up {
            if let Some(b) = is_blocked(&pile) {
                // trim pile
                let rem = pile.split_off(b);
                total_height += pile.len();
                // add to cache
                let mut hashy = DefaultHasher::new();
                rem[..].hash(&mut hashy);
                let h = hashy.finish();

                if let Some((prev_height, prev_rock_id)) = cycle_cache.get(&(h, rock, jet_id)) {
                    eprintln!(
                        "CYCLE FOUND: previous height {prev_height} at {prev_rock_id} rocks."
                    );
                    eprintln!(
                        "Currently at {}+{} height and {} rocks, last input {:?}",
                        total_height,
                        rem.len(),
                        rock_id,
                        (rock, jet_id)
                    );

                    // Having found a cycle:
                    // (rock_id - prev_rock_id) * X  + prev_rock_id == TOTAL_SPAWNS
                    // X = (TOTAL_SPAWNS - prev_rock_id) / (rock_id - prev_rock_id)
                    // yes, but also integer maths here
                    let cycle_len = rock_id - prev_rock_id;
                    let xxx = (total_spawns - prev_rock_id) / cycle_len;
                    let fudge = (total_spawns - prev_rock_id) % xxx;
                    eprintln!(
                        "Estimated total number of cycles is {xxx} to hit {} rocks",
                        total_spawns - fudge - prev_rock_id
                    );
                    // Then the pile height becomes...
                    eprintln!(
                        "Estimated total height is at least {}",
                        (total_height - prev_height) * xxx + prev_height
                    );

                    cycle_height = total_height - prev_height;
                    cycle_count = xxx;
                    pre_cycle_height = *prev_height;
                    // let's skedaddle
                    finish_up = true;
                    rock_id += (cycle_count - 1) * cycle_len;
                    eprintln!("Skipping ahead to {rock_id}");
                } else {
                    /* eprintln!(
                        "Caching {:?} : {:?}\n\n",
                        (h, rock, jet_id),
                        (total_height, rock_id)
                    ); */
                    cycle_cache.insert((h, rock, jet_id), (total_height, rock_id));
                }
                // trim pile
                pile = rem;
            }
        }
        rock_id += 1;
    } // <-- end of rock loop

    eprintln!("Final pile length: {}", pile.len());
    pile.len() + (cycle_height * cycle_count) + pre_cycle_height // + cycle maths
}

/// Return the level (if any) that rocks cannot fall below
fn is_blocked(pile: &[u8]) -> Option<usize> {
    // simple check for 1 or 2-layer
    for k in 1..=WINDOW_DEPTH {
        for (i, w) in pile.windows(k).rev().enumerate() {
            if w.contains(&0x00) {
                continue;
            }
            if w.iter().map(|x| *x).reduce(|x, a| x | a) == Some(0xFE_u8) {
                //                eprintln!("--------------------------------");
                //                eprintln!("{}", print_pile(w));
                //                eprintln!("\tline {}, window size {k}", pile.len() - (i + k));
                //                eprintln!("{}", print_pile(pile));
                return Some(pile.len() - (i + k));
            }
        }
    }
    None
}
