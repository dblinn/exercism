pub fn raindrops(number: u32) -> String {
    let factor_sounds = [
        (number % 3 == 0, "Pling"),
        (number % 5 == 0, "Plang"),
        (number % 7 == 0, "Plong"),
    ];

    if factor_sounds.iter().any(|&(should_play, _)| should_play) {
        composite_sounds(&factor_sounds)
    }
    else {
        number.to_string()
    }
}

fn composite_sounds(factor_sounds: &[(bool, &str)]) -> String {
    factor_sounds.iter().fold(String::new(), |composite_sound, &(should_play, factor_sound)|
        if should_play { composite_sound + factor_sound }
        else { composite_sound }
    )
}
