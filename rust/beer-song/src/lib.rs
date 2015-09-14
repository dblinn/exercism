pub fn sing(start_verse: usize, end_verse: usize) -> String {
    (end_verse .. start_verse + 1).rev()
        .map(|verse_index| verse(verse_index))
        .collect::<Vec<String>>()
        .connect("\n")
}

pub fn verse(verse_index: usize) -> String {
    match verse_index {
        0 => verse_zero(),
        1 => verse_one(),
        2 => verse_two(),
        _ => other_verse(verse_index),
    }
}

fn verse_zero() -> String {
    String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
}

fn verse_one() -> String {
    String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
}

fn verse_two() -> String {
    String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
}

fn other_verse(verse_index: usize) -> String {
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
        verse_index,
        verse_index,
        verse_index - 1
    )
}
