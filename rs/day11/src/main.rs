use std::collections::HashMap;

const NUM_MONKEYS: usize = 8;

// Insight for part 2:
// Since we run for 10000 rounds, the value of the worry level
// quickly runs into overflow issues. But since all the tests are
// for divisibility for the primes 2 -> 19, we can take the modulo
// of the product of these primes and preserve the same divisibility
// across all the monkeys. Therefore, the tests will come out the same
const PRIME_PRODUCT: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

fn main() -> Result<(), std::io::Error> {
    let mut monkey_items: [Vec<u64>; 8] = [
        vec![91, 54, 70, 61, 64, 64, 60, 85],
        vec![82],
        vec![84, 93, 70],
        vec![78, 56, 85, 93],
        vec![64, 57, 81, 95, 52, 71, 58],
        vec![58, 71, 96, 58, 68, 90],
        vec![56, 99, 89, 97, 81],
        vec![68, 72],
    ];
    let monkey_ops: [fn(u64) -> u64; 8] = [
        |x| x * 13,
        |x| x + 7,
        |x| x + 2,
        |x| x * 2,
        |x| x * x,
        |x| x + 6,
        |x| x + 1,
        |x| x + 8,
    ];
    let monkey_tests: [fn(u64) -> bool; 8] = [
        |x| x % 2 == 0,
        |x| x % 13 == 0,
        |x| x % 5 == 0,
        |x| x % 3 == 0,
        |x| x % 11 == 0,
        |x| x % 17 == 0,
        |x| x % 7 == 0,
        |x| x % 19 == 0,
    ];
    let monkey_throws: [fn(bool) -> u64; 8] = [
        |x| if x { 5 } else { 2 },
        |x| if x { 4 } else { 3 },
        |x| if x { 5 } else { 1 },
        |x| if x { 6 } else { 7 },
        |x| if x { 7 } else { 3 },
        |x| if x { 4 } else { 1 },
        |x| if x { 0 } else { 2 },
        |x| if x { 6 } else { 0 },
    ];
    // Part 1: 20 rounds
    // Part 2: 10000 rounds
    // let number_rounds = 20;
    let number_rounds = 10000;
    let mut counts: [u64; 8] = [0; 8];
    for _ in 0..number_rounds {
        run_round(
            &mut monkey_items,
            &monkey_ops,
            &monkey_tests,
            &monkey_throws,
            &mut counts,
        );
    }
    println!(
        "After round {}, the monkeys are holding items in the following order:",
        number_rounds
    );
    for item in monkey_items {
        println!("{:?}", item);
    }

    println!("counts: {:?}", counts);
    counts.sort();
    println!("score: {:?}", counts[6] * counts[7]);


    Ok(())
}

fn run_round(
    monkey_items: &mut [Vec<u64>; 8],
    monkey_ops: &[fn(u64) -> u64; 8],
    monkey_tests: &[fn(u64) -> bool; 8],
    monkey_throws: &[fn(bool) -> u64; 8],
    counts: &mut [u64; 8],
) -> () {
    for i in 0..NUM_MONKEYS {
        // println!("checking monkey {}", i);
        // Each monkey inspects each item
        let items = &mut monkey_items[i];
        // println!("monkey {} queue: {:?}", i, items);
        let op = monkey_ops[i];
        let throws = monkey_throws[i];
        let test = monkey_tests[i];
        // Throw locations: value and new location
        let mut throw_locations: Vec<(u64, u64)> = vec![];
        for item in items.iter() {
            // Part 1: Divide by 3
            // Part 2: No longer need to divide by 3
            // let item_after_inspection = op(*item) / 3;
            let item_after_inspection = op(*item) % PRIME_PRODUCT;
            let test_result = test(item_after_inspection);
            let throw_location = throws(test_result);
            throw_locations.push((item_after_inspection, throw_location));
            // Increase the monkey's inspection count
            counts[i] += 1;
        }
        items.clear();
        // println!("Throw locations: {:?}", throw_locations);
        for (item, new_loc) in throw_locations.iter() {
            monkey_items[*new_loc as usize].push(*item);
            // println!("new {}: {:?}", *new_loc, monkey_items[*new_loc as usize]);
        }
    }
}

// struct Monkey {
//     items: Vec<u64>,
//     op: Fn<u64, u64>,
//     test: Fn<
// }
// const Monkeys = [

//     ];
