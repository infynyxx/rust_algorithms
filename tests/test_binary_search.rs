extern crate rust_algorithms;
use rust_algorithms::binary_search::search;

#[test]
fn test_binary_search_found() {
    let vector = rust_algorithms::utils::rand_array(150);
    let item1 = vector[149];
    let index1 = search(vector.clone(), item1);
    assert!(index1 >= 0, "key {} should exist with index {}", item1, index1);

    let item2 = vector[1];
    let index2 = search(vector.clone(), item2);
    assert!(index2 >= 0, "key {} should exist with index {}", item2, index2);
}

#[test]
fn test_binary_search_not_found() {
    let vector = vec![100i, 90i, 2i, 77i, 55i];
    let item1 = 56i;
    let index1 = search(vector.clone(), item1);
    assert!(index1 == -1, "key {} should not exist", item1);

    let item2 = 11i;
    let index2 = search(vector.clone(), item2);
    assert!(index2 == -1, "key {} should not exist", item2);
}
