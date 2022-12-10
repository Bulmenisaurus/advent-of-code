enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_position(previous_position: (i32, i32), follow: (i32, i32)) -> (i32, i32) {
    let delta_x = follow.0 - previous_position.0;
    let delta_y = follow.1 - previous_position.1;

    let delta_movement = (delta_x, delta_y);

    // is there a clever formula for this?
    let delta_change = match delta_movement {
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),
        (1, -2) => (1, -1),
        (-1, -2) => (-1, -1),
        (-2, -1) => (-1, -1),
        (-2, 1) => (-1, 1),
        (-1, 2) => (-1, 1),
        (1, 2) => (1, 1),
        (2, 0) => (1, 0),
        (0, -2) => (0, -1),
        (-2, 0) => (-1, 0),
        // actually not sure how the next four are possible but whatever
        (0, 2) => (0, 1),
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (-2, -2) => (-1, -1),
        _ => {
            println!("{:?}", delta_movement);
            unreachable!()
        }
    };

    return (
        previous_position.0 + delta_change.0,
        previous_position.1 + delta_change.1,
    );
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

    const length: usize = 10;
    let mut point_positions: [(i32, i32); length] = [(0, 0); length];

    let mut unique_tail_positions: Vec<(i32, i32)> = vec![];

    for movement in moves {
        for _ in 0..movement.1 {
            match movement.0 {
                Direction::Up => point_positions[0].1 += 1,
                Direction::Down => point_positions[0].1 -= 1,
                Direction::Left => point_positions[0].0 -= 1,
                Direction::Right => point_positions[0].0 += 1,
            }

            for tail_i in 1..point_positions.len() {
                let previous_segment = point_positions[tail_i - 1];
                let current_segment = point_positions[tail_i];

                let are_segments_adjacent = (previous_segment.0 - current_segment.0).abs() <= 1
                    && (previous_segment.1 - current_segment.1).abs() <= 1;

                if !are_segments_adjacent {
                    point_positions[tail_i] = update_position(current_segment, previous_segment)
                }
            }

            let tail_pos = point_positions[length - 1];

            if unique_tail_positions.iter().all(|pos| pos != &tail_pos) {
                unique_tail_positions.push(tail_pos)
            }
        }
    }
    println!("{:?}", unique_tail_positions.len());
}
