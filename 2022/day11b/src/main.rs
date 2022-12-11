use std::collections::HashMap;
use std::iter;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation_type: Operation,
    operation_value: i32,
    mod_test: i32,
    mod_test_true_monkey: i32,
    mod_test_false_monkey: i32,
}

#[derive(Debug, Clone)]
struct FunkyMonky {
    items: Vec<NumberRemainderManager>,
    operation_type: Operation,
    operation_value: i32,
    mod_test: i32,
    mod_test_true_monkey: i32,
    mod_test_false_monkey: i32,
}

#[derive(Debug, Clone)]
struct NumberRemainderManager {
    remainders: HashMap<i32, i32>,
}

fn initialize_num_remainder(integer: i32, moduli: Vec<i32>) -> NumberRemainderManager {
    let mut remainder_hash_map: HashMap<i32, i32> = HashMap::new();

    for modulus in moduli {
        remainder_hash_map.insert(modulus, integer % modulus);
    }

    NumberRemainderManager {
        remainders: remainder_hash_map,
    }
}

impl NumberRemainderManager {
    // re-mods all of the values
    fn recalculate_remainders(&mut self) {
        for (key, value) in self.remainders.clone().into_iter() {
            self.remainders.insert(key, value % key);
        }
    }
    fn add(&mut self, to: i32) {
        for (key, value) in self.remainders.clone().into_iter() {
            self.remainders.insert(key, value + to);
        }

        self.recalculate_remainders();
    }

    fn multiply(&mut self, by: i32) {
        for (key, value) in self.remainders.clone().into_iter() {
            self.remainders.insert(key, value * by);
        }

        self.recalculate_remainders();
    }

    fn square(&mut self) {
        for (key, value) in self.remainders.clone().into_iter() {
            self.remainders.insert(key, value * value);
        }

        self.recalculate_remainders();
    }

    fn get_mod(&self, by: i32) -> &i32 {
        self.remainders.get(&by).unwrap()
    }
}

fn apply_monkey_worry_level(
    item: NumberRemainderManager,
    operation_type: Operation,
    operation_value: i32,
) -> NumberRemainderManager {
    let mut item = item.clone();

    match operation_type {
        Operation::Add => item.add(operation_value),
        Operation::Multiply => item.multiply(operation_value),
        Operation::Square => item.square(),
    }

    item
}

fn simulate_monkey_round(monkeys: Vec<FunkyMonky>) -> (Vec<i32>, Vec<FunkyMonky>) {
    let mut new_monkeys = monkeys.clone();
    let mut inspections = iter::repeat(0).take(monkeys.len()).collect::<Vec<i32>>();

    for i in 0..monkeys.len() {
        let monkey = new_monkeys[i].clone();
        let items = monkey.items.clone();

        for item in items {
            inspections[i] += 1;

            let item_worry_level =
                apply_monkey_worry_level(item, monkey.operation_type, monkey.operation_value);

            let monkey_throw_target = if item_worry_level.get_mod(monkey.mod_test) == &0 {
                monkey.mod_test_true_monkey
            } else {
                monkey.mod_test_false_monkey
            };

            new_monkeys[monkey_throw_target as usize]
                .items
                .push(item_worry_level);

            new_monkeys[i].items.clear();
        }
    }

    (inspections, new_monkeys)
}

fn main() {
    let unfunkied_monkeys: Vec<Monkey> = include_str!("./input.txt")
        .split("\n\n")
        .map(|monkey_str| {
            let monkey_lines = monkey_str.lines().collect::<Vec<&str>>();

            let items: Vec<i32> = monkey_lines[1]
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(", ")
                .map(|item| item.parse::<i32>().unwrap())
                .collect();

            let operation_value = monkey_lines[2].split(" ").last().unwrap();

            let operation_type = if monkey_lines[2].contains("*") {
                if operation_value == "old" {
                    Operation::Square
                } else {
                    Operation::Multiply
                }
            } else {
                Operation::Add
            };

            let operation_value = operation_value.parse::<i32>().unwrap_or(0);

            let mod_test = monkey_lines[3]
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let rec1 = monkey_lines[4]
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let rec2 = monkey_lines[5]
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            Monkey {
                items,
                mod_test,
                operation_type,
                operation_value,
                mod_test_true_monkey: rec1,
                mod_test_false_monkey: rec2,
            }
        })
        .collect();

    let mut required_divisibility_checks = unfunkied_monkeys
        .iter()
        .map(|m| m.mod_test)
        .collect::<Vec<i32>>();
    required_divisibility_checks.dedup();
    required_divisibility_checks.sort();

    let funkied_monkies: Vec<FunkyMonky> = unfunkied_monkeys
        .iter()
        .map(|c| FunkyMonky {
            items: c
                .items
                .iter()
                .map(|&item| initialize_num_remainder(item, required_divisibility_checks.clone()))
                .collect::<Vec<NumberRemainderManager>>(),
            mod_test: c.mod_test,
            mod_test_false_monkey: c.mod_test_false_monkey,
            mod_test_true_monkey: c.mod_test_true_monkey,
            operation_type: c.operation_type,
            operation_value: c.operation_value,
        })
        .collect();

    // https://stackoverflow.com/a/29537747/13996389
    let mut monke_inspections: Vec<i32> = iter::repeat(0).take(funkied_monkies.len()).collect();

    let mut monke = funkied_monkies;
    for _ in 0..10000 {
        let res = simulate_monkey_round(monke);
        let inspections = res.0;

        inspections
            .iter()
            .enumerate()
            .for_each(|(i, inspections)| monke_inspections[i] += inspections);

        monke = res.1;
    }

    monke_inspections.sort();
    monke_inspections.reverse();

    println!("{:?} * {:?}", monke_inspections[0], monke_inspections[1]);
}
