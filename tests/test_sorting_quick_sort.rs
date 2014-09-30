extern crate rust_algorithms;
use rust_algorithms::sorting::quick_sort::QuickSort;
use rust_algorithms::sorting::Sorting;

#[test]
fn test_quick_sort() {
    let vector1 = QuickSort{ input_array: vec![100i, 90i, 2i, 77i, 55i] };
    let result1 = vector1.sort();
    assert_eq!(vec![2i, 55i, 77i, 90i, 100i], result1);
}
