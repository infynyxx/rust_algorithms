use sorting::Sorting;
use utils::rand_array;

pub struct QuickSort {
    pub input_array: Vec<int>
}

impl QuickSort {
    pub fn new (size: int) -> QuickSort {
        let mut rand_array: Vec<int> = rand_array(size);
        QuickSort {
            input_array: rand_array
        }
    }

    fn quick_sort(array: &Vec<int>, left: uint, right: uint) {
        let mut i = left;
        let mut j = right;
        let index: uint = left + (left + right) / 2;
        let pivot: int = *array.get_mut(index);
        while (i <= j) {
            while (array[i] < pivot) {
                i = i + 1;
            }

            while (array[j] < pivot) {
                j = j - 1;
            }

            if (i <= j) {
                let temp = array[i];
                *array.get_mut(i) = *array.get_mut(j);
                *array.get_mut(j) = temp;
                i = i + 1;
                j = j - 1;
            }
        }

        if left < j {
            QuickSort::quick_sort(array, left, j);
        }

        if i < right {
            QuickSort::quick_sort(array, i, right);
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
        QuickSort::quick_sort(&arr, 0, arr.len() - 1);
        println!("Sorted array: {}", arr);
        arr
    }
}
