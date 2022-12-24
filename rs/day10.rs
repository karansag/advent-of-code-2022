mod utils;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Inst {
    Noop,
    Addx { val: i32 },
}

fn parse_line(s: String) -> Inst {
    let splits: Vec<&str> = s.split_whitespace().collect();
    match splits[0] {
        x if x == "noop" => Inst::Noop,
        x if x == "addx" => Inst::Addx {
            val: i32::from_str(splits[1]).unwrap(),
        },
        &_ => todo!(),
    }
}

fn process_insts<'a>(
    insts: &Vec<Inst>,
    reg_start: i32,
    cycle_map: &'a mut HashMap<i32, i32>,
) -> (i32, &'a HashMap<i32, i32>) {
    let mut accum = 0;
    let mut cycle = 1;
    let mut reg = reg_start;
    cycle_map.insert(1, reg_start);
    for inst in insts {
        cycle_map.insert(cycle, reg);
        let cymod = cycle % 40;
        if cymod == 20 {
            println!("Value at cycle {:?} is {:?}", cycle, reg);
            accum += reg * cycle;
        }
        if *inst == Inst::Noop {
            cycle += 1;
        }
        if let Inst::Addx { val } = *inst {
            if cymod == 19 {
                println!("Value at cycle {:?} (addx) is {:?}", cycle + 1, reg);
                accum += (cycle + 1) * reg;
            }
            cycle_map.insert(cycle + 1, reg);
            cycle += 2;
            reg += val;
        }
    }
    (accum, cycle_map)
}
fn draw_screen(values: &Vec<bool>, width: i32) -> () {
    let chars: Vec<char> = values
        .iter()
        .map(|x| match x {
            true => '#',
            false => '.',
        })
        .collect();
    let results: Vec<String> = chars
        .chunks(width as usize)
        .map(|c| c.iter().collect::<String>())
        .collect();

    for line in results {
        println!("{}", line);
    }
}

fn calculate_pixels(width: i32, height: i32, pos_map: &HashMap<i32, i32>) -> Vec<bool> {
    let mut ret = vec![false; (width * height) as usize];

    for pix in 0..ret.len() {
        let p = pix as i32;
        let sprite_center = pos_map[&(p + 1)];
        if ((p % width) - sprite_center).abs() < 2 {
            ret[pix] = true;
        }
    }
    ret
}
fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-10-input.txt")?;
    let insts: Vec<Inst> = contents.map(|l| l.unwrap()).map(parse_line).collect();

    let mut sprite_pos_map: HashMap<i32, i32> = HashMap::new();
    let (acc, _) = process_insts(&insts, 1, &mut sprite_pos_map);
    println!("Signal accumulated: {:?}", acc);
    let results: Vec<bool> = calculate_pixels(40, 6, &sprite_pos_map);
    draw_screen(&results, 40);
    Ok(())
}
