pub fn solve_2(filename: &str) -> u64 {
    if let Ok(lines) = common::read_lines(filename) {
        let mut scores: Vec<u64> = lines
            .into_iter()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| first_incorrect_closing_character(line).is_none())
            .map(|line| complete_sequence(&line))
            .map(|sequence| sequence_score(&sequence))
            .collect();
        scores.sort_unstable();
        return scores[scores.len() / 2];
    }
    0
}

fn sequence_score(chars: &[char]) -> u64 {
    chars.iter().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            }
    })
}

fn complete_sequence(line: &str) -> Vec<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if OPENING_CHARS.contains(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    stack
        .iter()
        .rev()
        .map(|c| CLOSING_CHARS[OPENING_CHARS.iter().position(|x| *x == *c).unwrap()])
        .collect()
}

pub fn solve_1(filename: &str) -> u32 {
    if let Ok(lines) = common::read_lines(filename) {
        return lines
            .into_iter()
            .filter_map(|line| first_incorrect_closing_character(line.unwrap().trim()))
            .map(|c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            })
            .sum();
    }
    0
}

const OPENING_CHARS: &[char] = &['(', '[', '{', '<'];
const CLOSING_CHARS: &[char] = &[')', ']', '}', '>'];

fn first_incorrect_closing_character(line: &str) -> Option<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if OPENING_CHARS.contains(&c) {
            stack.push(c);
            continue;
        }
        if let Some(last_char) = stack.pop() {
            if last_char != OPENING_CHARS[CLOSING_CHARS.iter().position(|x| *x == c).unwrap()] {
                return Some(c);
            }
        } else {
            return Some(c);
        }
    }
    None
}

fn main() {
    println!("{}", solve_1("day10/input.txt"));
    println!("{}", solve_2("day10/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(26397, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(288957, solve_2("test_input.txt"));
    }
}
