use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("/home/aidan/Projects/advent_of_code/2024/day2/src/input")
            .unwrap()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_whitespace())
            .map(|entries| {
                entries
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&num| num.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .filter(|e| fails_in_sequence(e) < 2)
            .count()
    );
}

fn fails_in_sequence(v: &Vec<isize>) -> isize {
    if v.len() <= 1 {
        return 0;
    }
    let direction = v[1] - v[0] > 0;
    let mut fails = 0;
    v.iter()
        .skip(1)
        .fold((v[0], direction), |(acc, direction), x| {
            let diff = x - acc;
            if diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != direction {
                fails += 1;
            }
            (*x, direction)
        });
    fails
}
