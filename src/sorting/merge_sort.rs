use sorting::Sorting;
use utils::rand_array;

pub struct MergeSort {
    pub input_array: Vec<int>
}

impl MergeSort {
    pub fn new(size: int) -> MergeSort {
        let rand_array: Vec<int> = rand_array(size);
        MergeSort {
            input_array: rand_array
        }
    }

    fn merge(left: Vec<int>, right: Vec<int>) -> Vec<int> {
        let mut result: Vec<int> = Vec::new();
        let mut left_arr = left.clone();
        let mut right_arr = right.clone();
        while left_arr.len() > 0 || right_arr.len() > 0 {
            if left_arr.len() > 0 && right_arr.len() > 0 {
                if left_arr[0] <= right_arr[0] {
                    result.push(*left_arr.get_mut(0));
                    left_arr.remove(0);
                } else {
                    result.push(*right_arr.get_mut(0));
                    right_arr.remove(0);
                }
            } else if left_arr.len() > 0 {
                result.push(*left_arr.get_mut(0));
                left_arr.remove(0);
            } else if right_arr.len() > 0 {
                result.push(*right_arr.get_mut(0));
                right_arr.remove(0);
            }
        }
        result
    }

    fn merge_sort(array: Vec<int>) -> Vec<int> {
        let mut arr = array.clone();
        let length :uint = arr.len();
        if length <= 1 {
            arr
        } else {
            let middle: uint = length / 2;
            let mut sort_left: Vec<int> = Vec::new();
            for i in range(0, middle) {
                sort_left.push(*arr.get_mut(i))
            }
            let mut sort_right: Vec<int> = Vec::new();
            for i in range(middle, length) {
                sort_right.push(*arr.get_mut(i))
            }
            MergeSort::merge(MergeSort::merge_sort(sort_left), MergeSort::merge_sort(sort_right))
        }
    }
}

impl Sorting for MergeSort {
    fn sort(&self) -> Vec<int> {
        println!("Initial array: {}", self.input_array);
        if self.input_array.len() <= 1 {
            println!("Sorted array: {}", self.input_array);
        }
        let arr = MergeSort::merge_sort(self.input_array.clone());
        println!("Sorted array: {}", arr);
        arr
    }
}
