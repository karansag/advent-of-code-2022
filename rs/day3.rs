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
    let (first_half, second_half) = (&line[0..length / 2], &line[length / 2..length]);
    let fh_set: HashSet<char> = first_half.chars().collect();
    let sh_set: HashSet<char> = second_half.chars().collect();
    let intersection_value_ch = fh_set.intersection(&sh_set).next().unwrap();
    get_priority(intersection_value_ch)
}

fn get_badge_total(v: &Vec<String>) -> u32 {
    let i = v.chunks(3);
    let mut sum: u32 = 0;
    for chunk in i {
        let common_value_set = chunk
            .iter()
            .map(|c: &String| -> HashSet<char> { c.chars().into_iter().collect() });
        let common_values = common_value_set
            .reduce(|x: HashSet<char>, y: HashSet<char>| -> HashSet<char> {
                x.intersection(&y).cloned().collect()
            })
            .unwrap();
        sum += get_priority(common_values.iter().next().unwrap());
    }
    return sum;
}

fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-3-input.txt")?;
    let lines: Vec<String> = contents.map(|p| p.unwrap()).collect();
    let total: u32 = lines.iter().map(handle_line).sum();
    println!("total: {:?}", total);
    let badge_total: u32 = get_badge_total(&lines);
    println!("badge_total: {:?}", badge_total);
    Ok(())
}
