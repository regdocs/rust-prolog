use std::io::Write;
use crate::error_msg;

fn to_celsius(fahrenheit: f64) -> f64 {
    5.0 / 9.0 * (fahrenheit - 32.0)
}

fn to_fahrenheit(celsius: f64) -> f64 {
    9.0 / 5.0 * celsius + 32.0
}

pub fn run_module() -> bool {
    println!("This is degc_degf::run_module() ->");
    println!(" 1. Convert deg Fahrenheit to deg Celsius");
    println!(" 2. Convert deg Celsius to deg Fahrenheit");
    print!("Enter choice or `b` to go back: ");
    std::io::stdout().flush().unwrap();

    let mut choice: String = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect(error_msg::FAILED_READ_LINE);

    let choice: &str = choice.trim();

    match choice {
        "1" => {
            print!("Enter value in deg Fahrenheit: ");
            std::io::stdout().flush().unwrap();
            let mut degf: String = String::new();
            std::io::stdin()
                .read_line(&mut degf)
                .expect(error_msg::FAILED_READ_LINE);
            let degf = degf.trim().parse::<f64>().expect(error_msg::FAILED_TO_PARSE_INTEGER);
            let degc = to_celsius(degf);
            println!(
                "{} deg Fahrenheit is equivalent to {} deg Celsius",
                degf, degc
            );
            true
        }

        "2" => {
            print!("Enter value in deg celsius: ");
            std::io::stdout().flush().unwrap();
            let mut degc: String = String::new();
            std::io::stdin()
                .read_line(&mut degc)
                .expect(error_msg::FAILED_READ_LINE);
            let degc = degc.trim().parse::<f64>().expect(error_msg::FAILED_TO_PARSE_INTEGER);
            let degf = to_fahrenheit(degc);
            println!(
                "{:.1} deg Celsius is equivalent to {:.1} deg Fahrenheit",
                degc, degf
            );
            true
        }

        "b" => {
            println!("[GOING BACK...]");
            false
        }
        
        _ => {
            println!("Wrong choice, try again!");
            true
        }
    }
}
