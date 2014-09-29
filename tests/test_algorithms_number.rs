extern crate rust_algorithms;
use rust_algorithms::algorithms::number::gcd;

#[test]
fn test_gcd() {
    let result1 = gcd(25i, 15i);
    assert_eq!(5i, result1);

    let result2 = gcd(4i, 2i);
    assert_eq!(2i, result2);

    let result3 = gcd(6i, 3i);
    assert_eq!(3i, result3);
}
