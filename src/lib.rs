//! A group of various sorting algorithms with varying efficiencies.
//! The list in order of efficiency is `merge_sort`, `bubble_sort`, `gnome_sort`, `bogo_sort`.
//! ***WARNING**: Please do not use `bogo_sort` with large vectors as it may never finish.
//! # Examples
//! ```
//! use sorting_lib::sorting::*;
//! let mut vector: Vec<i32> = vec![4,2,0,6,9];
//! let mut length = vector.len();
//! merge_sort(&mut vector, 0, length - 1);
//! println!("Sorted Vector: {:?}", vector);
//! ```
//! ~ *Made by Sirfredrick*
//!
pub mod sorting {
    extern crate rand;
    use rand::thread_rng;
    use rand::seq::SliceRandom;

    /// Recursive part of `merge_sort` algorithm that divides the array and merges the parts back together.
    /// Takes a mutable reference to a i32 Vector and a starting and ending usize.
    pub fn merge_sort(vect: &mut Vec<i32>, start: usize, end: usize) {
        if start < end {
            let middle: usize = (start + end) / 2;
            merge_sort(vect, start, middle);
            merge_sort(vect, middle + 1, end);
            merge(vect, start, middle, end);
        }
    }

    /// Merge part of `merge_sort` algorithm that combines the two halves together in sequential order.
    /// Takes a mutable reference to a i32 Vector and a starting, middle, and ending usize as parameters.
    fn merge(vect: &mut Vec<i32>, start: usize, middle: usize, end: usize) {
        // Finds the size of both halves of the array when split by middle
        let size1: usize = middle - start + 1;
        let size2: usize = end - middle;

        // Setups the two halves of the array
        let mut left = vec![0; size1];
        let mut right = vec![0; size2];

        // Fills the two halves with the data from `vect`
        for i in 0..size1 {
            left[i] = vect[start + i];
        }
        for j in 0..size2 {
            right[j] = vect[middle + 1 + j];
        }

        // Initializes iterating variables
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = start;

        // While neither half has been completely added back into `vect`
        while i < size1 && j < size2 {
            // If the left value is less than or equal to the right
            if left[i] <= right[j] {
                // Put the lower value (the left) in the next index in the full array
                vect[k] = left[i];
                i += 1;
            } else {
                // Otherwise, put the lower value (the right) in the next index in the full array
                vect[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        // If the left array has been completely added back into `vect`
        if i == size1 {
            // Fill the rest of `vect` with the rest of the right array
            while k <= end {
                vect[k] = right[j];
                k += 1;
                j += 1;
            }
        } else if j == size2 {
            // Otherwise, fill the rest of `vect` with the rest of the left array
            while k <= end {
                vect[k] = left[i];
                k += 1;
                i += 1;
            }
        }
    }

    /// `bubble_sort` algorithm that compares adjacent values and swaps until it is sorted.
    ///  Takes a mutable reference to a i32 Vector as a parameter.
    pub fn bubble_sort(vect: &mut Vec<i32>) {
        for _i in 0..vect.len() - 1 {
            // Setup bool to determine if the array has been swapped during this iteration
            let mut swapped = false;
            // Iterate up to the length of `vect` - 1
            for j in 0..vect.len() - 1 {
                // If the current value is greater than the next value
                if vect[j] > vect[j + 1] {
                    // Swap the values
                    let temp = vect[j];
                    vect[j] = vect[j + 1];
                    vect[j + 1] = temp;
                    // Set swapped to true
                    swapped = true;
                }
            }
            // If `vect` has not been swapped this iteration, break out of the loop
            if !swapped {
                break;
            }
        }
    }

    /// `gnome_sort` algorithm that is known for its concise code size hence the name gnome.
    ///  Takes a mutable reference to a i32 Vector as a parameter.
    pub fn gnome_sort(vect: &mut Vec<i32>) {
        let mut pos = 0;
        while pos < vect.len() {
            if pos == 0 || vect[pos] >= vect[pos - 1] {
                pos = pos + 1
            } else {
                let temp = vect[pos];
                vect[pos] = vect[pos - 1];
                vect[pos - 1] = temp;
                pos = pos - 1;
            }
        }
    }

    /// `bogo_sort` algorithm that checks if `vect` is sorted and if not it shuffles `vect` until it is sorted.
    /// Know for its inefficiency.
    /// ***WARNING**: Please do not use `bogo_sort` with large vectors as it may never finish.
    ///  Takes a mutable reference to a i32 Vector as a parameter.
    pub fn bogo_sort(vect: &mut Vec<i32>) {
        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            let mut randy = thread_rng();
            vect.shuffle(&mut randy);
            let mut i = 0;
            while i < vect.len() - 1 && is_sorted {
                if vect[i] > vect[i + 1] {
                    is_sorted = false;
                }
                i += 1;
            }
        }
    }

    #[cfg(test)]
    mod tests{
        ///! Test module for the sorting algorithms.
        use super::*;

        #[test]
        /// Tests `merge_sort` with various vectors.
        fn test_merge_sort() {
            let vect: Vec<i32> = vec![9, 7, 5, 3, 2, 0];
            let expect: Vec<i32> = vec![0, 2, 3, 5, 7, 9];
            let vect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let expect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let vect3: Vec<i32> = vec![84, 29, 0, 34, 23, 9, 8];
            let expect3: Vec<i32> = vec![0, 8, 9, 23, 29, 34, 84];
            let mut vector = vect.to_vec();
            let mut length = vector.len();
            merge_sort(&mut vector, 0, length - 1);
            assert_eq!(vector, expect);

            let mut vector2 = vect2.to_vec();
            length = vector2.len();
            merge_sort(&mut vector2, 0, length - 1);
            assert_eq!(vector2, expect2);

            let mut vector3 = vect3.to_vec();
            length = vector3.len();
            merge_sort(&mut vector3, 0, length - 1);
            assert_eq!(vector3, expect3);
        }

        #[test]
        /// Tests `bubble_sort` with various vectors.
        fn test_bubble_sort() {
            let vect: Vec<i32> = vec![9, 7, 5, 3, 2, 0];
            let expect: Vec<i32> = vec![0, 2, 3, 5, 7, 9];
            let vect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let expect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let vect3: Vec<i32> = vec![84, 29, 0, 34, 23, 9, 8];
            let expect3: Vec<i32> = vec![0, 8, 9, 23, 29, 34, 84];

            let mut vector = vect.to_vec();
            bubble_sort(&mut vector);
            assert_eq!(vector, expect);

            let mut vector2 = vect2.to_vec();
            bubble_sort(&mut vector2);
            assert_eq!(vector2, expect2);

            let mut vector3 = vect3.to_vec();
            bubble_sort(&mut vector3);
            assert_eq!(vector3, expect3);
        }

        #[test]
        /// Tests `gnome_sort` with various vectors.
        fn test_gnome_sort() {
            let vect: Vec<i32> = vec![9, 7, 5, 3, 2, 0];
            let expect: Vec<i32> = vec![0, 2, 3, 5, 7, 9];
            let vect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let expect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let vect3: Vec<i32> = vec![84, 29, 0, 34, 23, 9, 8];
            let expect3: Vec<i32> = vec![0, 8, 9, 23, 29, 34, 84];

            let mut vector = vect.to_vec();
            gnome_sort(&mut vector);
            assert_eq!(vector, expect);

            let mut vector2 = vect2.to_vec();
            gnome_sort(&mut vector2);
            assert_eq!(vector2, expect2);

            let mut vector3 = vect3.to_vec();
            gnome_sort(&mut vector3);
            assert_eq!(vector3, expect3);
        }

        #[test]
        /// Tests `bogo_sort` with various vectors.
        /// The vectors used are small so the test should still be able to complete.
        fn test_bogo_sort() {
            let vect: Vec<i32> = vec![9, 7, 5, 3, 2, 0];
            let expect: Vec<i32> = vec![0, 2, 3, 5, 7, 9];
            let vect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let expect2: Vec<i32> = vec![-1, 0, 4, 5, 9, 25];
            let vect3: Vec<i32> = vec![84, 29, 0, 34, 23, 9, 8];
            let expect3: Vec<i32> = vec![0, 8, 9, 23, 29, 34, 84];

            let mut vector = vect.to_vec();
            bogo_sort(&mut vector);
            assert_eq!(vector, expect);

            let mut vector2 = vect2.to_vec();
            bogo_sort(&mut vector2);
            assert_eq!(vector2, expect2);

            let mut vector3 = vect3.to_vec();
            bogo_sort(&mut vector3);
            assert_eq!(vector3, expect3);
        }
    }
}

