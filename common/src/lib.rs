use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Map {
    pub points: Vec<u32>,
    pub height: usize,
    pub width: usize,
}

impl Map {
    pub fn from_file<P>(filename: P) -> Map
    where
        P: AsRef<Path>,
    {
        let mut lines = read_lines(filename)
            .unwrap()
            .into_iter()
            .map(|line| line.unwrap());
        let first_line = lines.next().unwrap();
        let mut points = line_to_digits(first_line);
        let mut height = 1;
        let width = points.len();
        for line in lines {
            points.append(&mut line_to_digits(line));
            height += 1;
        }
        Map {
            points,
            height,
            width,
        }
    }

    pub fn neighbours<T>(&self, index: usize, condition: T) -> Vec<usize>
    where
        T: Fn(u32, usize, &[u32]) -> bool,
    {
        let point = self.points[index];
        let (px, py) = Map::idx_to_xy(index, self.width);
        let imw = self.width as i32;
        let imh = self.height as i32;
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
            .map(|(x, y)| Map::xy_to_idx(x, y, self.width))
            .filter(|n_index| condition(point, *n_index, &self.points))
            .collect()
    }

    fn idx_to_xy(idx: usize, map_width: usize) -> (usize, usize) {
        (idx % map_width, idx / map_width)
    }

    fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }
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

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn list_of_numbers<T: AsRef<str>>(line: T, separator: char) -> Vec<i32> {
    line.as_ref()
        .split(separator)
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lines = read_lines("test.txt").unwrap();
        assert_eq!("one", lines.next().unwrap().unwrap());
        assert_eq!("two", lines.next().unwrap().unwrap());
    }
}
