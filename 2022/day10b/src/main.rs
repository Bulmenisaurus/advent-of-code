fn main() {
    // the value of X during the execution of the instruction
    let mut x_values: Vec<i32> = vec![];
    let mut X = 1;

    include_str!("./input.txt").lines().for_each(|line| {
        let parsed_command = line.split(" ").collect::<Vec<&str>>();

        match parsed_command[0] {
            "noop" => x_values.push(X),
            "addx" => {
                let amount = parsed_command[1].parse::<i32>().unwrap();

                for _ in 0..2 {
                    x_values.push(X);
                }

                X += amount;
            }
            _ => unimplemented!("{}", parsed_command[0]),
        }
    });

    const width: usize = 40;
    const height: usize = 6;
    let mut screen_data: [&str; width * height] = ["."; width * height];

    for x in 0..width {
        for y in 0..height {
            let sprite_x = x_values[x + y * width];
            let should_sprite_render = (x as i32 - sprite_x).abs() <= 1;

            screen_data[x + y * width] = if should_sprite_render { "#" } else { "." };
        }
    }

    println!(
        "{}",
        screen_data
            .chunks(width)
            .map(|c| { c.join("") })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
