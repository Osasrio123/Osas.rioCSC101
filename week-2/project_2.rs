fn main() {
	let t1:f64 = 2.0;
	let t2:f64 = 450_000.0;
	let m1:f64 = 1.0;
	let m2:f64 = 1_500_000.0;
	let h1:f64 = 3.0;
	let h2:f64 = 750_000.0;
	let d1:f64 = 3.0;
	let d2:f64 = 2_850_000.0;
	let a1:f64 = 1.0;
	let a2:f64 = 250_000.0;

	// sum
	let su = (t1 * t2) + (m1 * m2) + (h1 * h2) + (d1 * d2) + (a1 * a2);
	println!("Sum is {}", su);
	let qty = t1 + m1 + h1 + d1 + a1;

	// average
	let a = su/qty; 
	println!("Average is {}", a);
}