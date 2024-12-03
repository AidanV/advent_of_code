use std::fs;

fn main() {
    fs::read_to_string("/home/aidan/Projects/advent_of_code/2024/day2/src/input")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace())
        .map(|entries| entries.map(|num: &str| num.parse::<isize>().unwrap()))
        .filter(|e| is_valid_sequence(e));
}

fn is_valid_sequence(v: Vec<isize>) -> bool {
    if v.len() <= 1 {
        return true;
    }
    let direction = v[1] - v[0] > 0;
    v.iter()
        .fold(Some((v[0], direction)), |acc, x| match acc {
            Some((acc, direction)) => {
                let diff = x - acc;
                if diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != direction {
                    None
                } else {
                    Some((*x, direction))
                }
            }
            None => None,
        })
        .is_some()
}
