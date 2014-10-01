use sorting::Sorting;
use utils::rand_array;

pub struct SelectionSort {
    pub input_array: Vec<int>
}

impl SelectionSort {
    pub fn new (size: int) -> SelectionSort {
        SelectionSort {
            input_array: rand_array(size)
        }
    }
}

impl Sorting for SelectionSort {
    fn sort(&self) -> Vec<int> {
        println!("Initial array: {}", self.input_array);
        let mut i = 0;
        
        let mut arr: Vec<int> = self.input_array.clone();
        while i < arr.len() - 1 {
            let mut min = i;
            let mut j = i + 1;
            while j < arr.len() {
                if arr[j] < arr[min] {
                    min = j;
                }

                j = j + 1;
            }

            if min != i {
                let tmp = arr[i];
                *arr.get_mut(i) = *arr.get_mut(min);
                *arr.get_mut(min) = tmp;
            }

            i = i + 1;
        }
        println!("Sorted array: {}", arr);
        arr
    }
}
