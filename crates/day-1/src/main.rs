use itertools::Itertools;
use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("part1 -> {}", part1(INPUT));
    println!("part2 -> {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(str::chars)
        .map(|chars| chars.filter(|c| c.is_ascii_digit()))
        .flat_map(|mut chars| Some((chars.next()?, chars.last())))
        .flat_map(|(first, last)| {
            let last = last.unwrap_or(first);

            format!("{}{}", first, last).parse::<u32>()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let regexes = [
        Regex::new(r"1|one"),
        Regex::new(r"2|two"),
        Regex::new(r"3|three"),
        Regex::new(r"4|four"),
        Regex::new(r"5|five"),
        Regex::new(r"6|six"),
        Regex::new(r"7|seven"),
        Regex::new(r"8|eight"),
        Regex::new(r"9|nine"),
    ]
    .map(Result::unwrap);

    input
        .lines()
        .flat_map(|line| {
            let matches: Vec<_> = regexes
                .iter()
                .enumerate()
                .map(|(i, re)| re.find_iter(line).map(move |mat| (i + 1, mat)))
                .flatten()
                .sorted_by(|(_, a), (_, b)| a.range().cmp(b.range()))
                .map(|(num, _)| num)
                .collect();

            let first = *matches.first()? as u32;
            let last = *matches.last()? as u32;

            Some((first * 10) + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        const INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        const OUTPUT: u32 = 142;

        assert_eq!(part1(INPUT), OUTPUT);
    }

    #[test]
    fn part2_example() {
        const INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        const OUTPUT: u32 = 281;

        assert_eq!(part2(INPUT), OUTPUT);
    }
}
