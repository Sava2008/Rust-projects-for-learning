fn main() {
	/*task: Create a function to perform basic arithmetic operations that 
	includes addition, subtraction, multiplication and division on a string 
	number (e.g. "12 + 24" or "23 - 21" or "12 // 12" or "12 * 21").
	Here, we have 1 followed by a space, operator followed by another space 
	and 2. For the challenge, we are going to have only two numbers between 
	1 valid operator. The return value should be a number.*/
	
    let expressions: [&str; 7] = ["3 + 2", "9 - 5", "36 * 6", "180 / 9",
    "9 ** 3", "4 ... 1", "0 + 2 + 4"];

    for expression in expressions {
        match eval(expression) {
            Ok(answer) => println!("{answer}"),
            Err(message) => println!("{expression} evaluated in {message}"),
            _ => println!("Unknown"),
        }
    }
}

fn eval(expression: &str) -> Result<i32, String> {
    let listed_expression: Vec<&str> = expression.split_whitespace().collect::<Vec<&str>>();
    let len: u8 = listed_expression.len() as u8;
    if len != 3 {
        return Err(format!("{expression} must consist of exactly 3 parts, but {len} were provided. Example of a correct expression: 1 + 1"))
    }
    let first_num: i32 = listed_expression[0].parse().unwrap();
    let second_num: i32 = listed_expression[2].parse().unwrap();
    match listed_expression[1] {
        "+" => return Ok(first_num + second_num),
        "-" => return Ok(first_num - second_num),
        "*" => return Ok(first_num * second_num),
        "/" => { if second_num != 0 {
                return Ok(first_num / second_num);
            } else {
                   return Err("Can't divide by zero".to_string());
               }
        },
        "**" => return Ok(first_num.pow(second_num as u32)),
        operator => return Err(format!("unknown operator {operator}")),
    }
}