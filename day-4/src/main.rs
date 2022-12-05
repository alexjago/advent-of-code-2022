use std::io;

fn main() -> io::Result<()> {
    let mut total_a = 0;
    let mut total_b = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let nums: Vec<i32> = line?
            .split(',')
            .flat_map(|x| x.split('-'))
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        let d = nums[3];

        // Part A: range AB or range CD
        // completely contained within the other
        if (a <= c && b >= d) || (c <= a && d >= b) {
            total_a += 1;
        }

        // Part B: range AB and CD overlap
        if (a <= c && c <= b) || (c <= b && b <= d) || (a <= d && d <= b) || (c <= a && a <= d) {
            total_b += 1
        }
    }

    println!("Total part A: {}", total_a);
    println!("Total part B: {}", total_b);

    Ok(())
}
