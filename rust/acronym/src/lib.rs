struct AcronymTracker {
    prev: char,
}

impl AcronymTracker {
    pub fn track_next(&mut self, next: char) -> Option<char> {
        let result =
            if Self::alphabetic_rules(self.prev, next) || Self::capitalization_rules(self.prev, next)
                { Some(next) } else { None };

        self.prev = next;
        result
    }

    fn alphabetic_rules(prev: char, next: char) -> bool {
        !prev.is_alphabetic() && next.is_alphabetic()
    }

    fn capitalization_rules(prev: char, next: char) -> bool {
        prev.is_lowercase() && next.is_uppercase()
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let mut tracker = AcronymTracker { prev: ' ' };

    phrase.chars()
        .flat_map(|c| tracker.track_next(c) )
        .flat_map(|c| c.to_uppercase() )
        .collect()
}
