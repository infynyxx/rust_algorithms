extern crate rust_algorithms;
use rust_algorithms::sorting::selection_sort::sort;
use rust_algorithms::utils::rand_array;

#[test]
fn test_selection_sort() {
    let vector1 = vec![100i, 90i, 2i, 77i, 55i];
    let result1 = sort(vector1);
    assert_eq!(vec![2i, 55i, 77i, 90i, 100i], result1);

    let vector2 = vec![9i, 1i, 2i, 5i];
    let result2 = sort(vector2);
    assert_eq!(vec![1i, 2i, 5i, 9i], result2);

    test_sort(10);
    test_sort(101);
}

fn test_sort(size: int) {
    let mut vector = rand_array(size);
    let result = sort(vector.clone());
    vector.sort_by(|a, b| a.cmp(b)); // ascending order
    assert_eq!(vector, result);
}
