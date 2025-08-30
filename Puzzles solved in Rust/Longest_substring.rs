use std::collections::HashSet;
fn main() {
	println!("{}", longest_substring("225424272163254474441338664823"));
}

fn longest_substring(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
	let mut start: usize = 0;
	let mut max_len: usize = 0;
	let mut max_start: usize = 0;
	let mut stack: HashSet<char> = HashSet::new();
	
	for (i, &c) in chars.iter().enumerate() {
		while stack.contains(&c) {
			start += 1;
			stack.remove(&chars[start]);
		}
		stack.insert(c);
		
		let current_len: usize = i - start + 1;
		if max_len < current_len {
			max_len = current_len;
			max_start = start;
		}
	}
	return chars[max_start..max_start + max_len].iter().collect();
}
