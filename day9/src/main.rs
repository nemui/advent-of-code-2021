use itertools::Itertools;

pub fn solve_2(filename: &str) -> usize {
    if let Ok(lines) = common::read_lines(filename) {
        let mut legit_lines = lines.into_iter().map(|line| line.unwrap());
        let first_line = legit_lines.next().unwrap();
        let map_width = first_line.len();
        let mut map_height = 1;

        let mut height_map = line_to_digits(first_line);
        for line in legit_lines {
            height_map.append(&mut line_to_digits(line));
            map_height += 1;
        }

        let low_points = height_map
            .iter()
            .enumerate()
            .filter(|(index, _)| is_low_point(*index, &height_map, map_width, map_height))
            .map(|(index, _)| index);

        return low_points
            .map(|index| basin(index, &height_map, map_width, map_height))
            .map(|b| b.len())
            .sorted()
            .rev()
            .take(3)
            .product();
    }
    0
}

fn basin(
    start_index: usize,
    height_map: &[u32],
    map_width: usize,
    map_height: usize,
) -> Vec<usize> {
    let mut frontier = vec![start_index];
    let mut visited = vec![];

    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.push(next);
        frontier.extend(
            neighbours_that(next, height_map, map_width, map_height, |p, n_index, m| {
                m[n_index] < 9 && m[n_index] > p
            })
            .into_iter()
            .filter(|idx| !&visited.contains(idx)),
        );
    }

    visited
}

pub fn solve_1(filename: &str) -> u32 {
    if let Ok(lines) = common::read_lines(filename) {
        let mut legit_lines = lines.into_iter().map(|line| line.unwrap());
        let first_line = legit_lines.next().unwrap();
        let map_width = first_line.len();
        let mut map_height = 1;

        let mut height_map = line_to_digits(first_line);
        for line in legit_lines {
            height_map.append(&mut line_to_digits(line));
            map_height += 1;
        }

        return height_map
            .iter()
            .enumerate()
            .filter(|(index, _point)| is_low_point(*index, &height_map, map_width, map_height))
            .map(|(_, point)| point + 1)
            .sum();
    }
    0
}

fn is_low_point(index: usize, height_map: &[u32], map_width: usize, map_height: usize) -> bool {
    neighbours_that(index, height_map, map_width, map_height, |p, n_index, m| {
        m[n_index] <= p
    })
    .is_empty()
}

fn neighbours_that<T>(
    index: usize,
    height_map: &[u32],
    map_width: usize,
    map_height: usize,
    condition: T,
) -> Vec<usize>
where
    T: Fn(u32, usize, &[u32]) -> bool,
{
    let point = height_map[index];
    let (px, py) = idx_to_xy(index, map_width);
    let imw = map_width as i32;
    let imh = map_height as i32;
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let (x, y) = (px as i32 + dx, py as i32 + dy);
            if x >= 0 && x < imw && y >= 0 && y < imh {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
        .map(|(x, y)| xy_to_idx(x, y, map_width))
        .filter(|n_index| condition(point, *n_index, height_map))
        .collect()
}

fn idx_to_xy(idx: usize, map_width: usize) -> (usize, usize) {
    (idx % map_width, idx / map_width)
}

fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn line_to_digits<T: 'static>(line: T) -> Vec<u32>
where
    T: AsRef<str>,
{
    line.as_ref()
        .chars()
        .into_iter()
        .filter_map(|c| c.to_digit(10))
        .collect()
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
