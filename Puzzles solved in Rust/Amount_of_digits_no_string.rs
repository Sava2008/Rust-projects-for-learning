fn main() {
    println!("{}", amount_of_digits(-1000 as isize));
}

fn amount_of_digits(mut num: isize) -> u8 {
    if num == 0 {
        return 1;
    }
    let mut count: u8 = 0;
    while num != 0 {
        num /= 10;
        count += 1;
    }
    return count;
}
