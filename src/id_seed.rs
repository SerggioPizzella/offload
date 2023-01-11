use std::sync::atomic::AtomicUsize;

pub struct IdSeed {
	pub seed: AtomicUsize
}

impl IdSeed {
	pub fn new(value: usize) -> IdSeed {
		IdSeed { seed: AtomicUsize::new(value) }
	}
}
