// pub fn sum_of_multiples(range_high : u32, factors: &Vec<u32>) -> u32 {
//     let mut sum = 0;
//
//     for i in 1..range_high {
//         sum += if factors.iter().any(|f| i % f == 0) { i } else { 0 }
//     }
//
//     sum
// }

pub fn sum_of_multiples(range_high : u32, factors: &Vec<u32>) -> u32 {
    (1..range_high).fold(0, |sum, i| {
        sum + if factors.iter().any(|f| i % f == 0) { i } else { 0 }
    })
}
