use std::collections::HashMap;

fn points_between_lines(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let x_distance = b.0 - a.0;
    let y_distance = b.1 - a.1;

    let distance = std::cmp::max(x_distance.abs(), y_distance.abs());

    debug_assert!(x_distance.abs() == y_distance.abs() || x_distance == 0 || y_distance == 0);

    (0..distance)
        .map(|i| {
            let x_offset = i * x_distance.signum();
            let y_offset = i * y_distance.signum();

            (a.0 + x_offset, a.1 + y_offset)
        })
        .collect()
}

fn get_path_points(segments: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut resulting_points: Vec<(i32, i32)> = vec![];

    for i in 1..segments.len() {
        let current_point = segments[i];
        let previous_point = segments[i - 1];

        let mut points_between = points_between_lines(previous_point, current_point);

        resulting_points.append(&mut points_between);
    }

    resulting_points.push(*segments.last().unwrap());

    resulting_points
}

#[derive(Debug)]
enum TileType {
    Sand,
    Air,
    Stone,
}

fn drop_sand(
    world_tile_map_keys: Vec<&(i32, i32)>,
    source_coordinate: (i32, i32),
    floor_y: i32,
) -> (i32, i32) {
    let mut sand_coordinate = source_coordinate;

    loop {
        let candidates = [
            (sand_coordinate.0, sand_coordinate.1 + 1),
            (sand_coordinate.0 - 1, sand_coordinate.1 + 1),
            (sand_coordinate.0 + 1, sand_coordinate.1 + 1),
        ];

        let mut has_found_candidate = false;
        for candidate in candidates {
            if !world_tile_map_keys.contains(&&candidate) {
                sand_coordinate = candidate;
                has_found_candidate = true;

                break;
            }
        }

        // sand has come to a rest
        if !has_found_candidate {
            return sand_coordinate;
        }

        // sand is right above above
        if sand_coordinate.1 + 1 == floor_y {
            return sand_coordinate;
        }
    }
}

fn main() {
    let mut used_tiles = include_str!("./input.txt")
        .lines()
        .map(|lines| {
            let path_segments = lines
                .split(" -> ")
                .map(|segment| {
                    let (x, y) = segment.split_once(",").unwrap();
                    let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

                    (x, y)
                })
                .collect::<Vec<(i32, i32)>>();

            get_path_points(path_segments)
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    used_tiles.sort();
    used_tiles.dedup();

    let floor_location = used_tiles.iter().map(|tile| tile.1).max().unwrap() + 2;

    let mut world_tile_map: HashMap<(i32, i32), TileType> = HashMap::new();
    let source_coordinate = (500, 0);

    for tile in used_tiles {
        world_tile_map.insert(tile, TileType::Stone);
    }

    let mut sand_fallen = 1;
    loop {
        let keys = world_tile_map.keys().collect::<Vec<&(i32, i32)>>();

        let new_sand_pos = drop_sand(keys, source_coordinate, floor_location);

        world_tile_map.insert(new_sand_pos, TileType::Sand);

        if world_tile_map.contains_key(&(500, 0)) {
            break;
        }

        sand_fallen += 1;

        println!("[{}]{:?}", sand_fallen, new_sand_pos);
    }

    println!("{}", sand_fallen);
}
