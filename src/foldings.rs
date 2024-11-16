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

	let d: Vec<Vec<Vec<usize>>> = (0..=n_dim)
		.map(|i| {
			(0..=n)
				.map(|l| (0..=l).map(|m| d_fn(i, l, m, shape)).collect())
				.collect()
		})
		.collect();

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
					gaps[gg..gg + l].copy_from_slice(&(0..l).collect::<Vec<usize>>());
					gg += l;
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

fn p_fn(i: usize, shape: &[usize]) -> usize {
	shape.iter().take(i).fold(1, |a, b| a * b)
}

fn c_fn(i: usize, m: usize, shape: &[usize]) -> usize {
	(m - 1) / p_fn(i - 1, shape) - ((m - 1) / p_fn(i, shape)) * shape[i - 1] + 1
}

fn d_fn(i: usize, l: usize, m: usize, shape: &[usize]) -> usize {
	if i == 0 || l == 0 || m == 0 {
		return 0;
	}
	if c_fn(i, l, shape) & 1 == c_fn(i, m, shape) & 1 {
		if c_fn(i, m, shape) == 1 {
			m
		} else {
			m - p_fn(i - 1, shape)
		}
	} else {
		if c_fn(i, m, shape) == shape[i - 1] || m + p_fn(i - 1, shape) > l {
			m
		} else {
			m + p_fn(i - 1, shape)
		}
	}
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
