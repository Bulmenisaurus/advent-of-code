fn str_halves(string: Vec<char>) -> (Vec<char>, Vec<char>) {
    let half_len = string.len() / 2;
    (string[..half_len].to_vec(), string[half_len..].to_vec())
}

fn chars_intersection(arr1: Vec<char>, arr2: Vec<char>) -> char {
    for item in arr1 {
        if arr2.contains(&item) {
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
    println!("{:?}", str_halves("abcdef".chars().collect::<Vec<char>>()));
    let result: usize = include_str!("./input.txt")
        .lines()
        .map(|i| {
            let characters: Vec<char> = i.chars().collect();
            let (left_half, right_half) = str_halves(characters);
            let common_item = chars_intersection(left_half, right_half);

            get_priority(common_item)
        })
        .sum();

    println!("{:?}", result);
}
