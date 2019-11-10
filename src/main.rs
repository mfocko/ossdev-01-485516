#![crate_name = "ossdev_01_485516"]

use std::io::{self, Write};
use std::vec::Vec;

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

/// Returns primes in a given interval.
///
/// # Arguments
///
/// * `lower: u32` - lower bound of an interval (inclusive)
/// * `upper: u32` - upper bound of an interval (inclusive)
///
/// # Return
///
/// `Vec` of all primes in given interval
///
/// # Example
///
/// ```
/// print_primes(1, 10)
/// >>> 2 3 5 7 9
/// ```
pub fn get_primes(lower: u32, upper: u32) -> Vec<u32> {
    (lower..=upper).filter(|x| is_prime(*x)).collect()
}

/// Reads unsigned 4B number from input.
///
/// # Arguments
///
/// * `prompt: &str` - string that is printed as prompt
///
/// # Return
///
/// `u32` number read from input
fn read_unsigned(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let result: u32 = read!();

    return result;
}

fn main() {
    let lower = read_unsigned("Insert lower bound: ");
    let upper = read_unsigned("Insert upper bound: ");
    let primes = get_primes(lower, upper);

    println!("\nPrimes:");
    primes.iter().for_each(|x| print!("{} ", x));
    println!();
}
