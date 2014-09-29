extern crate rust_algorithms;
use rust_algorithms::algorithms::number::gcd;

#[test]
fn test_gcd() {
    let result = gcd(25i, 15i);
    assert_eq!(5i, result);
}
