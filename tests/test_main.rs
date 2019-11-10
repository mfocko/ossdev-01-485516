use ossdev_01_485516::*;

#[test]
fn test_basic_primes() {
    vec![2, 3, 5, 7, 11, 13, 17, 19]
        .iter()
        .for_each(|x| assert_eq!(is_prime(*x as u32), true))
}

#[test]
fn test_not_primes() {
    vec![4, 6, 9, 15, 27, 81, 64]
        .iter()
        .for_each(|x| assert_ne!(is_prime(*x as u32), true))
}

#[test]
fn test_non_trivial_not_primes() {
    vec![9059*6841, 2617*9949, 9887*9839, 4451 * 2, 4093 * 4099]
        .iter()
        .for_each(|x| assert_ne!(is_prime(*x as u32), true))
}

#[test]
fn test_get_few_primes() {
    assert_eq!(get_primes(1, 10), vec![2, 3, 5, 7])
}

#[test]
fn test_get_more_primes() {
    assert_eq!(get_primes(1000, 1050), vec![1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049])
}

#[test]
fn test_get_no_primes() {
    assert_eq!(get_primes(20, 22) ,vec![])
}
