mod utils;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::str::FromStr;
use utils::Coord;

#[derive(Debug, Clone)]
struct Rock {
    points: Vec<Coord>,
}

#[derive(Debug, Clone, PartialEq)]
enum GridState {
    Rock,
    Sand,
}

type Grid = HashMap<Coord, GridState>;

fn parse_rock(l: String) -> Rock {
    fn make_point(s: &str) -> Coord {
        let pairs: Vec<&str> = s.split(',').collect();
        (
            usize::from_str(pairs[0]).unwrap(),
            usize::from_str(pairs[1]).unwrap(),
        )
    }
    let cleaned_pairs = l.split("->").map(|s| s.trim());
    let points: Vec<Coord> = cleaned_pairs.map(make_point).collect();
    Rock { points: points }
}

fn invert_coord(c: Coord) -> Coord {
    (c.1, c.0)
}

fn interpolate<'a>(c1: &'a Coord, c2: &'a Coord) -> Box<dyn Iterator<Item = Coord>> {
    let cp1 = *c1;
    let cp2 = *c2;
    if cp1.0 == cp2.0 {
        let r = if cp1.1 < cp2.1 {
            cp1.1..=cp2.1
        } else {
            cp2.1..=cp1.1
        };
        Box::new(r.map(move |i| (cp1.0, i)))
    } else if cp1.1 == cp2.1 {
        Box::new(interpolate(&invert_coord(cp1), &invert_coord(cp2)).map(invert_coord))
    } else {
        panic!("{:?} and {:?} cannto be interpolated.", cp1, cp2);
    }
}

fn fill_grid(rock_data: Vec<Rock>, g: &mut Grid) -> () {
    // let mut g: Grid = HashMap::new();
    for rock in rock_data {
        let points = rock.points;
        let pairs = points.iter().zip(points.iter().skip(1)).collect::<Vec<_>>();
        // let pairs = make_pairs(points);
        pairs.iter().for_each(|(p1, p2)| {
            let interpolated_pts = interpolate(&p1, &p2);
            for pt in interpolated_pts {
                (*g).insert(pt, GridState::Rock);
            }
        })
    }
}
fn group_by_column(g: &Grid, by_column: &mut HashMap<usize, Vec<Coord>>) -> () {
    for (coord, _) in g {
        let v = match by_column.entry(coord.0) {
            Vacant(entry) => entry.insert(Vec::new()),
            Occupied(entry) => entry.into_mut(),
        };
        v.push(*coord);
    }
}

fn drop_grain(grid: &Grid, by_column: &HashMap<usize, Vec<Coord>>, start: Coord) -> Option<Coord> {
    // Check if sand will go into the void
    if let Some(v) = by_column.get(&start.0) {
        let max = v.iter().map(|(_, y)| y).max();
        if let Some(m) = max {
            if m < &start.1 {
                return None;
            }
        }
    } else {
        return None;
    }
    let cur_coord = start;
    let below_coord = (cur_coord.0, cur_coord.1 + 1);
    let below_lcoord = (cur_coord.0 - 1, cur_coord.1 + 1);
    let below_rcoord = (cur_coord.0 + 1, cur_coord.1 + 1);

    if grid.get(&below_coord) == None {
        drop_grain(grid, by_column, below_coord)
    } else if grid.get(&below_lcoord) == None {
        drop_grain(grid, by_column, below_lcoord)
    } else if grid.get(&below_rcoord) == None {
        drop_grain(grid, by_column, below_rcoord)
    } else {
        Some(cur_coord)
    }
}

fn drop_sand(g: &mut Grid, by_column: &mut HashMap<usize, Vec<Coord>>, start: Coord) -> i32 {
    let mut grains = 0;
    let mut cont = true;

    while cont {
        let rest_spot = drop_grain(&g, &by_column, start);
        if let Some(coord) = rest_spot {
            g.insert(coord, GridState::Sand);
            let v = match by_column.entry(coord.0) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            v.push(coord);
            grains += 1;
        } else {
            cont = false;
        }
    }

    grains
}

fn main() -> Result<(), std::io::Error> {
    let lines = utils::read_file("../inputs/day-14-input.txt")?;
    let data: Vec<Rock> = lines.map(|l| l.unwrap()).map(parse_rock).collect();

    let mut grid: Grid = HashMap::new();
    let mut by_column: HashMap<usize, Vec<Coord>> = HashMap::new();
    fill_grid(data, &mut grid);
    println!("{:?}", grid.get(&(487, 90)));
    let grid_ref = &grid;
    group_by_column(grid_ref, &mut by_column);
    println!("{:?}", by_column.get(&487));

    let droplets = drop_sand(&mut grid, &mut by_column, (500, 0));
    println!("Number of sand drops: {:?}", droplets);

    Ok(())
}
