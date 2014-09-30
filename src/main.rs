extern crate rust_algorithms;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!")
    println!("{}", rust_algorithms::sorting::selection_sort::sort(vec![1i, 2, 3, 4, 0]));
}
