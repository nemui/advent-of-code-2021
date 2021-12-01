fn main() {
    if let Ok(lines) = common::read_lines("day1/input.txt") {
        let numbers: Vec<i32> = lines
            .filter_map(|line| line.ok())
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();
        println!(
            "{}",
            numbers
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(index, number)| **number > numbers[index - 1])
                .count()
        );
        println!(
            "{}",
            (0..numbers.len() - 2)
                .filter(|index| numbers.iter().skip(*index).take(3).sum::<i32>()
                    < numbers.iter().skip(index + 1).take(3).sum())
                .count()
        );
    }
}
