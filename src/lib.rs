#[allow(dead_code)]
mod sortlib {

    pub mod quicksort {
        pub fn swap(tab: &mut Vec<i32>, i: usize, j: usize) {
            let tmp = tab[i];
            tab[i] = tab[j];
            tab[j] = tmp;
        }

        fn partition(tab: &mut Vec<i32>, from: usize, to: usize) -> usize {
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

        pub fn quick_sort(tab: &mut Vec<i32>, from: usize, to: usize) {
            if from < to {
                let pivot = partition(tab, from, to);
                quick_sort(tab, from, pivot.saturating_sub(1));
                quick_sort(tab, pivot.saturating_add(1), to);
            }
        }
    }

    pub mod mergesort {
        pub fn merge_sort(tab: &Vec<i32>, mut depth: u8) -> Vec<i32> {
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

        fn merge<'a>(tab1: &Vec<i32>, tab2: &Vec<i32>) -> Vec<i32> {
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

            let mut merged: Vec<i32> = Vec::with_capacity(sizei + sizej);

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
}

#[cfg(test)]
mod tests {
    use super::sortlib::{
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
}
