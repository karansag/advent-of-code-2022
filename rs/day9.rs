mod utils;

use utils::Direction;
use std::str::FromStr;
use std::collections::HashSet;

type Instruction = (Direction, i32);
type Coord = (i32, i32);

fn parse_line(s: String) -> Instruction {
    let data: Vec<&str> = s.split_whitespace().collect();
    let d = match data[0] {
        "U" => Direction::Top,
        "D" => Direction::Bottom,
        "L" => Direction::Left,
        "R" => Direction::Right,
        &_ => todo!(),
    };
    let num = i32::from_str(data[1]).unwrap();
    (d, num)

}

fn move_c(s: &Coord, dir: &Direction) -> Coord {
    let (x, y): Coord = *s;
    match dir {
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Top => (x, y + 1),
        Direction::Bottom => (x, y - 1),
    }
}

fn mh_dist(x: &Coord, y: &Coord) -> i32 {
    let (x1, y1) = *x;
    let (x2, y2) = *y;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn calc_tail(tail_pos: &Coord, head_pos: &Coord) -> Coord {
    // println!("calculating with head/ tail {:?}, {:?}", head_pos, tail_pos);
    let (head_x, head_y) = *head_pos;
    let (tail_x, tail_y) = *tail_pos;
    if head_x == tail_x {
        return match head_y - tail_y {
            diff if diff > 1 => (tail_x, head_y - 1),
            diff if diff < -1 => (tail_x, head_y + 1),
            _ => (tail_x, tail_y)
        };
    } else if head_y == tail_y {
        return match head_x - tail_x {
            diff if diff > 1 => (head_x - 1, head_y),
            diff if diff < -1 => (head_x + 1, head_y),
            _ => (tail_x, tail_y)
        };
    } else if mh_dist(head_pos, tail_pos) > 2 {
        return match (head_x - tail_x, head_y - tail_y) {
            (x_diff, y_diff) if x_diff > 0 && y_diff > 0 => (tail_x + 1, tail_y + 1),
            (x_diff, y_diff) if x_diff < 0 && y_diff < 0 => (tail_x - 1, tail_y - 1),
            (x_diff, y_diff) if x_diff > 0 && y_diff < 0 => (tail_x + 1, tail_y - 1),
            _ => (tail_x - 1, tail_y + 1),
        };
    } else {
       return *tail_pos;
    }

}

// Plan: get the current positions.
// At each step --

// Move the head, store that.
// Move each of the rest, store that.
// Make the stored results the new positions
fn proc_inst<'a>(inst: &'a Instruction, positions: &'a Vec<Coord>, visited: &'a mut HashSet<Coord>) -> Vec<Coord> {
    // println!("Inst: {:?}", inst);
    let (dir, num) = inst;
    let mut new_positions: Vec<Coord> = vec![];
    let mut positions_tmp: Vec<Coord> = positions.iter().cloned().collect();
    let mut tmp_tail: Coord = (0, 0);
    for _ in 0..*num {
        new_positions.push(move_c(&positions_tmp[0], dir));
        positions_tmp[1..].iter().cloned().fold(new_positions[0], |prev, next| {
            tmp_tail = calc_tail(&next, &prev);
            new_positions.push(tmp_tail);
            tmp_tail
        });
        visited.insert(tmp_tail);
        positions_tmp = new_positions;
        new_positions = vec![];
    }
    positions_tmp
}

// Process each of the instructions
// To do this, track the position of the head after each instruction and the position of the tail
// This involves calculating the new position of the tail after the head movement
// After calculating the new coordinates, add the result to the set
fn process_instructions(insts: &Vec<Instruction>, visited: &mut HashSet<Coord>, num_knots: i32) -> () {
    let mut positions: Vec<Coord> = vec![(0, 0); num_knots as usize];
    for inst in insts {
        positions = proc_inst(&inst, &positions, visited);
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-9-input.txt")?;
    let parsed_lines: Vec<Instruction> = contents.map(|l| l.unwrap()).map(parse_line).collect();
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert((0, 0));
    process_instructions(&parsed_lines, &mut visited, 2);


    println!("Number of visited coordinates with two knots: {:?}", visited.len());

    let mut visited_part2: HashSet<Coord> = HashSet::new();
    visited_part2.insert((0, 0));
    process_instructions(&parsed_lines, &mut visited_part2, 10);
    println!("Number of visited coordinates with ten knots: {:?}", visited_part2.len());
    Ok(())
}
