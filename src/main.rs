use std::io::{self, Write};
#[macro_use] extern crate text_io;

fn is_prime(number: u32) -> bool {
    return match number {
        1 => false,
        2 => true,
        number if number % 2 == 0 => false,
        num => {
            for div in (3..=(num / 2)).step_by(2) {
                if num % div == 0 {
                    return false;
                }
            }
            return true;
        }
    };
}

fn print_primes(lower: u32, upper: u32) {
    println!();
    println!("Primes:");

    for number in lower..=upper {
        if is_prime(number) {
            print!("{} ", number);
        }
    }

    println!();
}

fn main() {
    print!("Insert lower bound: ");
    io::stdout().flush().unwrap();
    let lower: u32 = read!();

    print!("Insert upper bound: ");
    io::stdout().flush().unwrap();
    let upper: u32 = read!();

    print_primes(lower, upper);
}
