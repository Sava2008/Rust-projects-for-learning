fn main() -> () {
    println!("{}", hex_lattice(1));
    println!("{}", hex_lattice(7));
    println!("{}", hex_lattice(37));
    println!("{}", hex_lattice(19));
    println!("{}", hex_lattice(28));
    println!("{}", hex_lattice(100));
    println!("{}", hex_lattice(91));
}

pub fn hex_lattice(num: i16) -> String {
    let sqrt_discriminant = ((9 + 12 * (num - 1)) as f32).sqrt();
    
    if sqrt_discriminant.fract() != 0.0 {
        return "invalid num parameter".to_string();
    }
    
    let n = ((3.0 + sqrt_discriminant) / 6.0) as i16;

    let mut decreasing: bool = false;
    let mut addition: i16 = 0;

    let mut hexagon: String = String::new();

    for _ in 0..(n + (n - 1)) {
        if addition == n - 1 {
            decreasing = true;
        }
        hexagon.push_str(&"o".repeat((n + addition) as usize));
        hexagon.push('\n');
        match decreasing {
            true => { addition -= 1; },
            false => { addition += 1; },
        }
    }
    return hexagon;
}
