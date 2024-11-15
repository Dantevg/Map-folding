use crate::foldings;

#[derive(Default)]
pub struct A001415 {
	n: usize,
}

impl Iterator for A001415 {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.n += 1;

		if self.n == 1 {
			Some(1)
		} else {
			Some(foldings(&[self.n - 1, 2]))
		}
	}
}

#[derive(Default)]
pub struct A001418 {
	n: usize,
}

impl Iterator for A001418 {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.n += 1;

		if self.n == 1 {
			Some(1)
		} else {
			Some(foldings(&[self.n, self.n]))
		}
	}
}
