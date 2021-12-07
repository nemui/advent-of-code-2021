pub fn solve_1(filename: &str) -> i32 {
    if let Ok(mut lines) = common::read_lines(filename) {
        let crab_submarine_positions = common::list_of_numbers(lines.next().unwrap().unwrap(), ',');
        let min_position = *crab_submarine_positions.iter().min().unwrap();
        let max_position = *crab_submarine_positions.iter().max().unwrap();
        return (min_position..max_position + 1)
            .into_iter()
            .map(|position| {
                crab_submarine_positions
                    .iter()
                    .fold(0, |acc, p| acc + (p - position).abs())
            })
            .min()
            .unwrap();
    }
    0
}

pub fn solve_2(filename: &str) -> i32 {
    if let Ok(mut lines) = common::read_lines(filename) {
        let crab_submarine_positions = common::list_of_numbers(lines.next().unwrap().unwrap(), ',');
        let min_position = *crab_submarine_positions.iter().min().unwrap();
        let max_position = *crab_submarine_positions.iter().max().unwrap();
        return (min_position..max_position + 1)
            .into_iter()
            .map(|position| {
                crab_submarine_positions
                    .iter()
                    .fold(0, |acc, p| acc + crabonacci((p - position).abs())) // 1 1, 2 - 3, 3 - 6
            })
            .min()
            .unwrap();
    }
    0
}

fn crabonacci(n: i32) -> i32 {
    if n > 0 {
        n + crabonacci(n - 1)
    } else {
        n
    }
}

fn main() {
    println!("{}", solve_1("day7/input.txt"));
    println!("{}", solve_2("day7/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(37, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(168, solve_2("test_input.txt"));
    }
}
