extern crate rust_algorithms;

use rust_algorithms::sorting::merge_sort::MergeSort;
use rust_algorithms::sorting::Sorting;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!")
    let vector = MergeSort{input_array: rust_algorithms::utils::rand_array(150)};
    let r = vector.sort();
    println!("{}", r);
}
