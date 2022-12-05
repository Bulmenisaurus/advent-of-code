use std::num;

fn parse_assignment(line: &str) -> (i32, i32, i32, i32) {
    let elfs = line
        .split(",")
        .map(|instructions| {
            let nums = instructions
                .split("-")
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (nums[0], nums[1])
        })
        .collect::<Vec<(i32, i32)>>();

    (elfs[0].0, elfs[0].1, elfs[1].0, elfs[1].1)
}

fn main() {
    let res = include_str!("./input.txt")
        .lines()
        .filter(|pair| {
            let (e1min, e1max, e2min, e2max) = parse_assignment(pair);

            // either e1min or e1max is inside the range [e2min, e2max], or both e1min and e1max surround elf 2's range

            (e2min <= e1min && e1min <= e2max)
                || (e2min <= e1max && e1max <= e2max)
                || (e1min <= e2min && e1max >= e2max)
        })
        .collect::<Vec<&str>>()
        .len();
    println!("{}", res);
}
