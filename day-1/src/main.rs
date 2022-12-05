use std::io;

// we have an input file of newline-delimited numbers
// empty line = new Elf
// we want to sum these numbers by Elf
fn main() -> io::Result<()> {
    let lines = io::stdin().lines();
    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf = 0;
    for line in lines {
        if let Ok(x) = line?.parse::<i32>() {
            current_elf += x;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    // Sort by totals (low to high)
    elves.sort();

    // Part A: return the largest
    println!("Part A: {}", elves.last().unwrap());
    // Part B: return the sum of the 3 largest
    println!("Part A: {}", elves.iter().rev().take(3).sum::<i32>());

    Ok(())
}
