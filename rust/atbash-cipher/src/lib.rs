fn atbash(c: char) -> Option<char> {
    match c {
        'a' => Some('z'),
        'b' => Some('y'),
        'c' => Some('x'),
        'd' => Some('w'),
        'e' => Some('v'),
        'f' => Some('u'),
        'g' => Some('t'),
        'h' => Some('s'),
        'i' => Some('r'),
        'j' => Some('q'),
        'k' => Some('p'),
        'l' => Some('o'),
        'm' => Some('n'),
        'n' => Some('m'),
        'o' => Some('l'),
        'p' => Some('k'),
        'q' => Some('j'),
        'r' => Some('i'),
        's' => Some('h'),
        't' => Some('g'),
        'u' => Some('f'),
        'v' => Some('e'),
        'w' => Some('d'),
        'x' => Some('c'),
        'y' => Some('b'),
        'z' => Some('a'),
        '0' => Some('0'),
        '1' => Some('1'),
        '2' => Some('2'),
        '3' => Some('3'),
        '4' => Some('4'),
        '5' => Some('5'),
        '6' => Some('6'),
        '7' => Some('7'),
        '8' => Some('8'),
        '9' => Some('9'),
        _ => None
    }
}

struct Encoder {
    count: u32,
    text: String
}

impl Encoder {
    pub fn encode_next(mut self, c: char) -> Encoder {
        if self.count % 5 == 0 && self.count != 0 { self.text.push(' '); }
        self.count += 1;
        self.text.push(c);

        self
    }
}

// It would be nice to be able to use impl Trait to avoid duplication
// but it is not in stable yet.
pub fn encode(input: &str) -> String {
    let initial_encoder = Encoder { count: 0, text: String::with_capacity(input.len()) };

    input.chars()
        .flat_map(char::to_lowercase)
        .map(atbash)
        .flat_map(|c| c)
        .fold(initial_encoder, |encoder, c| encoder.encode_next(c))
        .text
}

pub fn decode(input: &str) -> String {
    input.chars()
        .flat_map(char::to_lowercase)
        .map(atbash)
        .flat_map(|c| c)
        .collect()
}
