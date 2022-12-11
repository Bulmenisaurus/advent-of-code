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

fn apply_monkey_worry_level(item: i32, operation_type: Operation, operation_value: i32) -> i32 {
    match operation_type {
        Operation::Add => item + operation_value,
        Operation::Multiply => item * operation_value,
        Operation::Square => item * item,
    }
}

fn monkey_get_check_res(worry_level: i32, mod_test: i32) -> bool {
    // tests divisibility by mod_test
    worry_level % mod_test == 0
}

fn simulate_monkey_round(monkeys: Vec<Monkey>) -> (Vec<i32>, Vec<Monkey>) {
    let mut new_monkeys = monkeys.clone();
    let mut inspections = iter::repeat(0).take(monkeys.len()).collect::<Vec<i32>>();

    for i in 0..monkeys.len() {
        let monkey = new_monkeys[i].clone();
        let items = &monkey.items;

        for item in items {
            inspections[i] += 1;

            let item_worry_level =
                apply_monkey_worry_level(*item, monkey.operation_type, monkey.operation_value);
            // worry level is divided by 3
            let item_worry_level = item_worry_level / 3;

            let monkey_throw_target = if monkey_get_check_res(item_worry_level, monkey.mod_test) {
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
    let monkeys: Vec<Monkey> = include_str!("./input.txt")
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

            let operation_value = operation_value.parse::<i32>().unwrap_or(-1);

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

    // https://stackoverflow.com/a/29537747/13996389
    let mut monke_inspections: Vec<i32> = iter::repeat(0).take(monkeys.len()).collect();

    let mut monke = monkeys;
    for i in 0..20 {
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

    println!("{:?}", monke_inspections[0] * monke_inspections[1]);
}
