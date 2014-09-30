pub fn sort(numbers: Vec<int>) -> Vec<int> {
    println!("Initial array: {}", numbers);
    let mut i = 0;
    let mut min = 0;
    let mut tmp = 0;

    let mut arr: Vec<int> = numbers;
    while i < arr.len() - 1 {
        min = i;
        let mut j = i + 1;
        while j < arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }

            j = j + 1;
        }

        if min != i {
            tmp = arr[i];
            *arr.get_mut(i) = *arr.get_mut(min);
            *arr.get_mut(min) = tmp;
        }

        i = i + 1;
    }
    println!("Sorted array: {}", arr);
    arr
}
