#[allow(dead_code)]
mod sortlib {

    pub mod quicksort {
        pub fn swap<T: PartialOrd + Copy>(tab: &mut Vec<T>, i: usize, j: usize) {
            let tmp = tab[i];
            tab[i] = tab[j];
            tab[j] = tmp;
        }

        fn partition<T: PartialOrd + Copy>(tab: &mut Vec<T>, from: usize, to: usize) -> usize {
            let pivot = tab[to];
            let mut pointer = from;
            for i in from..to {
                if tab[i] <= pivot {
                    if i != pointer {
                        swap(tab, i, pointer);
                    }
                    pointer += 1;
                }
            }
            swap(tab, to, pointer);
            return pointer;
        }

        pub fn quick_sort<T: PartialOrd + Copy>(tab: &mut Vec<T>, from: usize, to: usize) {
            if from < to {
                let pivot = partition(tab, from, to);
                quick_sort(tab, from, pivot.saturating_sub(1));
                quick_sort(tab, pivot.saturating_add(1), to);
            }
        }
    }

    pub mod mergesort {
        pub fn merge_sort<T: PartialOrd + Copy>(tab: &Vec<T>, mut depth: u8) -> Vec<T> {
            let size = tab.len();
            if size <= 1 {
                return tab.to_vec();
            } else {
                let half = size / 2;
                depth += 1;
                let left = merge_sort(&tab[0..half].to_vec(), depth);
                let rigth = merge_sort(&tab[half..size].to_vec(), depth);
                let results = merge(&left, &rigth);
                return results;
            }
        }

        fn merge<'a, T: PartialOrd + Copy>(tab1: &Vec<T>, tab2: &Vec<T>) -> Vec<T> {
            if tab1.len() == 0 {
                return tab2.to_vec();
            }
            if tab2.len() == 0 {
                return tab1.to_vec();
            }

            let sizei = tab1.len();
            let mut i = 0;

            let sizej = tab2.len();
            let mut j = 0;

            let mut merged: Vec<T> = Vec::with_capacity(sizei + sizej);

            while i < tab1.len() && j < tab2.len() {
                if tab1[i] < tab2[j] {
                    merged.push(tab1[i]);
                    i += 1;
                } else {
                    merged.push(tab2[j]);
                    j += 1;
                }
            }

            for k in i..sizei {
                merged.push(tab1[k]);
            }

            for k in j..sizej {
                merged.push(tab2[k]);
            }

            return merged;
        }
    }

    pub mod insertion {

        pub fn sort<T: PartialOrd + Copy>(arr: &mut Vec<T>) {
            let length = arr.len();
            if length <= 1 {
                return;
            }
            for i in 1..length {
                let current_value = arr[i];
                let mut j = i;
                while j > 0 && arr[j - 1] > current_value {
                    arr[j] = arr[j - 1];
                    j -= 1;
                }
                arr[j] = current_value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sortlib::{
        insertion,
        mergesort::merge_sort,
        quicksort::{quick_sort, swap},
    };

    #[test]
    fn test_swap_ok() {
        let mut tab = vec![10, 13];
        swap(&mut tab, 0, 1);
        assert_eq!(tab, vec![13, 10]);
    }

    #[test]
    fn test_swap_not_ok() {
        let mut tab = vec![10, 13];
        swap(&mut tab, 0, 1);
        assert_ne!(tab, vec![10, 13]);
    }

    #[test]
    fn test_quick_sort() {
        let mut tab = vec![38, 13, 2, 99, 45, 17, 23, 89, 40];
        let length = tab.len();
        quick_sort(&mut tab, 0, length - 1);
        assert_eq!(vec![2, 13, 17, 23, 38, 40, 45, 89, 99], tab);
    }

    #[test]
    fn test_merge_sort() {
        let tab = vec![38, 13, 2, 99, 45, 17, 23, 89, 40];
        let sorted = merge_sort(&tab, 0);
        assert_eq!(vec![2, 13, 17, 23, 38, 40, 45, 89, 99], sorted);
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut arr: Vec<i32> = Vec::new();
        insertion::sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_insertion_sort_one_item() {
        let mut arr: Vec<i32> = vec![7];
        insertion::sort(&mut arr);
        assert_eq!(vec![7], arr);
    }

    #[test]
    fn test_insertion_sort_two_items() {
        let mut arr: Vec<i32> = vec![2, 5];
        insertion::sort(&mut arr);
        assert_eq!(vec![2, 5], arr);
    }

    #[test]
    fn test_insertion_sort_more_items() {
        let mut arr: Vec<i32> = vec![38, 13, 2, 99, 45, 17, 23, 89, 40];
        insertion::sort(&mut arr);
        assert_eq!(vec![2, 13, 17, 23, 38, 40, 45, 89, 99], arr);
    }
}
