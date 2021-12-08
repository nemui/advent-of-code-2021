use itertools::Itertools;

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
        .map(|signal| patterns.iter().position(|p| *p == signal).unwrap())
        .join("")
        .parse()
        .unwrap()
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
}
