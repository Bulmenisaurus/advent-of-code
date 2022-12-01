fn main() {
    let mut res = include_str!("./input.txt")
        .split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    res.sort();
    res.reverse();

    println!("{:?}", res[0] + res[1] + res[2]);
}
