extern crate rust_algorithms;
use rust_algorithms::sorting::selection_sort::SelectionSort;
use rust_algorithms::sorting::Sorting;

#[test]
fn test_selection_sort() {
    let vector1 = SelectionSort{ input_array: vec![100i, 90i, 2i, 77i, 55i] };
    let result1 = vector1.sort();
    assert_eq!(vec![2i, 55i, 77i, 90i, 100i], result1);

    let vector2 = SelectionSort{ input_array: vec![9i, 1i, 2i, 5i] };
    let result2 = vector2.sort();
    assert_eq!(vec![1i, 2i, 5i, 9i], result2);

    test_sort(10);
    test_sort(101);
    test_sort(1002);
}

fn test_sort(size: int) {
    let mut vector = SelectionSort::new(size);
    let result = vector.sort();
    vector.input_array.sort_by(|a, b| a.cmp(b)); // ascending order
    assert_eq!(vector.input_array, result);
}
