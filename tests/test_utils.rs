extern crate rust_algorithms;

use rust_algorithms::utils::rand_array;

#[test]
fn test_rand_array() {
    let arr1 = rand_array(100);
    assert_eq!(100, arr1.len());

    // should default to size of 10
    let arr1 = rand_array(-1);
    assert_eq!(10, arr1.len());
}
