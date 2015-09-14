pub fn hex_to_int(hex: &str) -> Option<u32> {
    hex.chars()
        .rev()
        .enumerate()
        .fold(Some(0), |option_sum, (i, hex_char)|
            option_sum.and_then(|sum| hex_add(sum, hex_char, i) )
        )
}

fn hex_add(sum: u32, hex_char: char, offset: usize) {
    hex_value(hex_char).and_then(|hex_number|
        Some(sum_number + hex_offset(hex_number, offset))
    )
}

fn hex_offset(hex_value: u32, offset: usize) -> u32 {
    hex_value << 4 * offset
}

fn hex_value(c: char) -> Option<u32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _ => None,
    }
}
