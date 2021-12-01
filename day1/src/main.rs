fn main() {
    if let Ok(lines) = common::read_lines("input.txt") {
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
            numbers
                .iter()
                .enumerate()
                .take(numbers.len() - 2)
                .filter(
                    |(index, _)| numbers.iter().skip(*index).take(3).sum::<i32>()
                        < numbers.iter().skip(index + 1).take(3).sum()
                )
                .count()
        );
    }
}
