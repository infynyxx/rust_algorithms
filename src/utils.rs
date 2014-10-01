//extern crate rust_algorithms;

use std::rand;

pub fn rand_array(size: int) -> Vec<int> {
    let u_size_option: Option<uint> = size.to_uint();
    let u_size = match u_size_option {
        Some(x) => x,
        None => 10 // default to size of 10
    };
    let rand_arr: Vec<int> = Vec::from_fn(u_size, | idx | {
        (rand::random::<int>() % 100i) + idx as int
    });
    rand_arr
}
