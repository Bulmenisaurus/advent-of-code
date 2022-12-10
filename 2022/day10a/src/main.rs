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

    let res = (x_values[19] * 20
        + x_values[59] * 60
        + x_values[99] * 100
        + x_values[139] * 140
        + x_values[179] * 180
        + x_values[219] * 220);

    println!("{:?}", res);
}
