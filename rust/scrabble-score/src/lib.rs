pub fn score(s: &str) -> u32 {
    s.chars().flat_map(char::to_uppercase).map(score_letter).sum()
}

fn score_letter(uppercase_letter: char) -> u32 {
    match uppercase_letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' =>                                                 2,
        'B' | 'C' | 'M' | 'P' =>                                     3,
        'F' | 'H' | 'V' | 'W' | 'Y' =>                               4,
        'K' =>                                                       5,
        'J' | 'X' =>                                                 8,
        'Q' | 'Z' =>                                                 10,
        _ =>                                                         0
    }
}
