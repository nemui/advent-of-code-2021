use std::collections::HashMap;
use std::iter::repeat;

#[derive(Clone, Copy, Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn new(coords: Vec<i32>) -> Line {
        Self {
            x1: coords[0],
            y1: coords[1],
            x2: coords[2],
            y2: coords[3],
        }
    }
}

pub fn solve_1(filename: &str) -> usize {
    if let Ok(lines) = common::read_lines(filename) {
        let line_coords: Vec<Line> = lines
            .filter_map(|line| line.ok())
            .map(|line| {
                Line::new(
                    line.split(" -> ")
                        .map(|pair| common::list_of_numbers(pair, ','))
                        .flatten()
                        .collect(),
                )
            })
            .collect();

        let points: Vec<(i32, i32)> = line_coords
            .iter()
            .filter(|line| is_straight_line(**line))
            .flat_map(|line| build_straight_line(*line))
            .collect();

        let mut ocean_floor: HashMap<(i32, i32), i32> = HashMap::new();
        for point in points {
            let vent_count = ocean_floor.entry(point).or_insert(0);
            *vent_count += 1;
        }

        if cfg!(test) {
            debug_print(&ocean_floor);
        }

        return ocean_floor
            .iter()
            .filter(|(_, vent_count)| **vent_count > 1)
            .count();
    }
    0
}

pub fn solve_2(filename: &str) -> usize {
    if let Ok(lines) = common::read_lines(filename) {
        let line_coords: Vec<Line> = lines
            .filter_map(|line| line.ok())
            .map(|line| {
                Line::new(
                    line.split(" -> ")
                        .map(|pair| common::list_of_numbers(pair, ','))
                        .flatten()
                        .collect(),
                )
            })
            .collect();

        let points: Vec<(i32, i32)> = line_coords
            .iter()
            .flat_map(|line| {
                if is_straight_line(*line) {
                    build_straight_line(*line)
                } else {
                    build_diagonal_line(*line)
                }
            })
            .collect();

        let mut ocean_floor: HashMap<(i32, i32), i32> = HashMap::new();
        for point in points {
            let vent_count = ocean_floor.entry(point).or_insert(0);
            *vent_count += 1;
        }

        if cfg!(test) {
            debug_print(&ocean_floor);
        }

        return ocean_floor
            .iter()
            .filter(|(_, vent_count)| **vent_count > 1)
            .count();
    }
    0
}

fn debug_print(floor: &HashMap<(i32, i32), i32>) {
    let width = floor
        .iter()
        .fold(0, |acc, (point, _)| std::cmp::max(acc, point.0));
    let height = floor
        .iter()
        .fold(0, |acc, (point, _)| std::cmp::max(acc, point.1));

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            print!("{}", floor.get(&(x, y)).unwrap_or(&0));
        }
        println!();
    }
}

fn build_straight_line(line: Line) -> Vec<(i32, i32)> {
    (std::cmp::min(line.x1, line.x2)..std::cmp::max(line.x1, line.x2) + 1)
        .into_iter()
        .flat_map(|x| {
            let min_y = std::cmp::min(line.y1, line.y2);
            let max_y = std::cmp::max(line.y1, line.y2);
            repeat(x)
                .take((max_y + 1 - min_y) as usize)
                .zip((min_y..max_y + 1).into_iter())
        })
        .collect()
}

fn build_diagonal_line(line: Line) -> Vec<(i32, i32)> {
    sequence(line.x1, line.x2)
        .into_iter()
        .zip(sequence(line.y1, line.y2))
        .collect()
}

fn sequence(start: i32, end: i32) -> Vec<i32> {
    let range = std::cmp::min(start, end)..std::cmp::max(start, end) + 1;
    if end < start {
        range.rev().collect()
    } else {
        range.collect()
    }
}

fn is_straight_line(line: Line) -> bool {
    line.x1 == line.x2 || line.y1 == line.y2
}

fn main() {
    println!("{}", solve_1("day5/input.txt"));
    println!("{}", solve_2("day5/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(5, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(12, solve_2("test_input.txt"));
    }

    #[test]
    fn build_horizontal_line() {
        assert_eq!(
            vec![(0, 9), (1, 9), (2, 9)],
            build_straight_line(Line::new(vec![0, 9, 2, 9]))
        );
    }

    #[test]
    fn build_vertical_line() {
        assert_eq!(
            vec![(0, 7), (0, 8), (0, 9)],
            build_straight_line(Line::new(vec![0, 7, 0, 9]))
        );
    }

    #[test]
    fn build_diagonal_line_test() {
        assert_eq!(
            vec![(1, 1), (2, 2), (3, 3)],
            build_diagonal_line(Line::new(vec![1, 1, 3, 3]))
        );
    }

    #[test]
    fn build_diagonal_line_test2() {
        assert_eq!(
            vec![(8, 0), (7, 1), (6, 2)],
            build_diagonal_line(Line::new(vec![8, 0, 6, 2]))
        );
    }
}
