use std::collections::HashSet;

fn main() {
	println!("{}", is_valid_hex("#454545"));
	println!("{}", is_valid_hex("#c6c7d2"));
	println!("{}", is_valid_hex("#0F4F9F"));
	println!("{}", is_valid_hex("#G507D2"));
	println!("{}", is_valid_hex("#n54545"));
	println!("{}", is_valid_hex("4545454"));
	println!("{}", is_valid_hex("#45454"));
}

fn is_valid_hex(hex: &str) -> bool {
	let valid_hex_chars: HashSet<char> = ('a'..='f').chain('0'..='9')
	.collect();
	
	if hex.as_bytes()[0] != b'#' || hex.chars().count() != 7 {
		return false;
	}
	for c in hex[1..].to_lowercase().chars() {
		if !valid_hex_chars.contains(&c) {
			return false;
		}
	}
	return true;
}
