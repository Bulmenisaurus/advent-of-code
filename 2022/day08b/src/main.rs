const SIZE: usize = 99;

fn scenic_score(tree_x: usize, tree_y: usize, grid: [i32; SIZE * SIZE], grid_size: usize) -> usize {
    let tree_height = grid[tree_x + grid_size * tree_y];

    // check right

    let scenic_score_right = (tree_x + 1..grid_size)
        .find(|x| grid[x + grid_size * tree_y] >= tree_height)
        .unwrap_or(grid_size - 1)
        - tree_x;

    let scenic_score_left = tree_x
        - (0..tree_x)
            .rev()
            .find(|x| grid[x + grid_size * tree_y] >= tree_height)
            .unwrap_or(0);

    let scenic_score_top = tree_y
        - (0..tree_y)
            .rev()
            .find(|y| grid[tree_x + grid_size * y] >= tree_height)
            .unwrap_or(0);

    let scenic_score_bottom = (tree_y + 1..grid_size)
        .find(|y| grid[tree_x + grid_size * y] >= tree_height)
        .unwrap_or(grid_size - 1)
        - tree_y;

    return scenic_score_right * scenic_score_left * scenic_score_top * scenic_score_bottom;
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

    let mut visible_trees = (0..SIZE)
        .map(|x| (0..SIZE).map(move |y| scenic_score(x, y, trees_grid, SIZE)))
        .flatten()
        .collect::<Vec<usize>>();

    visible_trees.sort();
    visible_trees.reverse();

    println!("{}", visible_trees[0]);
}
