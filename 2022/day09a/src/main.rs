enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn main() {
    let moves: Vec<(Direction, i32)> = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(" ").unwrap();

            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => unreachable!(),
            };

            let distance = distance.parse::<i32>().unwrap();

            (direction, distance)
        })
        .collect();

    let mut head_pos: (i32, i32) = (0, 0);
    let mut previous_head_pos: (i32, i32) = (0, 0);

    let mut tail_pos: (i32, i32) = (0, 0);

    let mut unique_head_positions: Vec<(i32, i32)> = vec![head_pos];
    let mut unique_tail_positions: Vec<(i32, i32)> = vec![tail_pos];

    for movement in moves {
        for _ in 0..movement.1 {
            match movement.0 {
                Direction::Up => head_pos.1 += 1,
                Direction::Down => head_pos.1 -= 1,
                Direction::Left => head_pos.0 -= 1,
                Direction::Right => head_pos.0 += 1,
            }

            if (unique_head_positions.iter().all(|pos| pos != &head_pos)) {
                unique_head_positions.push(head_pos);
            }

            let are_head_tail_adjacent =
                (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1;

            if !are_head_tail_adjacent {
                tail_pos = previous_head_pos;

                if unique_tail_positions.iter().all(|pos| pos != &tail_pos) {
                    unique_tail_positions.push(tail_pos);
                }
            }

            previous_head_pos = head_pos;
        }
    }
    println!("{:?}", unique_tail_positions.len());
}
