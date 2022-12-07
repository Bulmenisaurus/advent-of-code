use std::collections::HashMap;

fn main() {
    let commands = include_str!("./input.txt")
        .split("$")
        .filter_map(|c| {
            let trimmed = c.trim();

            if c.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect::<Vec<&str>>();

    let mut path: Vec<&str> = vec![];
    let mut file_system: HashMap<String, i32> = HashMap::new();

    commands.iter().for_each(|c| {
        let parsed = c.split("\n").collect::<Vec<&str>>();
        let command = parsed[0].split(" ").collect::<Vec<&str>>();

        match command[0] {
            "cd" => match command[1] {
                "/" => {
                    path.clear();
                }
                ".." => {
                    path.pop();
                }
                value => {
                    path.push(value);
                }
            },
            "ls" => {
                let output_files = &parsed[1..];

                let directory_size: i32 = output_files
                    .iter()
                    .filter_map(|file| {
                        let parsed_ls_line = file.split(" ").collect::<Vec<&str>>();
                        match parsed_ls_line[0] {
                            "dir" => None,
                            value => Some(value.parse::<i32>().unwrap()),
                        }
                    })
                    .sum();

                // append the size of the subdirectory to all previous directories
                for i in 0..path.len() + 1 {
                    let mut directory_path = String::from("/");
                    directory_path.push_str(&path[0..i].join("/"));

                    if let Some(directory_previous_size) = file_system.get(&directory_path) {
                        file_system
                            .insert(directory_path, directory_previous_size + directory_size);
                    } else {
                        file_system.insert(directory_path, directory_size);
                    }
                }
            }
            _ => unreachable!(),
        }
    });

    let res: i32 = file_system
        .iter()
        .filter_map(
            |(_, value)| {
                if value <= &100000 {
                    Some(value)
                } else {
                    None
                }
            },
        )
        .sum();

    println!("{:?}", res);
}
