use std::collections::HashMap;
fn main() -> () {
	println!("{:?}", integer_to_roman(352));
	println!("{:?}", integer_to_roman(600));
	println!("{:?}", integer_to_roman(2174));
	println!("{:?}", integer_to_roman(91));
	println!("{:?}", integer_to_roman(3888));
	println!("{:?}", integer_to_roman(4999));
	println!("{:?}", integer_to_roman(3999));
}

pub fn integer_to_roman<'a>(num: u16) -> Result<String, &'a str> {
	if num > 3999 || num == 0 {
		return Err("the num should be between 1 and 3999 inclusive");
	}
	let ranks: HashMap<u8, (char, char, char)> = HashMap::from(
		[(0, ('I', 'V', 'X')), (1, ('X', 'L', 'C')),
		(2, ('C', 'D', 'M')), (3, ('M', '_', '_'))]
	);
	let string_num = num.to_string();
	let reversed_num = string_num.chars().rev()
	.enumerate().map(|(i, c)| (i as u8, c.to_digit(10).unwrap() as usize));
	let mut roman_number: String = String::with_capacity(15);
	for (index, c) in reversed_num {
		let field: (char, char, char) = *ranks.get(&index).unwrap();
		match c {
			1..=3 => { roman_number.push_str(&field.0.to_string().repeat(c)); },
			5 => { roman_number.push(field.1); },
			4 => { roman_number.push_str(&format!("{}{}", field.1, field.0)); },
			9 => { roman_number.push_str(&format!("{}{}", field.2, field.0)); },
			6..=8 => { roman_number.push_str(&format!("{}{}", 
			field.0.to_string().repeat(c - 5), field.1)); },
			_ => (),
		}
	}
	return Ok(roman_number.chars().rev().collect());
}
