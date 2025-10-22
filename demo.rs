use k_bonacci::KBonacci;

fn main() {
	// prints powers of 2 :o
	for k in 2..10 {
		print!("{}, ", KBonacci::new(k).nth(2*k - 1).unwrap());
	} 
	println!();
	println!();

	// prints k-bonacci sequences from the point they're unique
	for k in 2..10 {
		for kibble in KBonacci::new(k).skip(2*k).take(10) {
				print!("{}, ", kibble);
			}
		println!();
	}
}
