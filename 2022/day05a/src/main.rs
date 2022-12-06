fn get_columns(array: Vec<char>, width: usize) -> Vec<Vec<char>> {
    let height = array.len() / width;
    (0..width)
        .map(|x| (0..height).map(|y| array[x + width * y]).collect())
        .collect()
}

fn execute_steps(steps: &str, c: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_columns: Vec<Vec<char>> = vec![vec![]; c.len()];

    // take each column, filter out empty spaces
    for (i, column) in c.iter().enumerate() {
        new_columns[i] = column
            .iter()
            .filter(|&&ch| ch != ' ')
            .copied()
            .collect::<Vec<char>>()
            .clone();
    }

    steps.lines().for_each(|line| {
        let parsed_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let amount = parsed_line[1].parse::<usize>().unwrap();
        let from_index = parsed_line[3].parse::<usize>().unwrap() - 1;
        let to_index = parsed_line[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let item_to_move = new_columns[from_index][0];
            // if only pop actually just returned the 0th item..
            new_columns[from_index] = new_columns[from_index][1..].to_vec();
            new_columns[to_index].insert(0, item_to_move);
        }
    });

    new_columns
}

fn main() {
    let parsed_sections = include_str!("./input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut tower_rows = parsed_sections[0]
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| (chunk[1]).to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let num_columns = tower_rows.remove(tower_rows.len() - 1).len();

    let tower_columns: Vec<char> = tower_rows
        .iter()
        .flatten()
        .map(|i| i.chars().nth(0).unwrap())
        .collect();

    let tower_columns = get_columns(tower_columns, num_columns);

    let steps = parsed_sections[1];
    let finished_stacks = execute_steps(steps, tower_columns);

    let finished_stacks = finished_stacks
        .iter()
        .map(|s| s[0].to_string())
        .collect::<Vec<String>>();
    let finished_stacks: String = finished_stacks.join("");

    println!("{}", finished_stacks);
}
