pub mod day01 {
    use itertools::Itertools;

    pub fn part1() {
        println!("----- Day 01 Part 1 ------");

        let lines = include_str!("inputs/day01.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let max: u64 = lines
            .split(|line| line.is_none())
            .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
            .max()
            .unwrap();

        println!("part1 = {max:?}");
    }

    pub fn part2() {
        println!("----- Day 01 Part 2 ------");

        let lines = include_str!("inputs/day01.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let sum: u64 = lines
            .split(|line| line.is_none())
            .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
            .sorted_by(|a,b| b.cmp(a))
            .take(3)
            .sum();

        println!("part2 = {sum:?}");
    }
}