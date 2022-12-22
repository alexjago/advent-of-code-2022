use anyhow::Result;
// use itertools::Itertools;
use michie::memoized;
use std::collections::{BTreeMap, HashMap};

fn main() -> Result<()> {
    println!("Hello, world!");

    let input = read_input()?;

    // input.iter().for_each(|(k, b)| eprintln!("{k}: {b:?}"));

    // println!(
    //     "Blueprint 2: {}",
    //     run_blueprint(input.get(&2).unwrap(), 24, [0; 4], [1, 0, 0, 0], None)
    // );

    println!("Part A: {}", part_a(&input, 24));
    println!("Part B: {}", part_b(&input, 32));

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn resource_order(input: &Resource) -> usize {
    match *input {
        Resource::Ore => 0,
        Resource::Clay => 1,
        Resource::Obsidian => 2,
        Resource::Geode => 3,
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Robot {
    product: Resource,
    costs: [usize; 4],
}

impl Robot {
    fn new(product: Resource, ore_cost: usize, clay_cost: usize, obsidian_cost: usize) -> Self {
        Robot {
            product,
            costs: [ore_cost, clay_cost, obsidian_cost, 0],
        }
    }

    // How many robots of a given type can we build with the given resources?
    fn can_build(self: &Self, resources: [usize; 4]) -> usize {
        let mut yes = true;
        for k in 0..3 {
            yes &= self.costs[k] <= resources[k]
        }
        if yes {
            return 1 + self.can_build(sub_arr(&resources, &self.costs));
        } else {
            return 0;
        }
    }
}

fn add_arr<const COUNT: usize>(
    resources: &[usize; COUNT],
    robots: &[usize; COUNT],
) -> [usize; COUNT] {
    let mut out: [usize; COUNT] = [0; COUNT];
    for k in 0..COUNT {
        out[k] = resources[k].saturating_add(robots[k]);
    }
    out
}

fn sub_arr<const COUNT: usize>(
    resources: &[usize; COUNT],
    robots: &[usize; COUNT],
) -> [usize; COUNT] {
    let mut out: [usize; COUNT] = [0; COUNT];
    for k in 0..COUNT {
        out[k] = resources[k].saturating_sub(robots[k]);
    }
    out
}

type Blueprints = BTreeMap<usize, HashMap<Resource, Robot>>;

fn read_input() -> Result<Blueprints> {
    use crate::Resource::*;
    use regex::Regex;
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 11 obsidian.

    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    )?;

    let mut out = Blueprints::new();

    for l in std::io::stdin().lines() {
        let line = l?;

        let matches = re.captures(&line).unwrap();

        let blueprint_id: usize = matches.get(1).unwrap().as_str().parse()?;
        let ore_robot_ore_cost: usize = matches.get(2).unwrap().as_str().parse()?;
        let clay_robot_ore_cost: usize = matches.get(3).unwrap().as_str().parse()?;
        let obsidian_robot_ore_cost: usize = matches.get(4).unwrap().as_str().parse()?;
        let obsidian_robot_clay_cost: usize = matches.get(5).unwrap().as_str().parse()?;
        let geode_robot_ore_cost: usize = matches.get(6).unwrap().as_str().parse()?;
        let geode_robot_obsidian_cost: usize = matches.get(7).unwrap().as_str().parse()?;

        let robots = HashMap::from([
            (Ore, Robot::new(Ore, ore_robot_ore_cost, 0, 0)),
            (Clay, Robot::new(Clay, clay_robot_ore_cost, 0, 0)),
            (
                Obsidian,
                Robot::new(
                    Obsidian,
                    obsidian_robot_ore_cost,
                    obsidian_robot_clay_cost,
                    0,
                ),
            ),
            (
                Geode,
                Robot::new(Geode, geode_robot_ore_cost, 0, geode_robot_obsidian_cost),
            ),
        ]);

        out.insert(blueprint_id, robots);
    }
    Ok(out)
}

/// Add up the quality level for all blueprints
fn part_a(input: &Blueprints, total_time: usize) -> usize {
    input
        .iter()
        .map(|(k, b)| {
            (
                k,
                b,
                run_blueprint(
                    &input,
                    *k,
                    total_time,
                    [0; 4],
                    [1, 0, 0, 0],
                    Some(Resource::Ore),
                    usize::MIN,
                ),
            )
        })
        .inspect(|x| eprintln!("{x:?}"))
        .map(|(k, _, g)| k * g)
        .sum()
}

/// Find the quality level for a blueprint
// #[memoized(key_expr = (blueprint_id, total_time, starting_resources, starting_robots, last_built), store_type = HashMap<(usize, usize, [usize;4], [usize;4], Option<Resource>), usize>)]
fn run_blueprint(
    blueprints: &Blueprints,
    blueprint_id: usize,
    total_time: usize,
    starting_resources: [usize; 4],
    starting_robots: [usize; 4],
    last_built: Option<Resource>,
    best_so_far: usize,
) -> usize {
    // use Resource::*;
    // let mut ore: usize = starting_resources[0];
    // let mut clay: usize = starting_resources[1];
    // let mut obsidian: usize = starting_resources[2];
    let geodes: usize = starting_resources[3];

    // let mut ore_bots: usize = starting_robots[0];
    // let mut clay_bots: usize = starting_robots[1];
    // let mut obsidian_bots: usize = starting_robots[2];
    let geode_bots: usize = starting_robots[3];

    let blueprint = blueprints.get(&blueprint_id).unwrap();

    let max_needed = [
        blueprint
            .values()
            .map(|r| r.costs[0])
            .max()
            .unwrap_or(total_time * total_time * total_time),
        blueprint
            .values()
            .map(|r| r.costs[1])
            .max()
            .unwrap_or(total_time * total_time * total_time),
        blueprint
            .values()
            .map(|r| r.costs[2])
            .max()
            .unwrap_or(total_time * total_time * total_time),
        total_time * total_time * total_time,
    ];

    if total_time == 1 {
        // if geode_bots > 0 {
        //     eprintln!("\t{total_time}, {starting_resources:?}, {starting_robots:?}");
        // }
        return geodes + geode_bots;
    }

    let mut optimal = best_so_far;

    // aggressive pruning tactic: is the *absolute theoretical maximum* number of geodes
    // still less than the best? begone
    // this is what got it over the line!
    if (geodes + geode_bots * total_time + (total_time * (total_time - 1)) / 2) < best_so_far {
        return usize::MIN;
    }

    /*
        // The maximum number of robots for each resource buildable with the current stock
        let build_max = HashMap::from_iter(
            blueprint
                .iter()
                .map(|(resource, robot)| (resource, robot.can_build([ore, clay, obsidian, geodes]))),
        );

        // decision to make each turn: what, if anything, to build
       for combo in build_max.iter().combinations(4) {
            todo!()
        }

        for orebot_count in 0..*build_max.get(&Ore).unwrap_or(&0) {
            for claybot_count in 0..*build_max.get(&Clay).unwrap_or(&0) {
                for obsidianbot_count in 0..*build_max.get(&Obsidian).unwrap_or(&0) {
                    for geodebot_count in 0..*build_max.get(&Geode).unwrap_or(&0) {
                        let geode_total = 0;
                        for k in 0..orebot_count

                        todo!();
                    }
                }
            }
        }
    */
    if blueprint
        .get(&Resource::Geode)
        .unwrap()
        .can_build(starting_resources)
        > 0
    {
        // heuristic: always build the geodebot when you can
        let mut new_robots = starting_robots.clone();
        new_robots[3] += 1;

        let outcome = run_blueprint(
            blueprints,
            blueprint_id,
            total_time - 1,
            sub_arr::<4>(
                &add_arr::<4>(&starting_resources, &starting_robots),
                &blueprint.get(&Resource::Geode).unwrap().costs,
            ),
            new_robots,
            Some(Resource::Geode),
            optimal,
        );
        if outcome > optimal {
            optimal = outcome;
        }
    } else {
        for (resource, robot) in blueprint {
            if robot.can_build(starting_resources) > 0
                && *resource != Resource::Geode
                && !(robot.can_build(starting_resources) > 1 && last_built.is_none())
                && max_needed[resource_order(resource)] > starting_robots[resource_order(resource)]
            {
                // eprintln!("\t\tBuilding {resource:?} robot at {total_time}");
                let mut new_robots = starting_robots.clone();
                new_robots[resource_order(resource)] += 1;

                let outcome = run_blueprint(
                    blueprints,
                    blueprint_id,
                    total_time - 1,
                    sub_arr::<4>(
                        &add_arr::<4>(&starting_resources, &starting_robots),
                        &robot.costs,
                    ),
                    new_robots,
                    Some(*resource),
                    optimal,
                );
                if outcome > optimal {
                    optimal = outcome;
                    // choice = Some(*resource);
                }
            }
        }
    }
    let null_hyp = run_blueprint(
        blueprints,
        blueprint_id,
        total_time - 1,
        add_arr::<4>(&starting_resources, &starting_robots),
        starting_robots,
        None,
        optimal,
    );
    optimal.max(null_hyp)
}

fn run_blueprint_bottom_up(
    blueprint: &HashMap<Resource, Robot>,
    total_time: usize,
    starting_resources: [usize; 4],
    starting_robots: [usize; 4],
) -> usize {
    let mut cache: HashMap<(usize, [usize; 4], [usize; 4]), usize> = HashMap::new();

    // we need to iterate over a great many things

    // alright kids today's the day we learn about dynamic programming

    // for time in 1..=total_time {}

    todo!()
}

// "If you don't need more than N of a resource to make anything, you don't need to make more than N of that robot;
// past that point, everything it produces is guaranteed to go to waste."
// -- via /u/ephemient

// We can prune search states through a number of options:
// * since we can only build 1 robot per turn, we don't need more than N robots for a material where N is that material's largest cost
// * we can estimate an upper bound of ("if we could a geode robot from now to the end, how many geodes would we have") and prune if that's worse than an actual result
// * /u/stevie-o-read-it has a very interesting cheating strategy that's a little more complicated,
//   * by setting all ore costs to zero and building one of every robot (if possible) each minute you can greedy-solve a better upper bound
// * if you choose to NOT build a robot R at time T (but had the resources to do so) then you can safely prune building R at T+1

fn part_b(input: &Blueprints, total_time: usize) -> usize {
    input
        .iter()
        .filter(|(k, _)| **k < 4)
        .map(|(k, b)| {
            (
                k,
                b,
                run_blueprint(
                    input,
                    *k,
                    total_time,
                    [0; 4],
                    [1, 0, 0, 0],
                    Some(Resource::Ore),
                    usize::MIN,
                ),
            )
        })
        .inspect(|x| eprintln!("{x:?}"))
        .map(|(_, _, x)| x)
        .reduce(|x, a| x * a)
        .unwrap_or_default()
}
