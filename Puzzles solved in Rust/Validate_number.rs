fn main() -> () {
    println!("{}", is_number("123")); // true
    println!("{}", is_number("-123")); // true
    println!("{}", is_number("+4385")); // true
    println!("{}", is_number("e356")); // false
    println!("{}", is_number("8e7")); // true
    println!("{}", is_number("19e-3")); // true
    println!("{}", is_number("32e.")); // false
    println!("{}", is_number("0.4")); // true
    println!("{}", is_number("12e9.7")); // false
    println!("{}", is_number("5a90")); // false
    println!("{}", is_number("76...313")); // false
    println!("{}", is_number(".00000")); // false
    println!("{}", is_number("7e-32")); // true
    println!("{}", is_number("100e+42")); // true
    println!("{}", is_number("5+4e+3")); // false
}

pub struct StateMachine {
    encountered_e: bool,
    check_e_followup: bool,
    encountered_dot: bool,
    encountered_operator: bool,
}
impl StateMachine {
    fn new() -> Self {
        return StateMachine {
            encountered_e: false,
            check_e_followup: false,
            encountered_dot: false,
            encountered_operator: false,
        };
    }
}

fn is_digit(digit: &char) -> bool {
    return matches!(digit, '0'..='9');
}

fn is_operator(digit: &char) -> bool {
    return matches!(digit, '+' | '-');
}

pub fn is_number(number: &str) -> bool {
    let mut num_iterator: std::str::Chars<'_> = number.chars();
    let first_digit: char = if let Some(x) = num_iterator.next() {
        x
    } else {
        return false;
    };

    let mut state: StateMachine = StateMachine::new();

    if is_digit(&first_digit) || is_operator(&first_digit) {
        for digit in num_iterator {
            if state.check_e_followup {
                if !is_digit(&digit) && !is_operator(&digit) {
                    return false;
                }
                state.check_e_followup = false;
            }
            match digit {
                '0'..='9' => continue,
                'e' | 'E' => {
                    if state.encountered_e {
                        return false;
                    }
                    state.encountered_e = true;
                    state.check_e_followup = true;
                    state.encountered_dot = true;
                }
                '.' => {
                    if state.encountered_dot {
                        return false;
                    }
                    state.encountered_dot = true;
                }
                '+' | '-' => {
                    if state.encountered_operator {
                        return false;
                    }
                    state.encountered_operator = true;
                }
                _ => return false,
            }
        }
        return true;
    }
    return false;
}
