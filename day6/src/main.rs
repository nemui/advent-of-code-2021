pub fn solve_1(filename: &str) -> usize {
    if let Ok(mut lines) = common::read_lines(filename) {
        return multiply_fish(
            common::list_of_numbers(lines.next().unwrap().unwrap(), ','),
            80,
        );
    }
    0
}

pub fn solve_2(filename: &str) -> usize {
    if let Ok(mut lines) = common::read_lines(filename) {
        return multiply_fish_fast(
            common::list_of_numbers(lines.next().unwrap().unwrap(), ','),
            256,
        );
    }
    0
}

fn multiply_fish(starting_fish: Vec<i32>, days: i32) -> usize {
    let mut fish = starting_fish;
    for _ in 0..days {
        fish = fish
            .iter()
            .flat_map(|f| if f - 1 < 0 { vec![6, 8] } else { vec![f - 1] })
            .collect();
    }
    fish.len()
}

fn multiply_fish_fast(starting_fish: Vec<i32>, days: i32) -> usize {
    let mut fish_count_by_age: Vec<usize> = vec![0; 9];
    for fish in starting_fish {
        fish_count_by_age[fish as usize] += 1;
    }

    for _ in 0..days {
        let zero_days_fish = fish_count_by_age[0];
        for index in 0..8 {
            fish_count_by_age[index] = fish_count_by_age[index + 1];
        }
        fish_count_by_age[6] += zero_days_fish;
        fish_count_by_age[8] = zero_days_fish;
    }

    return fish_count_by_age.iter().sum();
}

fn main() {
    println!("{}", solve_1("day6/input.txt"));
    println!("{}", solve_2("day6/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(5934, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(26984457539, solve_2("test_input.txt"));
    }
}
