use anyhow::{bail, Result};
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;

fn main() -> Result<()> {
    let input = construct_part_a_input()?;
    println!("Part A: {}", part_a(&input));
    Ok(())
}

fn construct_part_a_input() -> Result<Vec<(Value, Value)>> {
    let mut out = Vec::new();
    for chunk in &std::io::stdin()
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| s.trim().len() > 0)
        .chunks(2)
    {
        if let Some((l, r)) = chunk.collect_tuple() {
            let left = parse_value(&l)?.0;
            let right = parse_value(&r)?.0;
            eprintln!("L: {l}   =>   {left:?}");
            eprintln!("R: {r}   =>   {right:?}");
            out.push((left, right))
        } else {
            bail!("weirdness!")
        }
    }
    Ok(out)
}

/// hmmmmmmmmmm
fn parse_value(input: &str) -> Result<(Value, usize)> {
    // eprintln!("{input}");
    use Value::*;
    let mut curnum = String::new();

    let mut out: Value = {
        if let Some(c) = input.chars().nth(0) {
            match c {
                '[' => List(vec![]),
                '0'..='9' => Integer(isize::MAX),
                _ => bail!("Unexpected item in bagging area"),
            }
        } else {
            bail!("Unexpected end of string")
        }
    };

    let mut i = 0;
    while i < input.len() {
        let c = input.chars().nth(i).unwrap();
        i += 1; // now rather than later for reasons
        match (&mut out, c) {
            (_, '0'..='9') => curnum.push(c),
            (Integer(_), _) => {
                if !curnum.is_empty() {
                    return Ok((Integer(curnum.parse().unwrap()), i));
                }
            }
            (_, ' ') => {
                continue;
            }
            (List(l), ',') => {
                // guess there's more than one item
                if !curnum.is_empty() {
                    l.push(Integer(curnum.parse().unwrap()));
                    curnum.clear();
                }
            } // finish whatever we're doing
            (List(l), '[') => {
                if i > 1 {
                    // kick off a new vec
                    let (val, n) = parse_value(&input[i - 1..])?;
                    // eprintln!("\t{n} chars parsed as sub-value");
                    l.push(val);
                    i += n;
                }
            }
            (List(l), ']') => {
                if !curnum.is_empty() {
                    l.push(Integer(curnum.parse().unwrap()))
                }
                return Ok((out, i));
            } // finish up vec
            _ => {
                bail!("unimplemented")
            }
        }
    }
    Ok((out, i))
}

#[derive(Debug, Deserialize)]
enum Value {
    Integer(isize),
    List(Vec<Value>),
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(self: &Self, other: &Self) -> bool {
        self.cmp(&other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    /// Recursively compare Values
    fn cmp(self: &Value, other: &Value) -> std::cmp::Ordering {
        // eprintln!("Comparing {self:?} with {other:?}");
        use std::cmp::Ordering::*;
        use Value::{Integer, List};
        match (self, other) {
            (&Integer(i), &Integer(j)) => i.cmp(&j),
            (&Integer(i), _) => List(vec![Integer(i)]).cmp(other),
            (_, &Integer(i)) => self.cmp(&List(vec![Integer(i)])),
            (List(l), List(r)) => {
                for k in 0..(l.len().min(r.len())) {
                    let i = &l[k];
                    let j = &r[k];
                    match i.cmp(&j) {
                        Greater => return Greater,
                        Less => return Less,
                        Equal => continue,
                    }
                }
                return l.len().cmp(&r.len());
            }
        }
    }
}

/// Checking if pairs of lists are in the correct order
/// * if both values are integers, left <= right
/// * if both values are lists, recurse on contents OR left should be a left-subset of right
///     * e.g. [
/// * compare lists and integers by converting the integer to a single-element list
fn part_a(input: &[(Value, Value)]) -> usize {
    use std::cmp::Ordering::*;
    input
        .iter()
        .map(|(left, right)| left.cmp(right))
        .inspect(|c| eprintln!("{c:?}"))
        .enumerate()
        .map(|(i, c)| match c {
            Less | Equal => i+1,
            Greater => 0,
        })
        .inspect(|c| eprintln!("{c:?}"))

        .sum()
}


mod test {
    use super::*;
    use std::cmp::Ordering::*;
//    #[test]
    fn normal_lists() -> Result<()> {
        let left = parse_value("[1,1,3,1,1]")?.0;
        let right = parse_value("[1,1,5,1,1]")?.0;
        assert!(left.cmp(&right) == Less);
        Ok(())
    }

    #[test]
    fn nested_lists() -> Result<()> {
        let left = parse_value("[[1],[2,3,4]]")?.0;
        let right = parse_value("[[1], 4]")?.0;
        assert!(left.cmp(&right) == Less);
        Ok(()) 
    }


}