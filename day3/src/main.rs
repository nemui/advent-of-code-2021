fn calculate_position_bits(position: usize, readings: &[String]) -> (i32, i32) {
    readings.iter().fold((0, 0), |acc, reading| {
        if reading.chars().nth(position).unwrap() == '0' {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        }
    })
}

fn parse_binary_string(bs: &str) -> i32 {
    isize::from_str_radix(bs, 2).unwrap() as i32
}

fn extract_rating(readings: &[String], digit_selector: impl Fn((i32, i32)) -> char) -> i32 {
    let mut local_readings: Vec<String> = readings.to_vec();

    parse_binary_string(
        &(0..readings[0].len())
            .filter_map(|index| {
                let pb = calculate_position_bits(index, &local_readings);
                let digit = digit_selector(pb);
                let matching_readings: Vec<String> = local_readings
                    .iter()
                    .cloned()
                    .filter(|reading| reading.chars().nth(index).unwrap() == digit)
                    .collect();
                if matching_readings.len() == 1 {
                    Some(matching_readings[0].clone())
                } else {
                    local_readings = matching_readings;
                    None
                }
            })
            .next()
            .unwrap(),
    )
}

pub fn solve(filename: &str) -> (i32, i32) {
    if let Ok(lines) = common::read_lines(filename) {
        let readings: Vec<String> = lines.filter_map(|line| line.ok()).collect();

        let position_bits = (0..readings[0].len())
            .into_iter()
            .map(|position| calculate_position_bits(position, &readings));

        let (gamma_rate, epsilon_rate): (Vec<char>, Vec<char>) = position_bits
            .clone()
            .map(|(zero_count, one_count)| {
                if zero_count > one_count {
                    ('0', '1')
                } else {
                    ('1', '0')
                }
            })
            .unzip();

        let gamma_rate = parse_binary_string(&gamma_rate.iter().collect::<String>());
        let epsilon_rate = parse_binary_string(&epsilon_rate.iter().collect::<String>());

        let oxygen_generator_rating = extract_rating(&readings, |(zeros, ones)| -> char {
            if zeros > ones {
                '0'
            } else {
                '1'
            }
        });
        let co2_scrubber_rating = extract_rating(&readings, |(zeros, ones)| -> char {
            if ones < zeros {
                '1'
            } else {
                '0'
            }
        });

        return (
            gamma_rate * epsilon_rate,
            oxygen_generator_rating * co2_scrubber_rating,
        );
    }
    (0, 0)
}

fn main() {
    println!("{:?}", solve("day3/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((198, 230), solve("test_input.txt"));
    }
}
