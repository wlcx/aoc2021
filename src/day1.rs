use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type StdError<T> = Result<T, Box<dyn Error>>;

fn main() -> StdError<()> {
    let fname = env::args().skip(1).next().expect("Gimme input file plz");
    let file = BufReader::new(File::open(fname)?);

    let input = file
        .lines() // Split the files into lines
        .map(|l| l.unwrap()) // Unwrap the result
        .map(|s| u16::from_str(&s)) // convert to integers
        .map(|l| l.unwrap()) // Unwrap the result
        .collect::<Vec<_>>(); // Pull it into a vec so we can use windows

    let part1 = input
        .windows(2) // Sliding windows of 2
        .filter(|s| s[1] > s[0]) // Filter for instances that are greater than the previous
        .count(); // Count them!

    let part2 = input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count();

    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
    Ok(())
}
