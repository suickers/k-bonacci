use std::collections::VecDeque;
use num::bigint::BigUint; 
use num_traits::{Zero, One};

pub struct KBonacci {
	window: VecDeque<BigUint>,
}

impl KBonacci {
	pub fn new(k: usize) -> Self {
		assert!(k >= 2, "K must be greater than 1.");
		
		let mut vec = VecDeque::new();
		for _ in 0..k-1 {
			vec.push_front(BigUint::zero());
		}
		vec.push_back(BigUint::one());

		Self { window: vec }
	}
}

impl Iterator for KBonacci {
	type Item = BigUint; 

	fn next(&mut self) -> Option<Self::Item> {
		let sum = self.window.iter().sum();
		let x = self.window.pop_front().unwrap();
		self.window.push_back(sum);

		Some(x)
	}
}
