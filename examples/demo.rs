use k_bonacci::KBonacci;

fn main() {
	// prints k-bonacci sequences from the point they're unique
	for k in 2..10 {
		for kibble in KBonacci::new(k).skip(2*k).take(10) {
				print!("{}, ", kibble);
			}
		println!();
	}
}
