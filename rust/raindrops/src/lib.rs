pub fn raindrops(number: u32) -> String {
    let sound = [(3, "Pling"), (5, "Plang"), (7, "Plong")].iter()
        .map(|&(factor, sound)| if number % factor == 0 { sound } else { "" } )
        .collect::<String>();
    if sound.is_empty() { number.to_string() } else { sound }
}
