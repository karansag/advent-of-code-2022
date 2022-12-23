mod utils;

use std::collections::HashSet;
use std::iter::Iterator;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

type Grid = Vec<Vec<i8>>;
type Coord = (usize, usize);

fn read_input<'a, 'b>(file_path: &'b str, grid: &'a mut Grid) -> Result< &'a Grid, std::io::Error> {
    let contents = utils::read_file(file_path)?;
    let parsed_lines = contents.map(|l| l.unwrap());
    for l in parsed_lines {
        let mut new_row: Vec<i8> = vec![];
        for ch in l.chars() {
            new_row.push(ch.to_digit(10).unwrap() as i8);
        }
        grid.push(new_row);
    }

    Ok(grid)
}

fn vis_row<R: Iterator<Item=usize>>(row: &Vec<i8>, range: R) -> Vec<usize> {
    let mut current_max = -1;
    let mut visible_inds = vec![];
    for ind in range {
        if row[ind] > current_max {
            visible_inds.push(ind);
            current_max = row[ind];
        }
    }
    return visible_inds;
}
fn calc_visibility(grid: &Grid, direction: Direction) -> HashSet<Coord> {
    let n_cols = grid[0].len();
    let direction_coord_tx = match direction {
        Direction::Left | Direction::Right => |x| x,
        Direction::Top | Direction::Bottom => |(x, y)| (y, x),
    };

    let mut result = HashSet::new();
    for (row_ind, row) in grid.iter().enumerate() {
        let iter: Box<dyn Iterator<Item=usize>> = match direction {
            Direction::Left | Direction::Top => Box::new((0..n_cols).into_iter()),
            Direction::Right | Direction::Bottom => Box::new((0..n_cols).rev()),
        };
        let vis_inds = vis_row(&row, iter);
        for i in vis_inds {
            result.insert(direction_coord_tx((row_ind, i)));
        }
    }

    return result;
}

fn trees_viewable(row: &Vec<i8>, position: usize, direction: Direction) -> u32 {
    if position == 0 {
        return 0;
    }
    let pos_u32 = position as u32;
    let rowl_u32 = row.len() as u32;
    let house_val: i8 = row[position];
    if direction == Direction::Left || direction == Direction::Top {
        let x = row[0..position].iter().rev();
        let result = x.take_while(|v| *v < &house_val).count() as u32;

        if result < pos_u32 {
            return result + 1 ;
        } else {
            return result ;
        }
    } else {
        let x = row[position + 1..row.len()].iter();
        let result = x.take_while(|v| *v < &house_val).count() as u32;
        if result < (rowl_u32 - pos_u32 - 1) {
            return result + 1 ;
        } else {
            return result ;
        }
    }
}



fn scenic_score(grid: &Grid, t_grid: &Grid, coord: &Coord) -> u32 {
    let (r, c) = *coord;
    let left_score = trees_viewable(&grid[r], c, Direction::Left);
    let right_score = trees_viewable(&grid[r], c, Direction::Right);
    let top_score = trees_viewable(&t_grid[c], r, Direction::Top);
    let bottom_score = trees_viewable(&t_grid[c], r, Direction::Bottom);
    left_score * right_score * top_score * bottom_score
}

fn main() -> Result<(), std::io::Error> {
    let mut grid: Grid = vec![];
    read_input(&"../inputs/day-8-input.txt", &mut grid)?;
    let t_grid = utils::transpose(&grid);
    let from_left = calc_visibility(&grid, Direction::Left);
    let from_right = calc_visibility(&grid, Direction::Right);
    let from_top = calc_visibility(&t_grid, Direction::Top);
    let from_bottom = calc_visibility(&t_grid, Direction::Bottom);
    println!("Grid is {} by {}", grid.len(), grid[0].len());
    let visible_coords = utils::union(&vec![from_left, from_right, from_top, from_bottom]);

    println!("Total visible: {}", visible_coords.len());

    let xs = 0..grid.len();
    let ys = 0..grid[0].len();
    let coords = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));

    let max_scenic_score = coords.map(|coord| scenic_score(&grid, &t_grid, &coord)).max().unwrap();

    println!("Max scenic score: {:?}", max_scenic_score);

    Ok(())
}
