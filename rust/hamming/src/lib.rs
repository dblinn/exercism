pub fn hamming_distance(a: &str, b: &str) -> Result<usize, usize> {
    if a.len() == b.len() {
        Ok(compute_hamming_distance(a, b))
    } else {
        Err(0)
    }
}

fn compute_hamming_distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).fold(0, |sum, (ca, cb)| {
        if ca == cb { sum } else { sum + 1 }
    })
}
