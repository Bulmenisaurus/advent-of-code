// just visits every single accessible tile until the target index is reached
// note: don't discard the pool of previously visited tile indices, they are used to check which tiles
fn explore(
    height_map: Vec<i32>,
    height_map_width: i32,
    start_index: usize,
    end_index: usize,
) -> i32 {
    let mut visited_tile_indices: Vec<usize> = vec![];
    let mut new_tiles_previous_turn: Vec<usize> = vec![start_index];
    let mut i: i32 = 0;

    while !(visited_tile_indices.contains(&end_index)
        || new_tiles_previous_turn.contains(&end_index))
    {
        let mut new_tiles_this_turn: Vec<usize> = vec![];
        for visited_tile_index in &new_tiles_previous_turn {
            let tile_direction_offsets = [
                -height_map_width, // up
                1,                 // right
                height_map_width,  // down
                -1,                // left
            ];

            for direction_offset in tile_direction_offsets {
                let offset_tile_index = *visited_tile_index as i32 + direction_offset;

                // overflow over row
                if (direction_offset == -1 && *visited_tile_index as i32 % height_map_width == 0)
                    || (direction_offset == 1
                        && *visited_tile_index as i32 % height_map_width == height_map_width - 1)
                {
                    continue;
                }

                // skip if the index if off the map
                if offset_tile_index < 0
                    || offset_tile_index >= height_map.len().try_into().unwrap()
                {
                    continue;
                }

                // skip tile if already visited
                if visited_tile_indices.contains(&(offset_tile_index as usize))
                    || new_tiles_this_turn.contains(&(offset_tile_index as usize))
                {
                    continue;
                }

                // check if index can be accessed according to rules

                let from_tile_height = height_map[*visited_tile_index];
                let to_tile_height = height_map[offset_tile_index as usize];

                let can_tile_be_reached = to_tile_height - from_tile_height <= 1;

                if !can_tile_be_reached {
                    continue;
                }

                // successfully traversed this new tile! woohoo!

                new_tiles_this_turn.push(offset_tile_index as usize);
            }
        }

        visited_tile_indices.append(&mut new_tiles_previous_turn);
        new_tiles_previous_turn.append(&mut new_tiles_this_turn);
        i += 1;
    }

    i
}

fn main() {
    let grid_tiles = include_str!("./input.txt");

    let map_width = grid_tiles.split("\n").nth(0).unwrap().len() as i32;

    let grid_tiles = grid_tiles.replace("\n", "").chars().collect::<Vec<char>>();

    let start_index = grid_tiles.iter().position(|&c| c == 'S').unwrap();
    let end_index = grid_tiles.iter().position(|&c| c == 'E').unwrap();

    let grid_tiles = grid_tiles
        .iter()
        .map(|c| {
            let tile_name = match c {
                'S' => &'a',
                'E' => &'z',
                everything_else => everything_else,
            };

            let tile_height: i32 = "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .collect::<String>()
                .find(*tile_name)
                .unwrap() as i32;

            tile_height
        })
        .collect::<Vec<i32>>();

    let path_length = explore(grid_tiles, map_width, start_index, end_index);

    println!("{:?}", path_length);
}
