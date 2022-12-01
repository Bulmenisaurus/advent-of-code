fn main() {
    let res = include_str!("./input.txt")
        .split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max();
    println!("{:?}", res);
}
