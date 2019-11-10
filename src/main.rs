#![crate_name = "ossdev_01_485516"]

use ossdev_01_485516::{get_primes, read_unsigned};

fn main() {
    let lower = read_unsigned("Insert lower bound: ");
    let upper = read_unsigned("Insert upper bound: ");
    let primes = get_primes(lower, upper);

    println!("\nPrimes:");
    primes.iter().for_each(|x| print!("{} ", x));
    println!();
}
