mod utils;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
struct Valve {
    name: String,
    neighbors: Vec<String>,
    flow_rate: u32,
    // neighbors_ref: &'static mut Vec<&'static Valve>,
}

fn parse_line(s: String) -> Option<Valve> {
    //example: Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    println!("{:?}", s);
    let re: Regex =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels lead to valves (.*)").unwrap();
    let captures = re.captures(&s)?;

    let name = captures[1].parse::<String>().ok()?;
    let flow_rate = captures[2].parse::<u32>().ok()?;
    let neighbors: Vec<String> = captures[3]
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();
    Some(Valve {
        name,
        flow_rate,
        neighbors,
    })
}

fn parse_file(file_path: &str) -> Option<Vec<Valve>> {
    println!("here");
    let contents = utils::read_file(file_path).ok()?;
    println!("h1ere");
    let data: Vec<Valve> = contents
        .map(|l| l.unwrap())
        .map(parse_line)
        .filter_map(|x| x)
        .collect();
    // for valve in &data {
    //     for neighbor_name in valve.neighbors {
    //         let n_ref = data.iter().find(|v| v.name == neighbor_name).unwrap();
    //         valve.neighbors_ref.push(n_ref);
    //     }
    // }
    Some(data)
}

fn main() -> Result<(), std::io::Error> {
    let file_path = &"../../inputs/day-16-input.txt";
    let data = parse_file(file_path).unwrap();
    println!("{:?}", data);

    Ok(())
}
