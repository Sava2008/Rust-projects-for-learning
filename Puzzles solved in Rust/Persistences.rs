fn main() {
	/* task: The additive persistence of an integer, n, is the number of times you have to replace n with the sum of its digits until n becomes a single digit integer.
	   The multiplicative persistence of an integer, n, is the number of times you have to replace n with the product of its digits until n becomes a single digit integer.
	   Create two functions that take an integer as an argument and:

	   1. Return its additive persistence.
	   2. Return its multiplicative persistence. 
	   
	   The n can't be negative */

	println!("{}", Persistence::new(548999999999).additive());
	println!("{}", Persistence::new(3489).multiplicative());
}

#[derive(PartialEq)]
enum Operation {
	Addition,
	Multiplication,
}

fn process(mut x: usize, mut rep_count: u16, caller: Operation) -> u16 {
	if x < 10 {
		return rep_count;
	}
	rep_count += 1;

	let mut digits: Vec<u16> = Vec::new();
	while x > 0 {
		digits.push((x % 10) as u16);
		x /= 10;
	}
			
	let result: u16 = if caller == Operation::Multiplication { digits.into_iter().product() } 
					 else { digits.into_iter().sum() };
	return process(result as usize, rep_count, caller);
}

struct Persistence {
	num: usize,
	replacements: u16,
}

impl Persistence {
	fn new(num: usize) -> Self {
		return Self { 
			num: num,
			replacements: 0, 
		};
	}
	fn additive(&mut self) -> u16 {
		return process(self.num, self.replacements, Operation::Addition);
	}
	
	fn multiplicative(&mut self) -> u16 {
		return process(self.num, self.replacements, Operation::Multiplication);
	}
}
