use std::io::{self, Write};
#[macro_use] extern crate text_io;

/// Checks if given number is prime.
///
/// # Arguments
///
/// * `number: u32` - number to check
///
/// # Returns
/// `true` if `number` is a prime, `false` otherwise
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

/// Prints out primes in given interval.
///
/// # Arguments
///
/// * `lower: u32` - lower bound of an interval (inclusive)
/// * `upper: u32` - upper bound of an interval (inclusive)
///
/// # Example
///
/// ```
/// print_primes(1, 10)
/// >>> 2 3 5 7 9
/// ```
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
