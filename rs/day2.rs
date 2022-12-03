mod utils;
use std::str;
use std::collections::HashMap;


fn compare(a: &str, b: &str) -> i32 {
    let comparison = vec!["A", "B", "C"];
    // pos_a = comparison.position()
    return match (a, b) {
        (x, y) if x == y => 0,
        ("A", "B") => -1,
        ("A", "C") => 1,
        ("B", "C") => -1,
        ("B", "A") => 1,
        ("C", "A") => -1,
        ("C", "B") => 1,
        (&_, _) => todo!()
    }
}


fn handle_pair(mut p: std::str::Split<&str>) -> i32 {
    let conversion: HashMap<&str, &str> = [
        ("X", "A"), ("Y", "B"), ("Z", "C")
    ].iter().cloned().collect();
    let pair = (p.next().unwrap(), conversion.get(p.next().unwrap()).unwrap());
    let my_choice_score = match pair.1 {
        &"A" => 1,
        &"B" => 2,
        &"C" => 3,
        _  => todo!()
    };
    let result_score = (compare(pair.1, pair.0) + 1) * 3;
    return my_choice_score + result_score;

}


fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-2-input.txt")?;
    let total: i32 = contents
        .map(|p| handle_pair(p.unwrap().split(" "))).sum();
    println!("total: {:?}", total);
    Ok(())
}
