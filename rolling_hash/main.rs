use std::hash::{Hash, SipHasher, Hasher};

fn main() {
	let data = vec![3,4,6,5,1,2];
	let newdata = vec![3,4,6,5,1,2];
	let mut delta = Vec::new();
	let mut iterator = 0;

	for point in newdata {
		if point.ne(data[iterator]) {
			delta.push([iterator, point]);
		}
		iterator += 1;
		println!("{}", point);
	}
}