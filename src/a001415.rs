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

pub fn foldings(dim: &[usize]) -> usize {
	let n = dim.iter().fold(1, |a, b| a * b) as usize;

	let mut a = vec![0; n + 1];
	let mut b = vec![0; n + 1];
	let mut count = vec![0; n + 1];
	let mut gapter = vec![0];
	let mut gap = vec![0; n * n + 1];

	let mut p = vec![0; dim.len() + 1];
	let mut c = vec![vec![0; n + 1]; dim.len() + 1];
	let mut d = vec![vec![vec![0; n + 1]; n + 1]; dim.len() + 1];

	p[0] = 1;
	for i in 1..=dim.len() {
		p[i] = p[i - 1] * dim[i - 1];
	}

	for i in 1..=dim.len() {
		for m in 1..=n {
			c[i][m] = (m - 1) / p[i - 1] - ((m - 1) / p[i]) * dim[i - 1] + 1;
		}
	}

	for i in 1..=dim.len() {
		for l in 1..=n {
			for m in 1..=l {
				let delta = c[i][l] - c[i][m];
				d[i][l][m] = if delta & 1 == 0 {
					if c[i][m] == 1 {
						m
					} else {
						m - p[i - 1]
					}
				} else {
					if c[i][m] == dim[i - 1] || m + p[i - 1] > l {
						m
					} else {
						m + p[i - 1]
					}
				}
			}
		}
	}

	let mut g = 0;
	let mut n_foldings = 0;

	while !gapter.is_empty() {
		let l = gapter.len();
		if l <= 1 || b[0] == 1 {
			if l > n {
				n_foldings += n;
			} else {
				let mut dd = 0;
				let mut gg = gapter.last().unwrap().to_owned();
				g = gg;

				for i in 1..=dim.len() {
					if d[i][l][l] == l {
						dd += 1;
					} else {
						let mut m = d[i][l][l];
						while m != l {
							gap[gg] = m;
							if count[m] == 0 {
								gg += 1;
							}
							count[m] += 1;
							m = d[i][l][b[m]];
						}
					}
				}

				if dd == dim.len() {
					for m in 0..l {
						gap[gg] = m;
						gg += 1;
					}
				}

				for j in g..gg {
					gap[g] = gap[j];
					if count[gap[j]] == dim.len() - dd {
						g += 1;
					}
					count[gap[j]] = 0;
				}
			}
		}

		while gapter.last().is_some_and(|&x| x == g) {
			gapter.pop();
			b[a[gapter.len()]] = b[gapter.len()];
			a[b[gapter.len()]] = a[gapter.len()];
		}

		if !gapter.is_empty() {
			let l = gapter.len();
			g -= 1;
			a[l] = gap[g];
			b[l] = b[a[l]];
			b[a[l]] = l;
			a[b[l]] = l;
			gapter.push(g);
		}
	}

	n_foldings
}
