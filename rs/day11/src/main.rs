use std::collections::HashMap;

const num_monkeys: usize = 8;

fn main() -> Result<(), std::io::Error> {
    let mut monkey_items: [Vec<i32>; 8] = [
        vec![91, 54, 70, 61, 64, 64, 60, 85],
        vec![82],
        vec![84, 93, 70],
        vec![78, 56, 85, 93],
        vec![64, 57, 81, 95, 52, 71, 58],
        vec![58, 71, 96, 58, 68, 90],
        vec![56, 99, 89, 97, 81],
        vec![68, 72],
    ];
    let monkey_ops: [fn(i32) -> i32; 8] = [
        |x: i32| x * 13,
        |x: i32| x + 7,
        |x: i32| x + 2,
        |x: i32| x * 2,
        |x: i32| x * x,
        |x: i32| x + 6,
        |x: i32| x + 1,
        |x: i32| x + 8,
    ];
    let monkey_tests: [fn(i32) -> bool; 8] = [
        |x| x % 2 == 0,
        |x| x % 13 == 0,
        |x| x % 5 == 0,
        |x| x % 3 == 0,
        |x| x % 11 == 0,
        |x| x % 17 == 0,
        |x| x % 7 == 0,
        |x| x % 19 == 0,
    ];
    let monkey_throws: [fn(bool) -> i32; 8] = [
        |x| if x { 5 } else { 2 },
        |x| if x { 4 } else { 3 },
        |x| if x { 5 } else { 1 },
        |x| if x { 6 } else { 7 },
        |x| if x { 7 } else { 3 },
        |x| if x { 4 } else { 1 },
        |x| if x { 0 } else { 2 },
        |x| if x { 6 } else { 0 },
    ];

    let number_rounds = 20;
    let mut counts: [i32; 8] = [0; 8];
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

    Ok(())
}

fn run_round(
    monkey_items: &mut [Vec<i32>; 8],
    monkey_ops: &[fn(i32) -> i32; 8],
    monkey_tests: &[fn(i32) -> bool; 8],
    monkey_throws: &[fn(bool) -> i32; 8],
    counts: &mut [i32; 8],
) -> () {
    for i in 0..num_monkeys {
        // println!("checking monkey {}", i);
        // Each monkey inspects each item
        let mut items = &mut monkey_items[i];
        // println!("monkey {} queue: {:?}", i, items);
        let op = monkey_ops[i];
        let throws = monkey_throws[i];
        let test = monkey_tests[i];
        // Throw locations: value and new location
        let mut throw_locations: Vec<(i32, i32)> = vec![];
        for item in items.iter() {
            let item_after_inspection = op(*item) / 3;
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
//     items: Vec<i32>,
//     op: Fn<i32, i32>,
//     test: Fn<
// }
// const Monkeys = [

//     ];
