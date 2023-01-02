mod utils;

use core::ops::RangeInclusive;
use lazy_static::lazy_static;
use range_union_find::IntRangeUnionFind;
use regex::Regex;
use std::collections::HashSet;

type Coord = (i32, i32);

#[derive(PartialEq, Debug, Clone)]
struct Signal(Coord);
#[derive(PartialEq, Debug, Clone, Hash, Eq)]
struct Beacon(Coord);

struct SigPair {
    signal: Signal,
    beacon: Beacon,
}

fn parse_line(input: String) -> Option<SigPair> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();
    }
    // Compile the regular expression

    // Extract the x and y values from the input string
    let captures = RE.captures(&input)?;
    let x1 = captures[1].parse::<i32>().ok()?;
    let y1 = captures[2].parse::<i32>().ok()?;
    let x2 = captures[3].parse::<i32>().ok()?;
    let y2 = captures[4].parse::<i32>().ok()?;

    Some(SigPair {
        signal: Signal((x1, y1)),
        beacon: Beacon((x2, y2)),
    })
}

fn mh_dist(x: &Coord, y: &Coord) -> i32 {
    let (x1, y1) = *x;
    let (x2, y2) = *y;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() -> Result<(), std::io::Error> {
    // let y_star = 10;
    let y_star = 2000000;
    // let file_path = &"../../inputs/day-15-ex.txt";
    let file_path = &"../../inputs/day-15-input.txt";
    let contents = utils::read_file(file_path)?;
    let signal_beacon_pairs: Vec<SigPair> = contents
        .map(|l| l.unwrap())
        .map(parse_line)
        .filter_map(|x| x)
        .collect();

    let distances: Vec<i32> = signal_beacon_pairs
        .iter()
        .map(|sp: &SigPair| mh_dist(&sp.signal.0, &sp.beacon.0))
        .collect();

    let mut range_union = IntRangeUnionFind::<i32>::new();

    signal_beacon_pairs
        .iter()
        .map(|sp: &SigPair| &sp.signal)
        .zip(distances)
        .map(|(s, bc_d): (&Signal, i32)| {
            let x = s.0 .0 as i32;
            let y = s.0 .1 as i32;
            let y_dist = (y - y_star).abs() as i32;
            let bc_dist = bc_d as i32;
            if y_dist <= bc_dist {
                (x - bc_dist + y_dist)..=(x + bc_dist - y_dist)
            } else {
                -1..=0
            }
        })
        .for_each(|r| {
            range_union.insert_range(&r);
        });

    let y_star_beacons: Vec<i32> = signal_beacon_pairs
        .iter()
        .map(|sp| &sp.beacon)
        .filter(|b| b.0 .1 == y_star.into())
        .collect::<HashSet<&Beacon>>()
        .iter()
        .map(|b| b.0 .0)
        .collect();

    let sum: usize = range_union
        .to_collection::<Vec<RangeInclusive<i32>>>()
        .iter()
        .cloned()
        .map(|r: RangeInclusive<i32>| {
            let excluded_beacons = y_star_beacons
                .iter()
                .clone()
                .filter(|beacon_coord| r.contains(&beacon_coord))
                .count();
            let mut counter = 0;
            for _ in r {
                counter += 1;
            }
            counter - excluded_beacons
        })
        .sum();

    println!("sum: {}", sum);

    Ok(())
}
