static NOT_FOUND: int = -1;

fn find_rank(items: Vec<int>, item: int, low: uint, high: uint) -> int {
    if high < low {
        return NOT_FOUND;
    }
    let mid: uint = (high + low) / 2;
    println!("mid {} low {} high {}", mid, low, high);
    if items[mid] > item {
        find_rank(items, item, low, mid - 1)
    } else if items[mid] < item {
        find_rank(items, item, mid + 1, high)
    } else {
        mid as int
    }
}

pub fn search(items: Vec<int>, item: int) -> int {
    if items.len() == 0 {
        return NOT_FOUND;
    }
    let mut sorted_vector = items.clone();
    sorted_vector.sort_by(|a, b| {a.cmp(b)});
    let high = items.len();
    let low = 0;
    println!("Haystack: {}", items);
    println!("Sorted Haystack: {}", sorted_vector);
    println!("Needle: {}", item);
    find_rank(sorted_vector, item, low, high)
}
