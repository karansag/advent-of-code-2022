mod utils;

use std::collections::HashSet;


fn get_priority(c: &char) -> u32 {
    let a_value = u32::from('a');
    let mut inters_value = u32::from(c.to_ascii_lowercase()) - a_value + 1;
    if c.is_ascii_uppercase() {
        inters_value += 26;
    }
    inters_value
}

fn handle_line(line: &String) -> u32 {
    let length = line.len();
    let (first_half, second_half) = (&line[0..length / 2], &line[length / 2 .. length]);
    let fh_set: HashSet<char> = first_half.chars().collect();
    let sh_set: HashSet<char> = second_half.chars().collect();
    let intersection_value_ch = fh_set.intersection(&sh_set).next().unwrap();
    get_priority(intersection_value_ch)
    // println!("{}", intersection_value_ch);
    // println!("{}", inters_value);
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-3-input.txt")?;
    let lines: Vec<String> = contents
        .map(|p| p.unwrap()).collect();
    let total: u32 = lines.iter().map(handle_line).sum();
    println!("total: {:?}", total);
    // let badge_total: u32 = get_badge_total(&lines);
    // println!("total: {:?}", total);
    Ok(())
}
