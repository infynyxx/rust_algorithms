use sorting::Sorting;
use utils::rand_array;

pub struct QuickSort {
    pub input_array: Vec<int>
}

impl QuickSort {
    pub fn new(size: int) -> QuickSort {
        let mut rand_array: Vec<int> = rand_array(size);
        QuickSort {
            input_array: rand_array
        }
    }

    fn partition(array: &mut Vec<int>, left: uint, right: uint) -> uint {
        let pivot_index: uint = left + (right -  left) / 2;
        let pivot_value: int = *array.get(pivot_index);
        QuickSort::swap(array, pivot_index, right);
        let mut store_index: uint = left;
        let mut i = left;
        while i <= right - 1 {
            if *array.get(i) < pivot_value {
                QuickSort::swap(array, i, store_index);
                store_index = store_index + 1;
            }
            i = i + 1;
        }
        QuickSort::swap(array, store_index, right);
        store_index
    }

    fn swap(array: &mut Vec<int>, pivot_index: uint, right: uint) {
        let temp_pivot = *array.get(pivot_index);
        let temp_right = *array.get(right);
        *array.get_mut(pivot_index) = temp_right;
        *array.get_mut(right) = temp_pivot;
    }

    fn quick_sort(array: &mut Vec<int>, left: uint, right: uint) {
        if left < right {
            let partition = QuickSort::partition(array, left, right);
            QuickSort::quick_sort(array, left, right -1);
            QuickSort::quick_sort(array, partition + 1, right);
        }
    }
}

impl Sorting for QuickSort {
    fn sort(&self) -> Vec<int> {
        println!("Initial array: {}", self.input_array);
        if self.input_array.len() <= 1 {
            println!("Sorted array: {}", self.input_array);
            self.input_array.clone();
        }
        let mut arr = self.input_array.clone();
        QuickSort::quick_sort(&mut arr, 0, self.input_array.len() - 1);
        println!("Sorted array: {}", arr);
        arr
    }
}
