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

fn proc_inst(inst: &Instruction, head_pos: &Coord, tail_pos: &Coord, visited: &mut HashSet<Coord>) -> (Coord, Coord) {
    println!("Inst: {:?}", inst);
    // println!("head / tail start of inst: {:?}, {:?}", head_pos, tail_pos);
    let (dir, num) = inst;
    let mut new_head: Coord = *head_pos;
    let mut new_tail: Coord = *tail_pos;
    for i in 0..*num {
        // println!("Exeucting instruction..., {}", i);
        new_head = move_c(&new_head, dir);
        new_tail = calc_tail(&new_tail, &new_head);
        // println!("new tail {:?}", new_tail);
        visited.insert(new_tail);
        // println!("head / tail pos: {:?}, {:?}", new_head, new_tail);
    }
    // println!("{:?}", visited);
    (new_head, new_tail)
}
// Process each of the instructions
// To do this, track the position of the head after each instruction and the position of the tail
// This involves calculating the new position of the tail after the head movement
// After calculating the new coordinates, add the result to the set
fn process_instructions(insts: &Vec<Instruction>, visited: &mut HashSet<Coord>) -> () {
   let mut head_position: Coord = (0, 0);
    let mut tail_position: Coord = (0, 0);

    for inst in insts {
        (head_position, tail_position) = proc_inst(&inst, &head_position, &tail_position, visited);
    }
    ()
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-9-input.txt")?;
    let parsed_lines: Vec<Instruction> = contents.map(|l| l.unwrap()).map(parse_line).collect();
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert((0, 0));
    process_instructions(&parsed_lines, &mut visited);


    println!("Number of visited coordinates: {:?}", visited.len());
    // for x in visited {
    //     println!("{:?}", x);
    // }
    Ok(())
}
