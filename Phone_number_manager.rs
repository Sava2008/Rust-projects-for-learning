use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut phone_number_book: HashMap<String, usize> = HashMap::new();
    
    loop {
        let mut mode: String = String::new();
        print!("Select mode (add, remove, search, quit): ");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut mode);
        let mode = mode.trim().to_string();
        
        if mode == "quit" {
            break;
        }
        else if mode == "add" {
            loop {
                let mut name = String::new();
                print!("Add the name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name).expect("Invalid name");
                let name: &str = name.trim();

                if name.is_empty() {
                    break;
                }
                let mut tel_string: String = String::new();

                print!("Add the phone number: ");
                io::stdout().flush().unwrap();

                let _ = io::stdin().read_line(&mut tel_string);

                let tel: usize = tel_string.trim().parse().unwrap_or(0);
                phone_number_book.insert(name.to_string(), tel);
            }
        }
        else if mode == "remove" {
            loop {
                let mut name: String = String::new();
                print!("Enter a name: ");
                io::stdout().flush().unwrap();
                let _ = io::stdin().read_line(&mut name);
                let name: String = name.trim().to_string();
                if name.is_empty() {
                    break
                }
                phone_number_book.remove(&name);
            }
        }
        else if mode == "search" {
            loop {
                let mut name: String = String::new();
                print!("Enter a name: ");
                io::stdout().flush().unwrap();
                let _ = io::stdin().read_line(&mut name);
                let name: String = name.trim().to_string();
                if name.is_empty() {
                    break;
                }
                println!("{}", phone_number_book.get(&name).unwrap());
            }
        }
    }
    println!("{phone_number_book:?}");
}
