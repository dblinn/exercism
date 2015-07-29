fn sum(target: i32) -> i32 {
	(target * (target + 1)) / 2
}

pub fn square_of_sum(target: i32) -> i32 {
	let sum = sum(target);
	sum * sum
}

pub fn sum_of_squares(target: i32) -> i32 {
	sum(target) * (1 + 2 * target) / 3
}

pub fn difference(target: i32) -> i32 {
	square_of_sum(target) - sum_of_squares(target)
}