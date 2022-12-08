const SIZE: usize = 99;

fn is_tree_visible(
    tree_x: usize,
    tree_y: usize,
    grid: [i32; SIZE * SIZE],
    grid_size: usize,
) -> bool {
    if tree_x == 0 || tree_x == grid_size - 1 || tree_y == 0 || tree_y == grid_size - 1 {
        return true;
    }

    let tree_height = grid[tree_x + grid_size * tree_y];

    // check right
    let is_visible_right =
        (tree_x + 1..grid_size).all(|x| grid[x + grid_size * tree_y] < tree_height);

    if is_visible_right {
        return true;
    }

    let is_visible_left = (0..tree_x).all(|x| grid[x + grid_size * tree_y] < tree_height);

    if is_visible_left {
        return true;
    }

    let is_visible_top = (0..tree_y).all(|y| grid[tree_x + grid_size * y] < tree_height);
    if is_visible_top {
        return true;
    }

    let is_visible_bottom =
        (tree_y + 1..grid_size).all(|y| grid[tree_x + grid_size * y] < tree_height);
    if is_visible_bottom {
        return true;
    }

    return false;
}

fn main() {
    let trees_grid_file = include_str!("./input.txt");

    let trees_grid: [i32; SIZE * SIZE] = trees_grid_file
        .replace("\n", "")
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    let visible_trees = (0..SIZE)
        .map(|x| {
            (0..SIZE).filter_map(move |y| {
                if is_tree_visible(x, y, trees_grid, SIZE) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();
    println!("{}", visible_trees.len());
}
