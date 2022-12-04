fn chars_intersection(arr1: Vec<char>, arr2: Vec<char>, arr3: Vec<char>) -> char {
    for item in arr1 {
        if arr2.contains(&item) && arr3.contains(&item) {
            return item;
        }
    }

    unreachable!();
}

fn get_priority(character: char) -> usize {
    let char_index = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .position(|c| c == character.to_lowercase().nth(0).unwrap())
        .unwrap()
        + 1;

    if character.is_uppercase() {
        return char_index + 26;
    } else {
        return char_index;
    }
}

fn main() {
    let result: usize = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|arr| {
            if let [a, b, c] = arr {
                let common_item = chars_intersection(
                    a.chars().collect::<Vec<char>>(),
                    b.chars().collect::<Vec<char>>(),
                    c.chars().collect::<Vec<char>>(),
                );

                get_priority(common_item)
            } else {
                unreachable!();
            }
        })
        .sum();

    println!("{:?}", result);
}
