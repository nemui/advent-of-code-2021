pub fn solve_1(filename: &str) -> u32 {
    if let Ok(mut lines) = common::read_lines(filename) {
        let fish = common::list_of_numbers(lines.next().unwrap().unwrap(), ',');    
        return fish.len() as u32;
    }
    0
}

fn main() {
    println!("{}", solve_1("day6/input.txt"));    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(5934, solve_1("test_input.txt"));
    }   
}
