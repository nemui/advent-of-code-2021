const BOARD_SIZE: usize = 5;

pub fn solve_1(filename: &str) -> i32 {
    if let Ok(lines) = common::read_lines(filename) {
        let mut lines = lines.filter_map(|line| line.ok());
        let numbers = common::list_of_numbers(lines.next().unwrap(), ',');
        let mut boards = parse_boards(&lines.skip(1).collect::<Vec<String>>());

        for number in numbers {
            boards = mark_number(number, &boards);
            if let Some(win_board) = boards
                .iter()
                .filter_map(|board| is_winning_board(board))
                .next()
            {
                return final_score(&win_board, number);
            }
        }

        return 0;
    }
    0
}

pub fn solve_2(filename: &str) -> i32 {
    if let Ok(lines) = common::read_lines(filename) {
        let mut lines = lines.filter_map(|line| line.ok());
        let numbers = common::list_of_numbers(lines.next().unwrap(), ',');
        let mut boards = parse_boards(&lines.skip(1).collect::<Vec<String>>());

        for number in numbers {
            boards = mark_number(number, &boards);
            let won_this_round: Vec<Vec<(i32, bool)>> = boards
                .iter()
                .filter_map(|board| {
                    if has_winning_row(board) || has_winning_col(board) {
                        Some(board.clone())
                    } else {
                        None
                    }
                })
                .collect();
            boards.retain(|board| !won_this_round.contains(board));

            if boards.is_empty() {
                return final_score(won_this_round.iter().last().unwrap(), number);
            }
        }

        return 0;
    }
    0
}

fn final_score(board: &[(i32, bool)], number: i32) -> i32 {
    let unmarked_sum: i32 = board
        .iter()
        .filter(|(_, is_marked)| !is_marked)
        .map(|(num, _)| num)
        .sum();
    unmarked_sum * number
}

fn is_winning_board(board: &[(i32, bool)]) -> Option<Vec<(i32, bool)>> {
    if has_winning_row(board) || has_winning_col(board) {
        return Some(board.to_vec());
    }

    None
}

fn has_winning_row(board: &[(i32, bool)]) -> bool {
    (0..BOARD_SIZE).into_iter().any(|row| {
        !(0..BOARD_SIZE)
            .into_iter()
            .any(|col| !board[idx(row, col)].1)
    })
}

fn has_winning_col(board: &[(i32, bool)]) -> bool {
    (0..BOARD_SIZE).into_iter().any(|col| {
        !(0..BOARD_SIZE)
            .into_iter()
            .any(|row| !board[idx(row, col)].1)
    })
}

fn idx(row: usize, col: usize) -> usize {
    col + row * BOARD_SIZE
}

fn parse_boards(lines: &[String]) -> Vec<Vec<(i32, bool)>> {
    (0..lines.len())
        .into_iter()
        .step_by(BOARD_SIZE + 1)
        .map(|index| {
            (index..index + BOARD_SIZE)
                .into_iter()
                .flat_map(|sub_index| {
                    common::list_of_numbers(&lines[sub_index], ' ')
                        .into_iter()
                        .zip(vec![false; BOARD_SIZE].into_iter())
                })
                .collect::<Vec<(i32, bool)>>()
        })
        .collect()
}

fn mark_number(number: i32, boards: &[Vec<(i32, bool)>]) -> Vec<Vec<(i32, bool)>> {
    boards
        .iter()
        .clone()
        .into_iter()
        .map(|board| {
            board
                .iter()
                .clone()
                .map(|(num, is_marked)| (*num, *is_marked || *num == number))
                .collect::<Vec<(i32, bool)>>()
        })
        .collect()
}

fn main() {
    println!("{}", solve_1("day4/input.txt"));
    println!("{}", solve_2("day4/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(4512, solve_1("test_input.txt"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(1924, solve_2("test_input.txt"));
    }

    fn test_board<T: AsRef<str>>(board: T) -> Vec<String> {
        board
            .as_ref()
            .lines()
            .into_iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn parse_boards_test() {
        assert_eq!(
            vec![vec![(1, false); 25]],
            parse_boards(&test_board(
                r"1 1 1 1 1
                1 1 1 1 1
                1 1 1 1 1
                1 1 1 1 1
                1 1 1 1 1"
            ))
        );
    }

    #[test]
    fn winning_row_test() {
        let boards = parse_boards(&test_board(
            r"1 2 3 4 5
            1 1 1 1 1
            11 12 13 14 15
            15 16 17 0 18
            19 20 21 22 23",
        ));
        assert_eq!(false, has_winning_row(&boards[0]));
        let boards = mark_number(1, &boards);
        assert_eq!(true, has_winning_row(&boards[0]));
    }

    #[test]
    fn winning_col_test() {
        let boards = parse_boards(&test_board(
            r"1 2 0 4 5
            1 1 0 1 1
            3 12 0 14 11
            1 16 0 0 18
            19 20 0 22 23",
        ));
        println!("{:?}", boards[0]);
        assert_eq!(false, has_winning_col(&boards[0]));
        let boards = mark_number(0, &boards);
        assert_eq!(true, has_winning_col(&boards[0]));
    }
}
