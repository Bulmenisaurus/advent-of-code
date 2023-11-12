use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: i32,
    destinations: Vec<String>,
    open: bool,
    opened_time: Option<i32>,
}

enum ValveAction<'a> {
    GoTo(&'a str),
    OpenValve(&'a str),
}

fn find_quickest_path(start_valve: &str, end_valve: &str, network: &HashMap<&str, Valve>) -> i32 {
    let mut visited_tiles: Vec<&str> = vec![];
    let mut new_tiles_previous_turn: Vec<&str> = vec![start_valve];
    let mut i: i32 = 0;

    while !(new_tiles_previous_turn.contains(&end_valve)) {
        let mut new_tiles_this_turn: Vec<&str> = vec![];

        for visited_tile in &new_tiles_previous_turn {
            let valve = &network[visited_tile];
            for destination in &valve.destinations {
                if visited_tiles.contains(&destination.as_str())
                    || new_tiles_previous_turn.contains(&destination.as_str())
                {
                    continue;
                }
                new_tiles_this_turn.push(&destination);
            }
        }

        visited_tiles.append(&mut new_tiles_previous_turn);
        new_tiles_previous_turn.append(&mut new_tiles_this_turn);
        i += 1;
    }

    return i;
}

fn mama_mia(
    valves_network: &HashMap<&str, Valve>,
    valves_distances: &HashMap<(&str, &str), i32>,
    unvisited_valves: Vec<&&str>,
    current_valve: &str,
    time_remaining: i32,
    time_max: i32,
) -> i32 {
    if unvisited_valves.is_empty() {
        // calculate all the times and stuff
    }

    for unvisited_valve in unvisited_valves {
        let distance = valves_distances[&(current_valve, *unvisited_valve)];

        if (distance > time_remaining) {
            continue;
        }
    }

    todo!();
}

fn main() {
    // iterate through every single possible path of the valves, using dfs
    // steps:
    // 1) Parse the valves in the input
    // 2) Create a list of the valves with flow >0, those will be the ones we iterate over
    // 3) Pre computer a list of the shortest paths between each of the segments (do ties matter? - no, I don't think so)
    //
    // Then, recursively traverse the path, each iteration choosing which segment to go to. Once all non-zero valves are open, there is nothing we can do but wait
    // Each iteration in the function we need to keep track of which valves we haven't opened yet
    let valves: Vec<Valve> = include_str!("./input.example.txt")
        .lines()
        .map(|line| {
            let parsed = line.split(" ").collect::<Vec<&str>>();

            let valve_name = parsed[1];

            let valve_flow_rate = parsed[4];
            let valve_flow_rate = &valve_flow_rate[5..valve_flow_rate.len() - 1];
            let valve_flow_rate = valve_flow_rate.parse::<i32>().unwrap();

            let destinations = &parsed[9..];
            let destinations = destinations.to_vec();
            let destinations: Vec<String> =
                destinations.iter().map(|c| c.replace(",", "")).collect();

            Valve {
                name: valve_name,
                flow_rate: valve_flow_rate,
                destinations,
                open: false,
                opened_time: None,
            }
        })
        .collect();

    let valve_names = valves.iter().map(|valve| valve.name).collect::<Vec<&str>>();

    let valves = {
        let mut new_valves: HashMap<&str, Valve> = HashMap::new();

        for valve in valves {
            new_valves.insert(valve.name.clone(), valve);
        }

        new_valves
    };

    let valves_to_open = valve_names
        .iter()
        .filter(|&&valve| {
            let valve = &valves[valve];
            valve.flow_rate > 0
        })
        .collect::<Vec<&&str>>();

    let mut all_paths: HashMap<(&str, &str), i32> = HashMap::new();
    for starting_point in &valve_names {
        for ending_point in &valve_names {
            if starting_point == ending_point {
                continue;
            }

            all_paths.insert(
                (starting_point, ending_point),
                find_quickest_path(starting_point, ending_point, &valves),
            );
        }
    }

    // let res: i32 = dfs_through_network(valves, "AA", 30, 30, &mut 0, valves_to_open);
    let res: i32 = mama_mia(&valves, &all_paths, valves_to_open, "AA", 30, 30);

    println!("{:?}", all_paths);
}
