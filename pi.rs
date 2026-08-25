fn main() {
	let mut sum: f64 = 0.0;
	let mut sign: f64 = 1.0;
	let mut n: u64 = 1;

	loop {
		sum += sign / n as f64;
		let pi_approx = 4.0 * sum;

		// Format to 10 decimal places and take the very last digit
		let s = format!("{:.10}", pi_approx);
		let last_digit = s.chars().last().unwrap();

		println!("{}", last_digit);

		n += 2;
		sign = -sign;
	}
}