use itertools::Itertools;
use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("part_1 -> {}", part_1(INPUT));
    println!("part_2 -> {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::chars)
        .map(|chars| chars.filter(|c| c.is_ascii_digit()))
        .flat_map(|mut chars| Some((chars.next()?, chars.last())))
        .flat_map(|(first, last)| {
            let last = last.unwrap_or(first);

            format!("{}{}", first, last).parse::<usize>()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let regexes = [
        "1|one", "2|two", "3|three", "4|four", "5|five", "6|six", "7|seven", "8|eight", "9|nine",
    ]
    .map(Regex::new)
    .map(Result::unwrap);

    input
        .lines()
        .flat_map(|line| {
            let matches: Vec<_> = regexes
                .iter()
                .enumerate()
                .flat_map(|(i, re)| re.find_iter(line).map(move |mat| (i + 1, mat)))
                .sorted_by(|(_, a), (_, b)| a.range().cmp(b.range()))
                .map(|(num, _)| num)
                .collect();

            let first = matches.first()?;
            let last = matches.last()?;

            Some(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        const INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        const OUTPUT: usize = 142;

        assert_eq!(part_1(INPUT), OUTPUT);
    }

    #[test]
    fn part_2_example() {
        const INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        const OUTPUT: usize = 281;

        assert_eq!(part_2(INPUT), OUTPUT);
    }
}
