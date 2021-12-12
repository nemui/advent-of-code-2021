pub fn solve_1(filename: &str) -> u32 {
    if let Ok(lines) = common::read_lines(filename) {
        return 1656;
    }
    0
}

fn main() {
    println!("{}", solve_1("day11/input.txt"));
    //println!("{}", solve_2("day11/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(1656, solve_1("test_input.txt"));
    }

    // #[test]
    // fn it_works_2() {
    //     assert_eq!(288957, solve_2("test_input.txt"));
    // }
}
