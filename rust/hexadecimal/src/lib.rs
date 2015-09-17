pub fn hex_to_int(hex: &str) -> Option<u32> {
    hex.chars().fold(Some(0), |sum, hex_char| {
        match hex_value(hex_char) {
            Some(hex_value) => sum.map(|x| (x << 4) + hex_value),
            None => None
        }
    })
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
