/// Monkey in the Middle
use std::{collections::VecDeque, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let instr = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    };

    println!("Part A: {}", part_a(&instr, 20));

    println!("Part B: {}", part_b(&instr, 10_000));

    Ok(())
}

fn parse_input(input: &str) -> (Vec<Monkey>, isize) {
    let mut out = Vec::new();
    let mut modulo = 1;
    for (k, l) in input.split("\n\n").enumerate() {
        let mut items = VecDeque::new();
        let mut operation: Box<dyn Fn(isize) -> isize> = Box::new(|x| x);
        let mut testmod: isize = 1;
        let mut iftrue: usize = 0;
        let mut iffalse: usize = 0;
        for line in l.lines().skip(1).map(|s| s.trim()) {
            if let Some(starting_items) = line.strip_prefix("Starting items: ") {
                items.extend(
                    starting_items
                        .split(",")
                        .map(|s| s.trim())
                        .filter_map(|s| s.parse::<isize>().ok()),
                );
            } else if let Some(oppy) = line.strip_prefix("Operation: new = old ") {
                let (op, val_s) = oppy.split_once(' ').unwrap();
                // eprintln!("op: {op}\t val_s: {val_s}");
                // TODO: handle non-const operand e.g. `old * old`
                if let Ok(val) = val_s.parse::<isize>() {
                    operation = match op {
                        "+" => Box::from(move |x| x + val),
                        "*" => Box::from(move |x| x * val),
                        _ => todo!(),
                    }
                } else {
                    // TODO check this but assume old + old or old - old
                    operation = match op {
                        "+" => Box::from(|x| x + x),
                        "*" => Box::from(|x| x * x),
                        _ => todo!(),
                    }
                }
            } else if let Some(testy) = line.strip_prefix("Test: divisible by ") {
                testmod = testy.parse().unwrap();
                modulo *= testmod;
            } else if let Some(iffy) = line.strip_prefix("If true: throw to monkey ") {
                iftrue = iffy.parse().unwrap();
            } else if let Some(iffy) = line.strip_prefix("If false: throw to monkey ") {
                iffalse = iffy.parse().unwrap();
            }
        }

        let test = Box::from(move |x| if x % testmod == 0 { iftrue } else { iffalse });

        out.insert(
            k,
            Monkey {
                items,
                operation,
                test,
            },
        )
    }

    (out, modulo)
}

struct Monkey<'a> {
    items: VecDeque<isize>,
    operation: Box<dyn Fn(isize) -> isize + 'a>,
    test: Box<dyn Fn(isize) -> usize + 'a>,
}

impl<'a> Monkey<'a> {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
            operation: Box::new(|x| x),
            test: Box::new(|x| 0),
        }
    }
}

fn part_a(input: &str, rounds: usize) -> usize {
    let mut monkeys = parse_input(input).0;
    let mc = monkeys.len();

    let mut counts = vec![0_usize; mc];

    for roundn in 1..=rounds {
        // eprintln!("Round {roundn}");
        for k in 0..mc {
            let mut throws: Vec<(usize, isize)> = Vec::new();
            {
                let m = &mut monkeys[k];
                // eprintln!("Monkey {k}");
                while let Some(item) = m.items.pop_front() {
                    counts[k] += 1;
                    // eprintln!("\tMonkey inspects an item with a worry level of {item}");
                    let worry = (m.operation)(item);
                    // eprintln!("\tWorry level is now {worry}");
                    let worry = worry / 3;
                    // eprintln!("\tMonkey gets bored with item. Worry level is divided by 3 to {worry}");
                    let nextmonkey = (m.test)(worry);
                    // eprintln!("\tItem is thrown to monkey {nextmonkey}");
                    throws.push((nextmonkey, worry));
                }
                for (n, w) in throws {
                    monkeys[n].items.push_back(w);
                }
            }
        }
        // for (k, m) in monkeys.iter().enumerate() {
        //     eprintln!("Monkey {k}: {:?}", m.items);
        // }
    }

    // for (i, c) in counts.iter().enumerate() {
    //     eprintln!("Monkey {i} inspected items {c} times.");
    // }

    counts.sort_by(|a, b| b.cmp(a));

    counts[0] * counts[1]
}

/// Like part A but without the div-3
fn part_b(input: &str, rounds: usize) -> usize {
    let (mut monkeys, modulo) = parse_input(input);
    let mc = monkeys.len();

    let mut counts = vec![0_usize; mc];

    for roundn in 1..=rounds {
        // eprintln!("Round {roundn}");
        for k in 0..mc {
            let mut throws: Vec<(usize, isize)> = Vec::new();
            {
                let m = &mut monkeys[k];
                // eprintln!("Monkey {k}");
                while let Some(item) = m.items.pop_front() {
                    counts[k] += 1;
                    // eprintln!("\tMonkey inspects an item with a worry level of {item}");
                    let worry = (m.operation)(item);
                    // eprintln!("\tWorry level is now {worry}");
                    // For part B we now take a modulo
                    let worry = worry % modulo;
                    // eprintln!("\tMonkey gets bored with item. Worry level is divided by 3 to {worry}");
                    let nextmonkey = (m.test)(worry);
                    // eprintln!("\tItem is thrown to monkey {nextmonkey}");
                    throws.push((nextmonkey, worry));
                }
                for (n, w) in throws {
                    monkeys[n].items.push_back(w);
                }
            }
        }
        // if roundn == 20 || roundn % 1000 == 0 {
        //     eprintln!("Round {roundn}");
        //     for (i, c) in counts.iter().enumerate() {
        //         eprintln!("\tMonkey {i} inspected items {c} times.");
        //     }
        // }
    }

    counts.sort_by(|a, b| b.cmp(a));

    counts[0] * counts[1]
}
