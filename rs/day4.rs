mod utils;

use std::collections::HashSet;
use std::str::FromStr;

fn overlap(p1: &[i32; 2], p2: &[i32; 2]) -> bool {
    p2[0] <= p1[1] && p2[1] >= p1[0]
}

fn pair_overlaps(p1: &[i32; 2], p2: &[i32; 2]) -> i32 {
    (overlap(p1, p2) || overlap(p2, p1)) as i32
}

fn contains(p1: &[i32; 2], p2: &[i32; 2]) -> bool {
    p1[0] <= p2[0] && p1[1] >= p2[1]
}

fn pair_contains(p1: &[i32; 2], p2: &[i32; 2]) -> i32 {
    (contains(p1, p2) || contains(p2, p1)) as i32
}

fn parse(s: String) -> Vec<[i32; 2]> {
    let mut str_pairs = s.split(",");
    let first = str_pairs.next().unwrap();
    let second = str_pairs.next().unwrap();
    let mut first_pair = first.split("-");
    let mut second_pair = second.split("-");
    let f1 = first_pair.next().unwrap();
    let f2 = first_pair.next().unwrap();
    let s1 = second_pair.next().unwrap();
    let s2 = second_pair.next().unwrap();
    vec![
        [i32::from_str(f1).unwrap(), i32::from_str(f2).unwrap()],
        [i32::from_str(s1).unwrap(), i32::from_str(s2).unwrap()],
    ]
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-4-input.txt")?;
    let total_overlap: [i32; 2] = contents
        .map(|p| p.unwrap())
        .map(|p| parse(p))
        .map(|pair| {
            [
                pair_contains(&pair[0], &pair[1]),
                pair_overlaps(&pair[0], &pair[1]),
            ]
        })
        .reduce(|acc, next| [acc[0] + next[0], acc[1] + next[1]])
        .unwrap();

    println!("total overlap: {}", total_overlap[0]);
    println!("any overlap: {}", total_overlap[1]);
    Ok(())
}
