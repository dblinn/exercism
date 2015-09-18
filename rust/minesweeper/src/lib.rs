pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    board.iter().enumerate().map(|(row, line)| {
        let previous_line = previous(row).map(|i| board[i]);
        let next_line = next(row, board.len()).map(|i| board[i]);

        line.chars().enumerate().map(|(col, c)| {
            count_neighbors(previous_line, line, next_line, col, c)
        }).collect()
    }).collect()
}

fn count_neighbors(previous_line: Option<&str>, cur_line: &str,
                    next_line: Option<&str>, char_index: usize, c: char) -> char
{
    if c == '*' { return '*' }
    to_minesweeper_char(
        line_value(previous_line, char_index) +
        line_value(Some(cur_line), char_index) - minesweeper_value(Some(c)) +
        line_value(next_line, char_index)
    )
}

fn previous(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some(index - 1) }
}

fn next(index: usize, max: usize) -> Option<usize> {
    if (index + 1) >= max { None } else { Some(index + 1) }
}

fn line_value(option_line: Option<&str>, char_index: usize) -> u32 {
    match option_line {
        None => 0,
        Some(line) => option_lookup(previous(char_index), line) +
                    minesweeper_value(line.chars().nth(char_index)) +
                    option_lookup(next(char_index, line.len()), line)
    }
}

fn option_lookup(option_index: Option<usize>, line: &str) -> u32 {
    option_index.map(|index| minesweeper_value(line.chars().nth(index))).unwrap_or(0)
}

fn minesweeper_value(c: Option<char>) -> u32 { match c { Some('*') => 1, _ => 0 }}

fn to_minesweeper_char(number: u32) -> char {
    match number {
        0 => ' ',
        _ => std::char::from_digit(number, 10).unwrap()
    }
}
