mod utils;

use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Inst {
    Noop,
    Addx {val: i32},
}

fn parse_line(s: String) -> Inst {
    let splits: Vec<&str> = s.split_whitespace().collect();
    match splits[0] {
        x if x == "noop" => Inst::Noop,
        x if x == "addx" => Inst::Addx { val: i32::from_str(splits[1]).unwrap()},
        &_ => todo!(),
    }
}

fn process_insts(insts: &Vec<Inst>, reg_start: i32) -> i32 {
    let mut accum = 0;
    let mut cycle = 1;
    let mut reg = reg_start;
    for inst in insts {
        let cymod = cycle % 40;
        if cymod == 20 {
            println!("Value at cycle {:?} is {:?}", cycle, reg);
            accum += reg * cycle;
        }
        if *inst == Inst::Noop {
            cycle += 1;
        }
        if let Inst::Addx {val} = *inst {
            if cymod == 19  {
                println!("Value at cycle {:?} (addx) is {:?}", cycle + 1, reg);
                accum += (cycle + 1) * reg;
            }
            cycle += 2;
            reg += val;
        }
    }
    accum
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-10-input.txt")?;
    let insts: Vec<Inst> = contents.map(|l| l.unwrap()).map(parse_line).collect();

    let acc = process_insts(&insts, 1);
    println!("Signal accumulated: {:?}", acc);
    Ok(())
}
