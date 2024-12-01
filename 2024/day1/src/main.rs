use std::{collections::HashMap, fs};

fn main() {
    let (l, r): (Vec<usize>, Vec<usize>) =
        fs::read_to_string("/home/aidan/Projects/advent_of_code/2024/day1/src/input")
            .unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_once(" ").unwrap())
            .map(|(l, r)| {
                (
                    l.trim().parse::<usize>().unwrap(),
                    r.trim().parse::<usize>().unwrap(),
                )
            })
            .unzip();
    let mut rcount: HashMap<usize, usize> = HashMap::new();
    r.iter().for_each(|r| *rcount.entry(*r).or_default() += 1);
    println!(
        "{}",
        l.iter()
            .fold(0, |acc, x| x * (*rcount.entry(*x).or_default()) + acc)
    );
}
