pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    let zero_one_board = convert_to_zero_one(board);

    zero_one_board.iter().enumerate().map(|(row, line)| {
        let previous_line = previous(row).map(|i| &zero_one_board[i]);
        let next_line = next(row, board.len()).map(|i| &zero_one_board[i]);

        line.iter().enumerate().map(|(col, n)| {
            count_neighbors(previous_line, line, next_line, col, *n)
        }).collect()
    }).collect()
}

fn count_neighbors(previous_line: Option<&Vec<u32>>, cur_line: &Vec<u32>,
                    next_line: Option<&Vec<u32>>, index: usize, n: u32) -> char
{
    if n == 1 { return '*' }
    to_minesweeper_char(
        line_value(previous_line, index) +
        line_value(Some(cur_line), index) +
        line_value(next_line, index)
    )
}

fn convert_to_zero_one(board: &Vec<&str>) -> Vec<Vec<u32>> {
    board.iter().map(|line|
        line.chars().map(|c| minesweeper_value(c)).collect()
    ).collect()
}

fn previous(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some(index - 1) }
}

fn next(index: usize, max: usize) -> Option<usize> {
    if (index + 1) >= max { None } else { Some(index + 1) }
}

fn line_value(option_line: Option<&Vec<u32>>, index: usize) -> u32 {
    match option_line {
        None => 0,
        Some(line) => option_lookup(previous(index), line) +
                    line[index] +
                    option_lookup(next(index, line.len()), line)
    }
}

fn option_lookup(option_index: Option<usize>, line: &Vec<u32>) -> u32 {
    option_index.map(|index| line[index]).unwrap_or(0)
}

fn minesweeper_value(c: char) -> u32 { match c { '*' => 1, _ => 0 }}

fn to_minesweeper_char(number: u32) -> char {
    match number {
        0 => ' ',
        _ => std::char::from_digit(number, 10).unwrap()
    }
}
