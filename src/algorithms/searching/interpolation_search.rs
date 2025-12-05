pub fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        if low == high {
            if arr[low] == target {
                return Some(low);
            }
            return None;
        }

        let pos = low + (((high - low) as i128 * (target - arr[low]) as i128)
            / (arr[high] - arr[low]) as i128) as usize;

        if arr[pos] == target {
            return Some(pos);
        }

        if arr[pos] < target {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_search() {
        let arr = [10, 12, 13, 16, 18, 19, 20, 21, 22, 23, 24, 33, 35, 42, 47];
        assert_eq!(interpolation_search(&arr, 18), Some(4));
        assert_eq!(interpolation_search(&arr, 10), Some(0));
        assert_eq!(interpolation_search(&arr, 47), Some(14));
        assert_eq!(interpolation_search(&arr, 48), None);
    }
}
