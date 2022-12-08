use std::{io, collections::HashSet};


fn main() -> io::Result<()> {
    const RADIX : u32 = 10;

    let grid : Vec<Vec<i8>> = io::stdin().lines().filter_map(|x| x.ok()).map(
            |s| s.chars().filter_map(|n| n.to_digit(RADIX)).map(|n| n as i8).collect()
        ).collect();

    println!("Part A {}", part_a(&grid));        

    println!("Part B {}", part_b(&grid));        

    Ok(())
}

/// Treetop tree house
/// Part A:
/// Count the number of trees that are visible from outside the grid when looking directly along a row or column.
/// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
/// Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
/// Every edge tree is visible as well.
/// Part A: how many trees are visible from outside the grid?
/// Need to find peaks running north, south, east and west along grid
fn part_a(grid: &Vec<Vec<i8>>) -> usize {
    // bounds:
    let width = grid.iter().map(|s| s.len()).max().unwrap();
    let height = grid.len();

    // Grid is indexed row-major with (0,0) at northwest

    // Complexity: trees can be of height zero
    // Luckily, all tree heights easily fit in an i8 so we can use -1

    // Complexity: double-counting trees
    // Simple solution: use a HashSet

    let mut tall_trees : HashSet<(usize, usize)> = HashSet::new();

    let mut max_so_far;

    // West to East
    for row in 0..height {
        // reset: west to east is by rows
        max_so_far = -1;
        for col in 0..width {
            if grid[row][col] > max_so_far {
                tall_trees.insert((row, col));
                max_so_far = grid[row][col];
            }           
        }
    }

    // East to West
    for row in 0..height {
        // reset: east to west is by rows
        max_so_far = -1;
        for col in 1..width {
            // reversal
            let col = width - col; 
            if grid[row][col] > max_so_far {
                 tall_trees.insert((row, col));
                max_so_far = grid[row][col];
            }           
        }
    }

    // North to South
    for col in 0..width {
        // reset: north to south is by cols
        max_so_far = -1;
        for row in 0..height {
            if grid[row][col] > max_so_far {
                 tall_trees.insert((row, col));
                max_so_far = grid[row][col];
            }           
        }
    }

    // South to North
    for col in 0..width {
        // reset: south to north is by cols
        max_so_far = -1;
        for row in 1..height {
            let row = height - row;
            if grid[row][col] > max_so_far {
                 tall_trees.insert((row, col));
                max_so_far = grid[row][col];
            }           
        }
    }
    
    tall_trees.len()
}


/// Part B: 
/// Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.
/// To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height 
/// or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)
/// A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
/// Question: what is the highest scenic score possible for any tree?
fn part_b(grid: &Vec<Vec<i8>>) -> usize {
    // OK so
    //  for each tree:
    //      for each direction:
    //          measure distance to edge or equal/taller tree
    //      multiply distances
    // n.b. edge trees have a score of zero so we don't need to consider them as sources

    // bounds:
    let width = grid.iter().map(|s| s.len()).max().unwrap();
    let height = grid.len();

    let mut max_score = 0;

    for row in 1..(height-1) { 
        for col in 1..(width-1) { 
            let this = grid[row][col];
            // eprintln!("row: {row} col: {col} this: {this}");
            let mut score = 1;
            
            // South to North
            for d in 1..=row {
                let r = row - d;
                let that = grid[r][col];

                if that >= this || r == 0 {
                    // eprintln!("\tr: {r} col: {col} that: {that}");
                    score *= d;
                    break;
                }
            }
            // North to South
            for d in 1..(height - row) {
                let r = row + d;
                let that = grid[r][col];

                if that >= this || r == height-1 {
                    // eprintln!("\tr: {r} col: {col} that: {that}");
                    score *= d;
                    break;
                }
            }

            // East to West
            for d in 1..=col {
                let c = col - d;
                let that = grid[row][c];

                if that >= this  || c == 0 {
                    // eprintln!("\trow: {row} c: {c} that: {that}");
                    score *= d;
                    break;
                }
            }

            // West to East
            for d in 1..(width - col) {
                let c = col + d;
                let that = grid[row][c];

                if that >= this || c == width-1 {
                    // eprintln!("\trow: {row} c: {c} that: {that}");
                    score *= d;
                    break;
                }
            }


            if score > max_score {
                eprintln!("row: {row} col: {col} this: {this} score: {score}");
                max_score = score;
            }
        }
    }

    max_score
}
