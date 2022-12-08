use std::collections::BTreeMap;
/// AOC Day 7

/// We have a terminal session to read through
/// From this we need to reconstruct a file tree

// $ cd / : return cursor to top of tree
// $ cd .. : go up one level in tree

fn main() -> std::io::Result<()> {
    // construct a tree

    let mut filetree: BTreeMap<Vec<String>, BTreeMap<FileName, usize>> = BTreeMap::new();

    let mut curdir_path = vec![];

    // Parse loop
    for lrez in stdin().lines() {
        let llll = lrez.unwrap();
        let line = llll.trim();

        let what = parse_line(line);

        // eprintln!("{}\n{:?}", line, what);

        match what {
            LineType::cmd(c) => {
                match c {
                    Command::ls => {
                        // Files listed by ls may need to be created in tree
                    }
                    Command::cd(l) => match l {
                        Location::Root => {
                            curdir_path.clear();
                        }
                        Location::Up => {
                            let _ = curdir_path.pop();
                        }
                        Location::Down(f) => curdir_path.push(f.clone()),
                    },
                }
            }
            LineType::output(node) => {
                // OK, we have to insert
                match node {
                    Node::File(name, size) => {
                        filetree
                            .entry(curdir_path.clone())
                            .or_default()
                            .insert(name, size);
                    }
                    _ => {}
                }
            }
        }
    }

    // Now that we have constructed a file tree we can do operations on it

    //     eprintln!("Directs:");

    let directs: BTreeMap<Vec<String>, usize> = BTreeMap::from_iter(
        filetree
            .iter()
            .map(|(path, files)| (path.clone(), files.values().sum::<usize>())), //             .inspect(|(p, v)| eprintln!("{:?} {}", p, v))
    );

    // ^^ this is almost correct.
    // it gets the size of each directory from the DIRECT contents
    // but does not account for INDIRECT contents

    // We'll build up an indirect listing as follows

    let maxdepth = filetree.keys().map(|k| k.len()).max().unwrap_or(0);

    let mut running_totals = directs.clone();

    for iii in 0..maxdepth {
        let i = maxdepth - iii;
        //         eprintln!("{i}");
        let to_add: Vec<(Vec<String>, usize)> = running_totals
            .iter()
            .filter(|(k, _)| k.len() == i)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (k, v) in to_add {
            let mut parent = k.clone();
            parent.pop();
            *(running_totals.entry(parent).or_default()) += v;
        }
    }

    //     eprintln!("With indirects:");
    //
    //     for (k, v) in &running_totals {
    //         eprintln!("{:?} {}", k, v);
    //     }

    eprintln!("");

    println!(
        "Part A: {}",
        running_totals
            .iter()
            .filter(|(_, v)| v < &&100_000_usize)
            //             .inspect(|(k, v)| eprintln!("{:?} {}", k, v))
            .map(|(_, v)| *v)
            .sum::<usize>()
    );

    // eprintln!("{:#?}", filetree);

    // PART B

    let max_disk = 70_000_000_usize;
    let space_needed = 30_000_000;
    let total_used = running_totals.get(&vec![]).unwrap();
    let min_free = space_needed - (max_disk - total_used);

    // We need to find the smallest directory that is larger than min_free

    println!(
        "Part B: {}",
        running_totals
            .iter()
            .filter(|(_, v)| v > &&min_free)
            //             .inspect(|(k, v)| eprintln!("{:?} {}", k, v))
            .map(|(_, v)| *v)
            .min()
            .unwrap()
    );

    Ok(())
}

#[derive(Debug)]
enum Node {
    Dir,
    File(FileName, usize),
}

type FileName = String;

#[derive(Debug)]
enum Location {
    Root,
    Up,
    Down(FileName),
}
#[derive(Debug)]
enum Command {
    cd(Location),
    ls,
}

#[derive(Debug)]
enum LineType {
    cmd(Command),
    output(Node),
}

fn parse_line(input: &str) -> LineType {
    if input.starts_with('$') {
        return LineType::cmd(parse_command(input.strip_prefix("$ ").unwrap()));
    } else {
        return LineType::output(parse_file(input));
    }
}

fn parse_file(input: &str) -> Node {
    if input.starts_with("dir") {
        return Node::Dir;
    } else {
        if let Some((sz, nm)) = input.split_once(' ') {
            if let Ok(size) = sz.parse() {
                return Node::File(String::from(nm), size);
            }
        }
        panic!("This isn't supposed to happen")
    }
}

fn parse_command(input: &str) -> Command {
    if input.starts_with("cd") {
        let location = input.strip_prefix("cd ").unwrap();
        return match location {
            "/" => Command::cd(Location::Root),
            ".." => Command::cd(Location::Up),
            _ => Command::cd(Location::Down(String::from(location))),
        };
    } else if input.starts_with("ls") {
        return Command::ls;
    }
    panic!("This isn't supposed to happen!")
}
