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

fn dfs_through_network(
    valves: HashMap<&str, Valve>,
    current_location: &str,
    minutes_remaining: i32,
    total_time: i32,
    best_value_encountered: &mut i32,
    valves_to_open: usize,
) -> i32 {
    // calculate the pressure released
    if minutes_remaining == 0 || valves_to_open == 0 {
        let current_time = total_time;
        let all_valves: i32 = valves
            .values()
            .map(|valve| {
                // calculate the pressure the valve has released
                let time_since_valve_opened = match valve.opened_time {
                    None => 0,
                    Some(time) => current_time - time,
                };

                time_since_valve_opened * valve.flow_rate
            })
            .sum();

        if all_valves > *best_value_encountered {
            *best_value_encountered = all_valves;
        }

        print!("\r{:?}", best_value_encountered);

        return all_valves;
    }

    // each round, we have several choices: either open the current valve (1m) or go to one of the destinations: (1m)
    let mut available_actions: Vec<ValveAction> = vec![];

    let destinations = &valves[current_location].destinations;

    for destination in destinations {
        available_actions.push(ValveAction::GoTo(destination));
    }

    let current_valve = valves.get(&current_location).unwrap();
    // if the current valve isn't opened and doesn't have a flow rate of 0, add the action of opening it to the choices
    if !current_valve.open && current_valve.flow_rate != 0 {
        available_actions.push(ValveAction::OpenValve(&current_location))
    }

    // pick the outcome with the best pressure released
    available_actions
        .iter()
        .map(|action| match action {
            ValveAction::GoTo(dest) => dfs_through_network(
                valves.clone(),
                dest,
                minutes_remaining - 1,
                total_time,
                best_value_encountered,
                valves_to_open,
            ),
            ValveAction::OpenValve(dest) => {
                let mut new_valve_network = valves.clone();

                let mut destination = new_valve_network[dest].clone();
                destination.open = true;
                destination.opened_time = Some(total_time - minutes_remaining);

                new_valve_network.insert(dest, destination);
                // new_valve_network.insert(d, v)
                // new_valve_network[dest].open = true;
                // new_valve_network[dest].opened_time = Some(minutes_remaining);

                dfs_through_network(
                    new_valve_network,
                    current_location,
                    minutes_remaining - 1,
                    total_time,
                    best_value_encountered,
                    valves_to_open - 1,
                )
            }
        })
        .max()
        .unwrap()
}

fn main() {
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

    let valves_to_open = valves.iter().filter(|valve| valve.flow_rate > 0).count();

    let valves = {
        let mut new_valves: HashMap<&str, Valve> = HashMap::new();

        for valve in valves {
            new_valves.insert(valve.name.clone(), valve);
        }

        new_valves
    };

    let res: i32 = dfs_through_network(valves, "AA", 30, 30, &mut 0, valves_to_open);

    println!("\ntotal: {}", res);
}
