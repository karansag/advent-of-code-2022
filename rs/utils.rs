use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn split_slice<P>(items: &[P], ind: usize) -> (&[P], &[P]) {
    (&items[..ind], &items[ind..])

}

pub fn split_every(s: &str, i: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars()
        .enumerate()
        .filter(|(ind, x)| ind % (i + 1) != i)
        .map(|(ind, x)| x)
        .collect();
    chars
        .chunks(i)
        .map(|c| c.iter().collect::<String>())
        .collect()
}


pub fn transpose<T: Copy>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}


fn main() -> Result<(), std::io::Error> {
    Ok(())
}
