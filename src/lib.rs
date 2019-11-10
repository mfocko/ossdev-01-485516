#[macro_use]
extern crate text_io;

use std::io::{self, Write};
use std::vec::Vec;

/// Checks if given number is prime.
///
/// # Arguments
///
/// * `number: u32` - number to check
///
/// # Return
///
/// `true` if `number` is a prime, `false` otherwise
///
/// # Example
/// ```
/// assert_eq!(ossdev_01_485516::is_prime(2), true);
/// assert_eq!(ossdev_01_485516::is_prime(3), true);
/// assert_eq!(ossdev_01_485516::is_prime(4), false);
/// assert_eq!(ossdev_01_485516::is_prime(0), false);
/// ```
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
/// assert_eq!(ossdev_01_485516::get_primes(1, 10), vec![2, 3, 5, 7]);
/// assert_eq!(ossdev_01_485516::get_primes(1, 1), vec![])
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
pub fn read_unsigned(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let result: u32 = read!();

    return result;
}
