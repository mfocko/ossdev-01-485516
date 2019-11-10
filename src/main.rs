#![crate_name = "ossdev_01_485516"]

use std::io::{self, Write};
#[macro_use]
extern crate text_io;

/// Checks if given number is prime.
///
/// # Arguments
///
/// * `number: u32` - number to check
///
/// # Return
///
/// `true` if `number` is a prime, `false` otherwise
pub fn is_prime(number: u32) -> bool {
    match number {
        num if num < 2 => false,
        2 => true,
        num if num % 2 == 0 => false,
        num => !(3..=(num as f32).sqrt() as u32)
            .step_by(2)
            .any(|div| num % div == 0),
    }
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
pub fn print_primes(lower: u32, upper: u32) {
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
