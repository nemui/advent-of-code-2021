fn main() {
    if let Ok(lines) = common::read_lines("day2/input.txt") {
        let commands: Vec<(String, i32)> = lines
            .filter_map(|line| line.ok())
            .map(|line| {
                let mut parts = line.split(' ');
                (
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect();

        let horizontal_position = commands.iter().fold(0, |acc, (code, distance)| {
            acc + if code == "forward" { *distance } else { 0 }
        });
        let vertical_position = commands.iter().fold(0, |acc, (code, distance)| {
            acc + match code.as_str() {
                "down" => *distance,
                "up" => -*distance,
                _ => 0,
            }
        });
        println!("{}", horizontal_position * vertical_position);

        let mut aim = 0;
        let mut vertical_position = 0;
        let mut horizontal_position = 0;

        commands
            .iter()
            .for_each(|(code, units)| match code.as_str() {
                "forward" => {
                    horizontal_position += units;
                    vertical_position += aim * units;
                }
                "down" => {
                    aim += units;
                }
                "up" => {
                    aim -= units;
                }
                _ => {}
            });

        println!("{}", horizontal_position * vertical_position);
    }
}
