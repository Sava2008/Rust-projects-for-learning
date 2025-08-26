fn main() {
	let nums: Vec<i32> = vec![1, 2, 5, 19];
	let ops: Vec<char> = vec!['+', '-', '-'];
	let nums_len: usize = nums.len();
	let ops_len: usize = ops.len();
	let expr: MathExpression = MathExpression {
		numbers: nums, operators: ops, numbers_len: nums_len, operators_len: ops_len,
	};
	println!("{:?}", expr.construct());
	println!("{}", expr.calculate());
	println!("{:?}", MathExpression::from("4+5-7-3+100"));
}

#[derive(Debug)]
struct MathExpression {
	numbers: Vec<i32>,
	operators: Vec<char>,
	numbers_len: usize,
	operators_len: usize,
}

impl<'a> From<&'a str> for MathExpression {
	fn from(item: &'a str) -> Self {
		let mut numbers: Vec<i32> = Vec::new();
		let mut operators: Vec<char> = Vec::new();
		
		let mut temp_num: String = String::new();
		for (i, c) in item.chars().enumerate() {
			if "+-*/%^".contains(c) {
				numbers.push(temp_num.trim().parse::<i32>().unwrap());
				operators.push(c);
				temp_num.clear();
			} else {
				temp_num.push(c);
				if i == item.len() - 1 {
					numbers.push(temp_num.trim().parse::<i32>().unwrap());
				}
			}
		}
		let nums_len = numbers.len();
		let ops_len = operators.len();

		let final_object: MathExpression = MathExpression { numbers: numbers, 
		operators: operators, numbers_len: nums_len, 
		operators_len: ops_len };
		
		return if final_object.validate() { final_object } else { panic!
																  ("Invalid math expression")
																};
	}
}

impl MathExpression {
	fn validate(&self) -> bool {
		const VALID_OPERATORS: [char; 2] = ['+', '-']; // will grow in future
		
		for op in &self.operators {
			if !VALID_OPERATORS.contains(op) {
				return false;
			}
		}
		return self.operators_len + 1 == self.numbers_len;
	}
	
	fn construct(&self) -> Result<String, String> {
		if !self.validate() {
			panic!("Invalid expression");
		}
		
		let mut expr: String = self.numbers[0].to_string();
		for (idx, num) in self.numbers[1..].iter().enumerate() {
			expr.push(self.operators[idx]);
			expr.push_str(&num.to_string());
		}
		return Ok(expr);
	}
	
	fn calculate(self) -> i32 {
		fn single_operation(x: i32, y: i32, op: char) -> Option<i32> {
			match op {
				'+' => return Some(x + y),
				'-' => return Some(x - y),
			 /* '*' => return x * y,
				'/' => return x / y,
				'%' => return x % y, */
				_ => return None,
			}
		}
		let mut accumulative: i32 = 0;
		
		let mut index: usize = 0;
		loop {
			if index == self.operators_len {
				break;
			}
			if index == 0 {
				accumulative += single_operation(self.numbers[index],
				self.numbers[index + 1], self.operators[index])
				.expect("Could not process the operation");
			} else {
			accumulative = single_operation(accumulative, 
											self.numbers[index + 1],
											self.operators[index])
											.expect("Could not process the operation");
			}
			index += 1;
		}
		return accumulative;
	}
}