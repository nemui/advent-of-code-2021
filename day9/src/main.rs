use common::Map;
use itertools::Itertools;

pub fn solve_2(filename: &str) -> usize {
    let map = Map::from_file(filename);

    let low_points = map
        .points
        .iter()
        .enumerate()
        .filter(|(index, _)| is_low_point(*index, &map))
        .map(|(index, _)| index);

    low_points
        .map(|index| basin(index, &map))
        .map(|b| b.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn basin(start_index: usize, map: &Map) -> Vec<usize> {
    let mut frontier = vec![start_index];
    let mut visited = vec![];

    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.push(next);
        frontier.extend(
            map.neighbours(next, |p, n_index, m| m[n_index] < 9 && m[n_index] > p)
                .into_iter()
                .filter(|idx| !&visited.contains(idx)),
        );
    }

    visited
}

pub fn solve_1(filename: &str) -> u32 {
    let map = Map::from_file(filename);
    map.points
        .iter()
        .enumerate()
        .filter(|(index, _point)| is_low_point(*index, &map))
        .map(|(_, point)| point + 1)
        .sum()
}

fn is_low_point(index: usize, map: &Map) -> bool {
    map.neighbours(index, |p, n_index, m| m[n_index] <= p)
        .is_empty()
}

fn main() {
    println!("{}", solve_1("day9/input.txt"));
    println!("{}", solve_2("day9/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(15, solve_1("test_input.txt"));
    }
    #[test]
    fn it_works_2() {
        assert_eq!(1134, solve_2("test_input.txt"));
    }
}
