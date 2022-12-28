use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Deserialize, Debug, Clone, Eq, Hash, Ord)]
#[serde(untagged)]
enum Value {
    Vec(Vec<Value>),
    Int(i32),
}

fn eval(s: &str) -> Value {
    let value: Value = serde_json::from_str(s).unwrap();
    match value {
        Value::Vec(vec) => Value::Vec(vec),
        _ => panic!("Expected a vector"),
    }
}

fn read_pairs(path: &str) -> Vec<Vec<String>> {
    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();
    let mut current_pair = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            pairs.push(current_pair);
            current_pair = Vec::new();
        } else {
            current_pair.push(line);
        }
    }
    if !current_pair.is_empty() {
        pairs.push(current_pair);
    }
    pairs
}

// Split a slice at index `ind` into two slices, one before `ind` and one after
pub fn split_slice<P>(items: &[P], ind: usize) -> (&[P], &[P]) {
    (&items[..ind], &items[ind..])
}

pub fn head_tail<P>(items: &[P]) -> (&P, &[P]) {
    (&items[0], &items[1..])
}

// Returns true iff left <= right
fn order(left: &Value, right: &Value) -> Ordering {
    // println!("Comparing {:?} and {:?}", left, right);
    if let Value::Int(left_int) = left {
        return match right {
            Value::Int(right_int) => left_int.cmp(right_int),
            Value::Vec(v) => order(
                &Value::Vec(vec![Value::Int(*left_int)]),
                &Value::Vec(v.to_vec()),
            ),
        };
    }
    {
        if let Value::Int(right_int) = right {
            return order(&left, &Value::Vec(vec![Value::Int(*right_int)]));
        }
    }
    if let Value::Vec(left_v) = left {
        if let Value::Vec(right_v) = right {
            if left_v.is_empty() {
                if right_v.is_empty() {
                    return Ordering::Equal;
                }
                return Ordering::Less;
            } else if right_v.is_empty() {
                return Ordering::Greater;
            }
            let (l_head, l_tail) = head_tail(&left_v);
            let (r_head, r_tail) = head_tail(&right_v);
            return match order(l_head, r_head) {
                Ordering::Equal => {
                    order(&Value::Vec(l_tail.to_vec()), &Value::Vec(r_tail.to_vec()))
                }
                x => x,
            };
        };
    };
    todo!();
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        order(&self, other) == Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(order(&self, other))
    }
}

fn main() -> Result<(), std::io::Error> {
    let pairs_str = read_pairs("../../inputs/day-13-input.txt");

    let mut packets: Vec<Value> = pairs_str
        .iter()
        .map(|p| (eval(&p[0]), eval(&p[1])))
        .map(|pair| {
            if order(&pair.0, &pair.1) == Ordering::Greater {
                (pair.1, pair.0)
            } else {
                pair
            }
        })
        .flat_map(|pair| vec![pair.0, pair.1])
        .collect();
    let divider_packets = vec![
        Value::Vec(vec![Value::Vec(vec![Value::Int(2)])]),
        Value::Vec(vec![Value::Vec(vec![Value::Int(6)])]),
    ];
    let divider_packets_2: HashSet<Value> = vec![
        Value::Vec(vec![Value::Vec(vec![Value::Int(2)])]),
        Value::Vec(vec![Value::Vec(vec![Value::Int(6)])]),
    ]
    .into_iter()
    .collect();
    packets.extend(divider_packets);
    packets.sort();

    let product: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets_2.contains(&packet))
        .map(|(i, _)| i + 1)
        .product();

    println!("product: {:?}", product);

    Ok(())
}
