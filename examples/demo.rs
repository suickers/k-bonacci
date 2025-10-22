use k_bonacci::KBonacci;

fn main() {
	let fib = KBonacci::new(2);

	for fibble in fib.take(15) {
		println!("{}", fibble);
	}
}
