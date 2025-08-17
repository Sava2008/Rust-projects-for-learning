fn main() {
    println!("{}", my_pow(2.00000, -2147483648));
    println!("{}", my_pow(3.5, 7));
    println!("{}", my_pow(9.0, 6));
    println!("{}", my_pow(-50.0, 3));
    println!("{}", my_pow(-1.0, 6));
}

pub fn my_pow(x: f64, n: i32) -> f64 {
    let n: i64 = n as i64;
    let mut result: f64 = 1.0;
    let mut base: f64 = x;
    let mut pow = n.abs();
    while pow > 0 {
        if pow % 2 == 1 {
            result *= base;
        }
        base *= base;
        pow /= 2;
    }

    match n.cmp(&0) {
        std::cmp::Ordering::Equal => return 1.0,
        std::cmp::Ordering::Less => return 1.0 / result,
        std::cmp::Ordering::Greater => return result,
    }
}