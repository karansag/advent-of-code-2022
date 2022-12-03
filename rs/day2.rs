mod utils;
use std::str;


fn compare(a: &str, b: &str) -> i32 {
    // let comparison = vec!["A", "B", "C"];
    // pos_a = comparison.position()
    match (a, b) {
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

fn get_my_choice<'a>(opponent_choice: &'a str, desired_outcome: &'a str) -> &'a str {
    let comparison = ["A", "B", "C"];
    let pos_opponent = comparison.iter().position(|&x| x == opponent_choice).unwrap();
    match (opponent_choice, desired_outcome) {
        (_, "X") => comparison[(pos_opponent + 2) % 3],
        (x, "Y") => x,
        (_, "Z") => comparison[(pos_opponent + 1) % 3],
            _ => todo!()
    }
}

fn handle_pair(mut p: std::str::Split<&str>) -> i32 {
    // Part 1 commented out
    // let conversion: HashMap<&str, &str> = [
    //     ("X", "A"), ("Y", "B"), ("Z", "C")
    // ].iter().cloned().collect();
    // let pair = (p.next().unwrap(), conversion.get(p.next().unwrap()).unwrap());
    // Part 2
    let pair = (p.next().unwrap(), p.next().unwrap());
    let my_choice = get_my_choice(pair.0, pair.1);
    let my_choice_score = match my_choice {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _  => todo!()
    };
    let result_score = (compare(my_choice, pair.0) + 1) * 3;
    my_choice_score + result_score

}


fn main() -> Result<(), std::io::Error> {
    let contents = utils::read_file("../inputs/day-2-input.txt")?;
    let total: i32 = contents
        // .take(10)
        .map(|p| handle_pair(p.unwrap().split(" "))).sum();
    println!("total: {:?}", total);
    Ok(())
}
