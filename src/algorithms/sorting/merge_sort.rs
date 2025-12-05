pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n > 1 {
        let mid = n / 2;
        merge_sort(&mut arr[0..mid]);
        merge_sort(&mut arr[mid..n]);
        let mut left = arr[0..mid].to_vec();
        let mut right = arr[mid..n].to_vec();
        merge(arr, &mut left, &mut right);
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &mut [T], right: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [12, 11, 13, 5, 6, 7];
        merge_sort(&mut arr);
        assert_eq!(arr, [5, 6, 7, 11, 12, 13]);
    }
}
