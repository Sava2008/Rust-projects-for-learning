fn main() {
    println!("{}", lcm(8, 38));
    println!("{}", lcm2(8, 38));
}

fn lcm(x: i32, y: i32) -> i32 {
    fn gcd(x: i32, y: i32) -> i32 {
        let mut minimal: i32 = x.min(y);
        while minimal > 0 {
            if (x % minimal == 0) & (y % minimal == 0) {
                return minimal;
            }
            minimal -= 1;
        }
        return 1;
    }
    return (x * y).abs() / gcd(x, y);
}

fn lcm2(x: i32, y: i32) -> i32 {
    fn gcd2(x: i32, y: i32) -> i32 {
        if y == 0 {
            return x;
        }
        return gcd2(y, x % y);
    }
    return (x * y).abs() / gcd2(x, y);
}