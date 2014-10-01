extern crate rust_algorithms;

use rust_algorithms::sorting::quick_sort::QuickSort;
use rust_algorithms::sorting::Sorting;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!")
    let vector = QuickSort{input_array: rust_algorithms::utils::rand_array(150)};
    let r = vector.sort();
    println!("{}", r);
}
