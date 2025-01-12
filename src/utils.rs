pub mod numbers;

pub trait MergeSort {
    /// In place merge sort which overwrites the original implementing struct
    fn merge_sort(&mut self);
}

impl<T> MergeSort for Vec<T>
where
    T: PartialOrd + Copy,
{
    /// In-place merge sort which will overwrite the original array.
    fn merge_sort(&mut self) {
        // Base case
        if self.len() < 2 {
            return;
        }

        // Divide
        let mut left = Vec::new();
        let mut right = Vec::new();

        let mid_point = (self.len() - (self.len() % 2)) / 2;
        for i in 0..mid_point {
            left.push(self[i]);
        }
        for i in mid_point..self.len() {
            right.push(self[i])
        }

        // Sort
        left.merge_sort();
        right.merge_sort();

        // Merge
        let mut left_pos = 0;
        let mut right_pos = 0;
        while (left_pos + right_pos) < self.len() {
            let val: T;
            let self_pos = left_pos + right_pos;

            if left_pos == left.len() {
                val = right[right_pos];
                right_pos += 1;
            } else if right_pos == right.len() {
                val = left[left_pos];
                left_pos += 1;
            } else if left[left_pos] <= right[right_pos] {
                val = left[left_pos];
                left_pos += 1;
            } else {
                val = right[right_pos];
                right_pos += 1;
            }

            self[self_pos] = val;
        }
    }
}

mod test {
    use crate::utils::MergeSort;

    #[test]
    fn sort_empty_vector() {
        let mut v: Vec<i32> = Vec::new();
        v.merge_sort();
    }

    #[test]
    fn sort_one_item() {
        let mut v = vec![1];
        v.merge_sort();
    }

    #[test]
    fn sort_two_items() {
        let mut v = vec![1, 2];
        v.merge_sort();
        assert_eq!(v, vec!(1, 2));

        v = vec![2, 1];
        v.merge_sort();
        assert_eq!(v, vec!(1, 2));

        v = vec![1, 1];
        v.merge_sort();
        assert_eq!(v, vec!(1, 1));
    }

    #[test]
    fn sort_three_items() {
        let mut v = vec![1, 2, 3];
        v.merge_sort();
        assert_eq!(v, vec!(1, 2, 3));

        let mut v = vec![2, 1, 3];
        v.merge_sort();
        assert_eq!(v, vec!(1, 2, 3));

        let mut v = vec![3, 2, 1];
        v.merge_sort();
        assert_eq!(v, vec!(1, 2, 3));

        let mut v = vec![1, 1, 0];
        v.merge_sort();
        assert_eq!(v, vec!(0, 1, 1));
    }
}
