use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = parse_input()?;

    println!("Part A: {}", part_a(input.clone()));

    println!("Part B: {}", part_b(input));

    Ok(())
}

fn part_a(mut input: HashMap<(X, Y), Material>) -> usize {
    // draw_grid(&input);

    while let Some(p) = sandfall(&input, (500, 0)) {
        input.insert(p, Material::Sand);
    }
    // draw_grid(&input);

    input.values().filter(|m| **m == Material::Sand).count()
}

/// now there's a floor...
fn part_b(mut input: HashMap<(X, Y), Material>) -> usize {
    let y_max = input.keys().map(|(_r, c)| *c).max().unwrap_or(0);

    // floor is level rock at y_max + 2
    // sand can only move one step right/left at a time so we only need to simulate x in
    // triangular range

    let x_min = 500 - 5 + y_max.abs();
    let x_max = 500 + 5 + y_max.abs();

    for x in x_min..=x_max {
        input.insert((x, y_max + 2), Material::Rock);
    }

    draw_grid(&input);

    let mut counter = 0;
    while let Some(p) = sandfall(&input, (500, 0)) {
        input.insert(p, Material::Sand);
        counter += 1;

        if counter % 1000 == 0 {
            eprint!("+")
        }
        // don't forget to cut the sand off!
        if p == (500, 0) {
            break;
        }
    }
    draw_grid(&input);

    // input.values().filter(|m| **m == Material::Sand).count()
    counter
}

/// Rightward
type X = isize;

/// Downward
type Y = isize;

#[derive(PartialEq, Eq, Clone)]
enum Material {
    Rock,
    Sand,
}

/// Return the position of all the rock
fn parse_input() -> Result<HashMap<(X, Y), Material>> {
    let mut out = HashMap::new();
    let mut paths: Vec<Vec<(X, Y)>> = vec![];

    for l in std::io::stdin().lines() {
        let line = l?;
        let mut p = vec![];
        for s in line.split(" -> ") {
            if let Some((x, y)) = s.split_once(',') {
                let x: isize = x.parse()?;
                let y: isize = y.parse()?;
                p.push((x, y));
            }
        }
        paths.push(p)
    }

    for p in paths {
        for w in p[..].windows(2) {
            if let [(x1, y1), (x2, y2)] = w {
                for x in *x1.min(x2)..=*x1.max(x2) {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        out.insert((x, y), Material::Rock);
                    }
                }
            }
        }
    }
    Ok(out)
}

fn draw_grid(input: &HashMap<(X, Y), Material>) {
    let x_min = input.keys().map(|(r, _c)| *r).min().unwrap_or(0);
    let x_max = input.keys().map(|(r, _c)| *r).max().unwrap_or(0);
    let y_min = input.keys().map(|(_r, c)| *c).min().unwrap_or(0);
    let y_max = input.keys().map(|(_r, c)| *c).max().unwrap_or(0);

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", x / 100)
    }
    println!();

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", (x % 100 - x % 10) / 10)
    }
    println!();

    print!("    ");
    for x in x_min..=x_max {
        print!("{}", x % 10)
    }
    println!();

    for y in y_min..=y_max {
        print!("{y:3} ");
        for x in x_min..=x_max {
            print!(
                "{}",
                match input.get(&(x, y)) {
                    Some(Material::Rock) => "#",
                    Some(Material::Sand) => "o",
                    None => ".",
                }
            )
        }
        println!();
    }
}

/// Simulate where a single block of falling sand would end up
fn sandfall(input: &HashMap<(X, Y), Material>, start: (X, Y)) -> Option<(X, Y)> {
    let y_max = input.keys().map(|(_, c)| *c).max().unwrap_or(0);
    // eprintln!("y_max: {y_max}");

    let mut y = start.1;
    let mut x = start.0;
    // eprintln!("({x}, {y})");

    while y < y_max {
        // eprintln!("({x}, {y})");
        // test coordinates
        if input.get(&(x, y + 1)).is_none() {
            // sand falls straight down
            y += 1;
        } else if input.get(&(x - 1, y + 1)).is_none() {
            // sand falls down-left
            y += 1;
            x -= 1;
        } else if input.get(&(x + 1, y + 1)).is_none() {
            // sand falls down-right
            y += 1;
            x += 1;
        } else {
            // sand cannot fall
            return Some((x, y));
        }
    }

    // Sand has reached the abyss
    None
}
