mod utils;

use std::fs;
use std::collections::HashSet;

fn find_signal_start(signal: &str) -> Option<usize> {
    let enumerated_chars: Vec<(usize, char)> = signal
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>();

    let matching: Option<&[(usize, char)]> = enumerated_chars
        .windows(4)
        .find(|pairs| pairs.iter().map(|p| p.1).collect::<HashSet<char>>().len() == 4);
    match matching {
        Some(slice) => Some(slice[0].0 + 3 + 1), // Add 3 to shift to end of sequence, 1 for 1-indexing
        None => None
    }
}

fn main() -> Result<(), std::io::Error> {
    let file_path = &"../inputs/day-6-input.txt";
    let contents = fs::read_to_string(file_path)?;
    let signal_start_ind = find_signal_start(&contents).unwrap();
    println!("{}", signal_start_ind);
    Ok(())
}
