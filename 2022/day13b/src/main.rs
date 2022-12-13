#[derive(Debug, Clone)]
enum NumArray {
    Array(Vec<NumArray>),
    Number(i32),
}

fn parse_brackets(brackets: &str) -> Vec<NumArray> {
    let mut stack: Vec<Vec<NumArray>> = vec![];

    let mut current_item_buffer: String = String::new();

    for c in brackets.chars() {
        match c {
            '[' => stack.push(vec![]),
            ']' => {
                if !current_item_buffer.is_empty() {
                    let stack_last_item = stack.len() - 1;
                    stack[stack_last_item].push(NumArray::Number(
                        current_item_buffer.parse::<i32>().unwrap(),
                    ));

                    current_item_buffer.clear()
                }

                let finished_arr = stack.pop().unwrap();

                if stack.len() == 0 {
                    return finished_arr;
                } else {
                    let stack_last_item = stack.len() - 1;
                    stack[stack_last_item].push(NumArray::Array(finished_arr));
                }
            }
            ',' => {
                if current_item_buffer.is_empty() {
                    continue;
                }

                let item = current_item_buffer.parse::<i32>().unwrap();
                let stack_last_item = stack.len() - 1;
                stack[stack_last_item].push(NumArray::Number(item));

                current_item_buffer.clear();
            }
            ' ' => {}
            other_char => current_item_buffer.push(other_char),
        }
    }

    unreachable!();
}

enum CompareResult {
    LessThan,
    MoreThan,
    Indeterminate,
}

fn compare_num_array(left: &NumArray, right: &NumArray) -> CompareResult {
    match (left, right) {
        (NumArray::Number(a), NumArray::Number(b)) => {
            if a < b {
                CompareResult::LessThan
            } else if b < a {
                CompareResult::MoreThan
            } else {
                CompareResult::Indeterminate
            }
        }

        (NumArray::Array(a), NumArray::Array(b)) => {
            let list_compare_len: usize = std::cmp::min(a.len(), b.len());

            for i in 0..list_compare_len {
                let left_item = &a[i];
                let right_item = &b[i];

                let item_compare_result = compare_num_array(&left_item, &right_item);

                match item_compare_result {
                    CompareResult::LessThan => return CompareResult::LessThan,
                    CompareResult::MoreThan => return CompareResult::MoreThan,
                    CompareResult::Indeterminate => {}
                }
            }

            if a.len() < b.len() {
                CompareResult::LessThan
            } else if b.len() < a.len() {
                CompareResult::MoreThan
            } else {
                CompareResult::Indeterminate
            }
        }
        (NumArray::Array(a), NumArray::Number(b)) => compare_num_array(
            &NumArray::Array(a.to_vec()),
            &NumArray::Array(vec![NumArray::Number(*b)]),
        ),
        (NumArray::Number(a), NumArray::Array(b)) => compare_num_array(
            &NumArray::Array(vec![NumArray::Number(*a)]),
            &NumArray::Array(b.to_vec()),
        ),
    }
}

fn main() {
    let mut packets = include_str!("./input.txt").replace("\n\n", "\n");
    packets.push_str("\n[[2]]\n[[6]]");

    let mut packets = packets
        .split("\n")
        .map(|packet| NumArray::Array(parse_brackets(packet)))
        .collect::<Vec<NumArray>>();

    packets.sort_by(|a, b| {
        let comparison = compare_num_array(a, b);

        match comparison {
            CompareResult::LessThan => std::cmp::Ordering::Less,
            CompareResult::MoreThan => std::cmp::Ordering::Greater,
            CompareResult::Indeterminate => unreachable!(),
        }
    });

    let divider_packet_a = NumArray::Array(vec![NumArray::Array(vec![NumArray::Number(2)])]);
    let divider_packet_b = NumArray::Array(vec![NumArray::Array(vec![NumArray::Number(6)])]);

    let index_a = packets
        .iter()
        .enumerate()
        .find(
            |(_, item)| match compare_num_array(&divider_packet_a, item) {
                CompareResult::LessThan => false,
                CompareResult::MoreThan => false,
                CompareResult::Indeterminate => true,
            },
        )
        .unwrap()
        .0;

    let index_b = packets
        .iter()
        .enumerate()
        .find(
            |(_, item)| match compare_num_array(&divider_packet_b, item) {
                CompareResult::LessThan => false,
                CompareResult::MoreThan => false,
                CompareResult::Indeterminate => true,
            },
        )
        .unwrap()
        .0;

    println!("{}", ((index_a + 1) * (index_b + 1)));
}
