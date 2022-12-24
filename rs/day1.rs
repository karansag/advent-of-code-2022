use std::fs;
use std::str;

fn read_file(file_name: &str) -> Result<Vec<Option<i32>>, std::io::Error> {
    let contents = fs::read_to_string(file_name)?;
    let values = contents
        .split_terminator("\n")
        .map(|s| {
            if s.is_empty() {
                return None;
            } else {
                return Some(s.parse().unwrap());
            }
        })
        .collect();
    return Ok(values);
}

fn main() -> Result<(), std::io::Error> {
    let contents = read_file("../inputs/day-1-input.txt")?;
    let mut vec_of_vecs: Vec<i32> = contents
        .split(|elt| elt.is_none())
        .map(|elt| elt.iter().map(|e| e.unwrap()).sum::<i32>())
        .collect();
    vec_of_vecs.sort_by(|a, b| b.cmp(a));
    let result: Vec<&i32> = vec_of_vecs.iter().take(3).collect();
    println!("{:?}", result);
    Ok(())
}
