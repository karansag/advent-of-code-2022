use std::fs;
use std::str;

fn read_file(file_name: &str) -> Result<Vec<Option<i32>>, std::io::Error> {
    let contents = fs::read_to_string(file_name)?;
    let values = contents.split_terminator("\n").map(|s| {
        if s.is_empty() {
            return None;
        } else {
            return Some(s.parse().unwrap());
        }
    }).collect();
    return Ok(values);
}

fn main() -> Result<(), std::io::Error> {
    let contents = read_file("../inputs/day-1-input.txt")?;
    let vec_of_vecs = contents.split(|elt| elt.is_none()).map(|elt| elt.iter().map(|e| e.unwrap()).sum::<i32>());
    println!("{}", vec_of_vecs.max().unwrap());
    Ok(())
}
