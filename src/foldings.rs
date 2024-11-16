#[derive(Debug, Default, Clone, Copy)]
struct Leaf {
	above: usize,
	below: usize,
	count: usize,
}

pub fn foldings(shape: &[usize]) -> usize {
	let n = shape.iter().fold(1, |a, b| a * b) as usize;
	let n_dim = shape.len();

	let mut leafs: Vec<Leaf> = vec![Leaf::default(); n + 1];

	let mut gapter = vec![0];
	let mut gaps = vec![0; n * n + 1];

	let mut p = vec![0; n_dim + 1];
	let mut c = vec![vec![0; n + 1]; n_dim + 1];
	let mut d = vec![vec![vec![0; n + 1]; n + 1]; n_dim + 1];

	p[0] = 1;
	for i in 1..=n_dim {
		p[i] = p[i - 1] * shape[i - 1];
	}

	for i in 1..=n_dim {
		for m in 1..=n {
			c[i][m] = (m - 1) / p[i - 1] - ((m - 1) / p[i]) * shape[i - 1] + 1;
		}
	}

	for i in 1..=n_dim {
		for l in 1..=n {
			for m in 1..=l {
				d[i][l][m] = if c[i][l] & 1 == c[i][m] & 1 {
					if c[i][m] == 1 {
						m
					} else {
						m - p[i - 1]
					}
				} else {
					if c[i][m] == shape[i - 1] || m + p[i - 1] > l {
						m
					} else {
						m + p[i - 1]
					}
				}
			}
		}
	}

	let mut g = 0; // number of actual gaps for leaf l + last gapter
	let mut n_foldings = 0;

	while !gapter.is_empty() {
		let l = gapter.len();
		if l <= 1 || leafs[0].below == 1 {
			if l > n {
				n_foldings += n;
			} else {
				g = gapter.last().unwrap().to_owned();
				let mut dd = 0; // number of sections in which leaf l is unconstrained
				let mut gg = g; // number of possible gaps for leaf l + last gapter

				for dim in d.iter().skip(1) {
					if dim[l][l] == l {
						dd += 1;
					} else {
						let mut m = dim[l][l];
						while m != l {
							gaps[gg] = m;
							if leafs[m].count == 0 {
								gg += 1;
							}
							leafs[m].count += 1;
							m = dim[l][leafs[m].below];
						}
					}
				}

				if dd == n_dim {
					for m in 0..l {
						gaps[gg] = m;
						gg += 1;
					}
				}

				for j in g..gg {
					gaps[g] = gaps[j];
					if leafs[gaps[j]].count == n_dim - dd {
						g += 1;
					}
					leafs[gaps[j]].count = 0;
				}
			}
		}

		while gapter.last().is_some_and(|&x| x == g) {
			gapter.pop();
			// remove from linked list?
			let l = leafs[gapter.len()];
			leafs[l.above].below = l.below;
			leafs[l.below].above = l.above;
		}

		if !gapter.is_empty() {
			g -= 1;
			// insert in linked list?
			leafs[gapter.len()].above = gaps[g];
			leafs[gapter.len()].below = leafs[gaps[g]].below;

			let l = leafs[gapter.len()];
			leafs[l.above].below = gapter.len();
			leafs[l.below].above = gapter.len();

			gapter.push(g);
		}
	}

	n_foldings
}

#[cfg(test)]
mod tests {
	use crate::iters::{A001415, A001418};

	// Number of ways of folding a 2 X n strip of stamps.
	const A001415: [usize; 19] = [
		1,
		2,
		8,
		60,
		320,
		1980,
		10512,
		60788,
		320896,
		1787904,
		9381840,
		51081844,
		266680992,
		1429703548,
		7432424160,
		39409195740,
		204150606976,
		1073644675448,
		5545305620064,
	];

	// Number of ways of folding an n X n sheet of stamps.
	const A001418: [usize; 7] = [1, 8, 1368, 300608, 186086600, 123912532224, 129950723279272];

	#[test]
	fn test_a001415() {
		for (&expected, actual) in A001415.iter().zip(A001415::default()).take(10) {
			assert_eq!(expected, actual, "expected {expected} = actual {actual}");
		}
	}

	#[test]
	fn test_a001418() {
		for (&expected, actual) in A001418.iter().zip(A001418::default()).take(4) {
			assert_eq!(expected, actual, "expected {expected} = actual {actual}");
		}
	}
}
