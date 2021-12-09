use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_1(filename: &str) -> usize {
    if let Ok(lines) = common::read_lines(filename) {
        return lines
            .into_iter()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .map(|line| simple_digits(&line))
            .sum();
    }
    0
}

pub fn solve_2(filename: &str) -> usize {
    if let Ok(lines) = common::read_lines(filename) {
        return lines
            .into_iter()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .map(|line| signal_value(&line))
            .sum();
    }
    0
}

fn simple_digits(line: &str) -> usize {
    let parts: Vec<_> = line.split('|').into_iter().map(|s| s.trim()).collect();
    let lengths: Vec<usize> = vec![2, 3, 4, 7];
    let patterns: Vec<_> = parts[0]
        .split(' ')
        .into_iter()
        .filter(|pattern| lengths.contains(&pattern.len()))
        .map(|pattern| pattern.chars().into_iter().sorted().collect::<String>())
        .collect();
    parts[1]
        .split(' ')
        .map(|signal| signal.chars().into_iter().sorted().collect::<String>())
        .filter(|signal| patterns.contains(signal))
        .count()
}

fn signal_value(line: &str) -> usize {
    let parts: Vec<_> = line.split('|').into_iter().map(|s| s.trim()).collect();

    let patterns: Vec<_> = parts[0]
        .split(' ')
        .into_iter()
        .map(|pattern| pattern.chars().into_iter().sorted().collect::<String>())
        .collect();

    let mut code: HashMap<String, usize> = HashMap::new();
    vec![(1, 2), (4, 4), (7, 3), (8, 7)]
        .into_iter()
        .for_each(|(digit, length)| {
            code.insert(find_by_length(&patterns, length), digit);
        });

    vec![(3, 5, 3, 1), (9, 6, 2, 4), (2, 5, 1, 9), (6, 6, 4, 7)]
        .into_iter()
        .for_each(|(digit, size, diff, base)| {
            code.insert(find_by_diff(&patterns, &code, size, diff, base), digit);
        });

    code.insert(
        patterns
            .iter()
            .find(|pattern| pattern.len() == 6 && !code.contains_key(*pattern))
            .unwrap()
            .to_string(),
        0,
    );

    code.insert(
        patterns
            .iter()
            .find(|pattern| !code.contains_key(*pattern))
            .unwrap()
            .to_string(),
        5,
    );

    println!("code {:?}", code);
    let signals: Vec<_> = parts[1]
        .split(' ')
        .map(|signal| signal.chars().into_iter().sorted().collect::<String>())
        .collect();
    println!("signals {:?}", signals);

    signals
        .iter()
        .map(|signal| code.get(signal).unwrap())
        .join("")
        .parse()
        .unwrap()
}

fn find_by_length(patterns: &[String], length: usize) -> String {
    patterns
        .iter()
        .find(|pattern| pattern.len() == length)
        .unwrap()
        .to_string()
}

fn find_by_diff(
    patterns: &[String],
    code: &HashMap<String, usize>,
    size: usize,
    diff: usize,
    base: usize,
) -> String {
    let base_pattern = code.iter().find(|(_, d)| **d == base).unwrap().0;
    patterns
        .iter()
        .filter(|pattern| pattern.len() == size)
        .find(|pattern| diff_patterns(pattern, base_pattern) == diff)
        .unwrap()
        .to_string()
}

fn diff_patterns(p1: &str, p2: &str) -> usize {
    p1.chars().filter(|c| !p2.contains(*c)).count()
}

fn main() {
    println!("{}", solve_1("day8/input.txt"));
    println!("{}", solve_2("day8/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(26, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(61229, solve_2("test_input.txt"));
    }

    #[test]
    fn diff_patterns_test() {
        assert_eq!(0, diff_patterns("abc", "abc"));
        assert_eq!(1, diff_patterns("abc", "ab"));
        assert_eq!(3, diff_patterns("abc", "de"));
    }
}
