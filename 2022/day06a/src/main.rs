fn main() {
    let mut has_result_been_found = false;
    include_str!("./input.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .for_each(|(i, window)| {
            if has_result_been_found {
                return ();
            }

            for i in 0..window.len() - 1 {
                for j in i + 1..window.len() {
                    if window[i] == window[j] {
                        return;
                    }
                }
            }

            println!("{}", i + 4);
            has_result_been_found = true;
        });
}
