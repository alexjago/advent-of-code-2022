use std::collections::{HashMap, HashSet, BTreeMap};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = parse_input()?;

    for (_, v) in &input {
        eprintln!("{:?}", v);
    }

    println!("Part A: {}", part_a()); 
    println!("Part B: {}", part_b());

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>
}

fn parse_input() -> Result<BTreeMap<String, Valve>> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB    
    let re = Regex::new(r"Valve (\w+) has flow rate=(-?\d+); tunnels? leads? to valves? (.*)")?;
    let mut out = BTreeMap::new();
    for l in std::io::stdin().lines() {
        let line = l?;
        // eprintln!("{line}");
        let matches = re.captures(&line).unwrap();

        let name = matches.get(1).unwrap().as_str().to_string();
        let rate: usize = matches.get(2).unwrap().as_str().parse()?;
        let tunnels: Vec<String> = matches.get(3).unwrap().as_str().split(", ").map(String::from).collect();

        out.insert(name.clone(), Valve {name, rate, tunnels});
    }

    Ok(out)
}

fn part_a() -> usize {
    todo!()
}



fn part_b() -> usize {
    todo!()
}

