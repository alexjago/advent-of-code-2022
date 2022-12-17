use std::{collections::BTreeMap, fmt::Display};

use anyhow::Result;
use regex::Regex;

static mut RECURSE_COUNT: usize = 0_usize;
static mut CACHE_HITS: usize = 0_usize;
const MAX_DEPTH: usize = 30;

fn main() -> Result<()> {
    let input = parse_input()?;

    print_dot(&input);

    let order = make_order(&input);
    let pair_dists = calc_pairs(&input, &order, "AA".into());

    println!("\n\nstrict graph {{");
    pair_dists
        .iter()
        .filter(|(_, d)| *d != &0_usize)
        .for_each(|((f, t), d)| println!("\t{f} -- {t} [label={d}]"));
    input
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .for_each(|(_, v)| println!("\t{} [label=\"{} {}\"]", v.name, v.name, v.rate));
    println!("}}\n\n");

    let (rez, seq) = part_a(&input, MAX_DEPTH);
    println!("\nPart A: {}", rez);
    seq.iter().for_each(|(s, t, h)| eprintln!("{s} {h} @{t}"));
    // 3941 too high; 1666 too low
    unsafe {
        println!(
            "recursed {RECURSE_COUNT} times with {CACHE_HITS} cache hits; max depth of {MAX_DEPTH}"
        )
    };
    // println!("Part B: {}", part_b());

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Position {
    // 26x26 > 2^8
    id: u16,
}

impl From<&str> for Position {
    fn from(string: &str) -> Position {
        let mut o: u32 = 0;
        let mut ccc = string.chars();
        o += (ccc.next().unwrap() as u32 - 'A' as u32) * 26;
        o += ccc.next().unwrap() as u32 - 'A' as u32;
        Position { id: o as u16 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = char::from_u32((self.id / 26) as u32 + ('A' as u32)).unwrap();
        let o = char::from_u32((self.id % 26) as u32 + ('A' as u32)).unwrap();
        write!(f, "{t}{o}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Valve {
    name: Position,
    rate: usize,
    tunnels: Vec<Position>,
}

fn parse_input() -> Result<BTreeMap<Position, Valve>> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let re = Regex::new(r"Valve (\w+) has flow rate=(-?\d+); tunnels? leads? to valves? (.*)")?;
    let mut out = BTreeMap::new();
    for l in std::io::stdin().lines() {
        let line = l?;
        // eprintln!("{line}");
        let matches = re.captures(&line).unwrap();

        let namestr = matches.get(1).unwrap().as_str();
        let name: Position = namestr.into();
        let rate = matches.get(2).unwrap().as_str().parse()?;
        let tunnels: Vec<Position> = matches
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(Position::from)
            .collect();

        out.insert(
            name,
            Valve {
                name,
                rate,
                tunnels,
            },
        );
    }

    Ok(out)
}

/// Output graph structure in
fn print_dot(input: &BTreeMap<Position, Valve>) {
    println!("strict graph {{");
    for (k, v) in input {
        if v.rate > 0 {
            println!("\t{k}[label=\"{} {}\"]", k, v.rate);
        }
        print!("\t{k} -- {{");
        for i in &v.tunnels {
            print!("{i} ");
        }
        println!("}}");
    }
    println!("}}")
}

fn make_order(input: &BTreeMap<Position, Valve>) -> BTreeMap<Position, usize> {
    let mut order: BTreeMap<Position, usize> = BTreeMap::new();
    for (i, k) in input.keys().enumerate() {
        order.insert(*k, i);
    }

    order
}

/// Need to maximise total "flow rate"
/// Starting at valve AA, we can move to connected valves in 1 min
/// and switch them on in another minute
/// they score at flow rate * minutes remaining
/// total score is sum of that
fn part_a(
    input: &BTreeMap<Position, Valve>,
    time: usize,
) -> (usize, Vec<(usize, usize, Position)>) {
    let mut memo = Memo::new();
    let position = "AA".into();
    let order = make_order(input);
    let dists = calc_pairs(input, &order, position);
    let activated = 0;
    solve_recurse(
        input, &order, &dists, activated, position, position, time, &mut memo,
    )
}

// Let's try memoizing?
// HashMap<({valves on}, current position, time left), score>)

type Memo = BTreeMap<(Activated, Position, usize), usize>;

fn solve_recurse(
    input: &BTreeMap<Position, Valve>,
    order: &BTreeMap<Position, usize>,
    dists: &BTreeMap<(Position, Position), usize>,
    activated: Activated,
    position: Position,
    start: Position,
    time: usize,
    _memo: &mut Memo,
) -> (usize, Vec<(usize, usize, Position)>) {
    if time == 0 {
        // we've run out and can score no more
        return (0, vec![(0, 0, position)]);
    }
    /*
        // Memoization results
        if memo.contains_key(&(activated, position, time)) {
            unsafe { CACHE_HITS += 1 }
            return *memo.get(&(activated, position, time)).unwrap();
        }
    */
    let mut max_score_yes = 0;
    let mut winner = Position { id: u16::MAX };
    let mut history = vec![];
    if !is_active(order, position, activated) {
        // ^^ if the valve is already active then we're just wasting a minute re-activating it
        // but we also have to keep in mind that our start node isn't activable
        let tap_score = input.get(&position).unwrap().rate * (time - 1);
        // ^^ always choose to open it (by graph embedding)
        max_score_yes = tap_score;
        let new_viz = activate(order, position, activated);
        let fudge = if position == start { 0 } else { 1 };
        // we might not have time to visit another room and turn it on
        // but we do at least have a minute to turn this one on
        if time == 1 || time == 2 {
            //     memo.insert((new_viz, position, time - 1), max_score_yes);
        } else {
            // OK, we do have time
            let options = dists.iter().filter_map(|((f, t), d)| {
                if *f == position && *t != position {
                    Some((*t, d))
                } else {
                    None
                }
            });

            for o in options {
                if time > *o.1 + fudge && !is_active(order, o.0, new_viz) {
                    let (rez, seq) = solve_recurse(
                        input,
                        order,
                        dists,
                        new_viz,
                        o.0,
                        start,
                        time - (o.1 + fudge), // +1 because we always activate
                        _memo,
                    );
                    if rez + tap_score > max_score_yes {
                        max_score_yes = rez + tap_score;
                        winner = o.0;
                        history = seq;
                    }
                }
            }
        }
    }

    // memo.insert((new_viz, position, time - (1 + o.1)), rez + tap_score);
    let s = max_score_yes;
    history.push((s, time, winner));
    // eprint!("{time}:{s}:{winner}  ");
    // SAFETY: single threaded
    unsafe {
        RECURSE_COUNT += 1;
        if RECURSE_COUNT % 1000000 == 0 {
            eprint!(".");
        }
    };
    (s, history)
}

// MASSIVE hack: I happen to know there are less than 64 rooms/valves
type Activated = u64;

/// Mark a room/valve as activated
fn activate(
    order: &BTreeMap<Position, usize>,
    position: Position,
    existing: Activated,
) -> Activated {
    let idx = order.get(&position).unwrap();
    existing | 1 << idx
}

fn is_active(order: &BTreeMap<Position, usize>, position: Position, existing: Activated) -> bool {
    let idx = order.get(&position).unwrap();
    (existing & 1 << idx) != 0
}

#[allow(dead_code)]
fn part_b() -> usize {
    todo!()
}

/// Calculate shortest distances between pairs of
/// Filter to nonzero-rate valves only, and also `start`
fn calc_pairs(
    input: &BTreeMap<Position, Valve>,
    _order: &BTreeMap<Position, usize>,
    start: Position,
) -> BTreeMap<(Position, Position), usize> {
    let mut tree: BTreeMap<(Position, Position), usize> = BTreeMap::new();

    // for each immediate edge, initialise
    for (ink, inv) in input.iter() {
        for outk in inv.tunnels.iter() {
            tree.insert((*ink, *outk), 1);
        }
        // and set self-distance to 0
        tree.insert((*ink, *ink), 0);
    }

    for k in input.keys() {
        for i in input.keys() {
            for j in input.keys() {
                let cur = tree.get(&(*i, *j)).unwrap_or(&usize::MAX);
                let pot = tree
                    .get(&(*i, *k))
                    .unwrap_or(&usize::MAX)
                    .saturating_add(*tree.get(&(*k, *j)).unwrap_or(&usize::MAX));
                if cur > &pot {
                    tree.insert((*i, *j), pot);
                }
            }
        }
    }

    BTreeMap::from_iter(
        tree.iter()
            .filter(|((kf, kt), _d)| {
                let tv = input.get(kt).unwrap();
                let fv = input.get(kf).unwrap();
                (*kt == start || tv.rate > 0) && (*kf == start || fv.rate > 0)
            })
            .map(|(p, d)| (*p, *d)),
    )
}

/*
    OK so we're definitely doing loops and backtracking, the sample has it
        AA
        DD + (i.e. turned on here)
        CC
        BB +
        AA
        II
        JJ +
        II
        AA
        DD
        EE
        FF
        GG
        HH +
        GG
        FF
        EE
        DD
        CC +

*/
