use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;



type Map = Vec<Vec<char>>;
type Coord = (usize, usize);

fn get_first_coordinate(map: &Map, ch: &char) -> Option<Coord> {
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if val == ch {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_all_coords(map: &Map, ch: &char) -> Vec<Coord> {
    let mut ret = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if val == ch {
                ret.push((i, j));
            }
        }
    }
    ret
}
fn char_to_height(ch: char) -> i32 {
    let elev = match ch {
        'S' => 'a',
        'E' => 'z',
        x => x,
    };
    elev as i32
}

fn find_path(map: &Map, start: &Coord, end: &Coord) -> Option<Vec<Coord>> {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let get_neighbors = |coord: &Coord|  -> Vec<Coord> {
        let c = (coord.0 as i32, coord.1 as i32);
        let coord_height = char_to_height(map[coord.0][coord.1]);
        let cartesian_product = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        cartesian_product
            .iter()
            .map(|(xd, yd)| (c.0 + xd, c.1 + yd))
            // Bounds of map
            .filter(|(x, y)| -1 < *x  && *x < rows && -1 < *y && *y < cols)
            // Allowed to ascend based on height
            .filter(|(x, y)| {
                let (x_coord, y_coord) = (*x as usize, *y as usize);
                let height = char_to_height(map[x_coord][y_coord]);
                height as i32 <= coord_height + 1
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    };
    let mut queue: VecDeque<Coord> = VecDeque::new();
    queue.push_back(*start);
    let mut previous: HashMap<Coord, Coord> = HashMap::new();
    let mut found = false;

    while !queue.is_empty() && !found {
        let next = queue.pop_front().unwrap();
        let neighbors_raw: Vec<Coord> = get_neighbors(&next);
        let neighbors: Vec<&Coord> = neighbors_raw
            .iter()
            .filter(|x| **x != *start && !previous.contains_key(*x))
            .collect();


        for coord in neighbors {
            let c: Coord = *coord;
            previous.insert(c, next);
            if c == *end {
                found = true;
            } else {
                queue.push_back(c);
            }
        }
    }
    if !found {
        return None
    }
    let mut ret: Vec<Coord> = vec![*end];
    let mut curr = end;
    while let Some(c) = previous.get(curr) {
        ret.push(*c);
        curr = c;
    }

    Some(ret)
}

fn main() -> Result<(), std::io::Error> {
    let file_contents = fs::read_to_string("../inputs/day-12-input.txt")?;
    let map: Vec<Vec<_>> = file_contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .collect()
        })
        .collect();
    let start_coordinate = get_first_coordinate(&map, &'S').unwrap();
    let end_coordinate = get_first_coordinate(&map, &'E').unwrap();

    let path = find_path(&map, &start_coordinate, &end_coordinate).unwrap();
    println!("path length (not including end): {:?}", path.len() - 1);

    let coords_with_a = get_all_coords(&map, &'a');
    println!("{:?}", coords_with_a.len());
    let min_val = coords_with_a.iter().map(|c| find_path(&map, c, &end_coordinate))
        .filter(|x| *x != None)
        .map(|x| match x {
            Some(p) => p.len() - 1,
            _ => 100000,
        })
        .min();

    // println!("{:?}", vals);
    println!("min: ${:?}", min_val);


    Ok(())
}
