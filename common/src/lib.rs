use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
