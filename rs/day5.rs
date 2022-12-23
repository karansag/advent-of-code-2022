mod utils;

use std::str::FromStr;
use std::vec::Drain;

#[derive(Debug)]
struct Stack {
    st: Vec<char>,
}

impl Stack {
    fn pop(&mut self) -> Option<char> {
        self.st.pop()
    }
    fn push(&mut self, item: char) -> () {
        self.st.push(item)
    }
    fn len(&self) -> usize {
        self.st.len()
    }
    fn remove(&mut self, n: usize) -> Drain<char> {
        let length = self.len();
        self.st.drain(length - n..)
    }
}

struct Instruction {
    from: usize,
    to: usize,
    quantity: u16,
}

fn parse_stacks(items: &[String]) -> Vec<Stack> {
    fn _tokenize_row(row_string: &String) -> Vec<Option<char>> {
        let values = utils::split_every(row_string, 3);
        values.iter().map(|v| {
            if v.starts_with('[') {
                return Some(v.chars().nth(1).unwrap());
            } else {
                return None;
            }
        }).collect()
    }
    let raw_tokens = items.iter().map(_tokenize_row).collect();
    let tokens = utils::transpose(&raw_tokens);
    let mut r: Vec<Stack> = Vec::with_capacity(tokens.len());
    // let row_length = tokens[0].len();
    for row in tokens {
        let mut new_stack = Stack { st: vec![] };
        for item in row.iter().rev() {
            if let Some(character) = item {
                new_stack.push(*character)
            }
        }
        r.push(new_stack);
    }
    r
}

fn parse_stack_section(stack_text: &Vec<String>) -> Vec<Stack> {
    let (all_but_last, last_line_vec) = utils::split_slice(&stack_text, stack_text.len() - 1);
    let count = last_line_vec[0]
        .split_whitespace()
        .map(u8::from_str)
        .count();
    let items = parse_stacks(&all_but_last);
    items
}

fn parse_instructions(inst_text: &Vec<String>) -> Vec<Instruction> {
    fn parse_inst(t: &String) -> Instruction {
        let words: Vec<&str> = t.split_whitespace().collect();
        Instruction {
            quantity: u16::from_str(words[1]).unwrap(),
            from: usize::from_str(words[3]).unwrap(),
            to: usize::from_str(words[5]).unwrap(),
        }
    }
    inst_text.iter().map(parse_inst).collect()
}

fn push_items(removed: &[char], to_stack: &mut Stack) -> () {
    for item in removed {
        to_stack.push(*item);
    }
}

fn apply_instructions(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>, keep_order: bool) -> () {
    for inst in instructions {
        let from_ind = inst.from - 1;
        let to_ind = inst.to - 1;
        if keep_order {
            let from_stack = &mut stacks[from_ind];
            let removed: Vec<char> = from_stack.remove(inst.quantity as usize).collect();
            push_items(&removed, &mut stacks[to_ind]);
        } else {
            for _ in 0..inst.quantity {
                let ch = stacks[from_ind].pop().unwrap();
                stacks[to_ind].push(ch);
            }
        }
    }
    ()
}

fn parse_data(s: &str) -> Result<(Vec<Stack>, Vec<Instruction>), std::io::Error> {
    let mut file_contents = utils::read_file(s)?;
    let stack_text = file_contents
        .by_ref()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    let instruction_text = file_contents.map(|l| l.unwrap()).collect();
    let stacks: Vec<Stack> = parse_stack_section(&stack_text);
    let insts: Vec<Instruction> = parse_instructions(&instruction_text);

    Ok((stacks, insts))
}

fn main() -> Result<(), std::io::Error> {
    let (mut stacks, insts) = parse_data("../inputs/day-5-input.txt")?;
    apply_instructions(&mut stacks, &insts, true);
    for s in stacks {
        println!("{:?}", s);
    }
    // execute_instructions(stacks, instructions);
    // let top_items: Vec<String> = read_top_items(stacks);
    Ok(())
}
