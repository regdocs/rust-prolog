use crate::error_msg;
use std::io::Write;

pub fn run_module() -> bool {
    println!("This is fibonacci::run_module() ->");
    println!(" 1. Find Nth Fibonacci number");
    println!(" 2. Print first N Fibonacci numbers");
    print!("Enter choice or `b` to go back: ");
    std::io::stdout().flush().expect(error_msg::FAILED_FLUSH);

    let mut choice: String = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect(error_msg::FAILED_READ_LINE);

    let choice: &str = choice.trim();

    if choice == "b" {
        println!("[GOING BACK...]");
        return false;
    } else if choice != "1" && choice != "2" {
        print!("Wrong choice, try again!\n\n");
        return true;
    }

    print!("Enter N (>= 1): ");
    std::io::stdout().flush().expect(error_msg::FAILED_FLUSH);
    let mut n: String = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect(error_msg::FAILED_READ_LINE);

    let n: u32 = n
        .trim()
        .parse::<u32>()
        .expect(error_msg::FAILED_TO_PARSE_INTEGER);

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 1;

    if n <= 0 {
        panic!("N cannot be a non-positive integer");
    } else {
        if choice == "2" {
            print!("[ {} ", c);
        }
        
        for _ in 1..n {
            c = a + b;
            a = b;
            b = c;

            if choice == "2" {
                print!("{} ", c);
            }
        }
    }

    match choice {
        "1" => println!("{}-th Fibonacci number is {}", n, c),
        "2" => println!("]"),
        _ => {},
    }

    true
}
