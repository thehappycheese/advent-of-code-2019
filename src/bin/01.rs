use advent_of_code_2019::parse::read_input;
use std::path::Path;

fn fuel(mass: u64) -> u64 {
    u64::div_euclid(mass, 3).saturating_sub(2)
}
fn correct_fuel(mass: u64) -> u64 {
    let mut sum = 0u64;
    let mut f = mass;
    loop {
        f = fuel(f);
        sum += f;
        if f == 0u64 {
            break;
        }
    }
    sum
}

fn part1(input: &Vec<u64>) -> u64 {
    let result: u64 = input.iter().map(|mass| fuel(*mass)).sum();
    result
}

fn part2(input: &Vec<u64>) -> u64 {
    let result = input.iter().map(|mass| correct_fuel(*mass)).sum();
    result
}

fn main() {
    let input = read_input(Path::new("./input/01.txt"));
    let input: Vec<u64> = input.lines().map(|i| str::parse(i).unwrap()).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2(){
        assert_eq!(correct_fuel(1969),966);
        assert_eq!(correct_fuel(100756),50346);
    }
}