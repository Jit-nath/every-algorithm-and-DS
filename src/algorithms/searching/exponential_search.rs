use std::cmp::min;
use super::binary_search::binary_search;

pub fn exponential_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    if &arr[0] == target {
        return Some(0);
    }

    let mut i = 1;
    while i < n && &arr[i] <= target {
        i *= 2;
    }

    let low = i / 2;
    let high = min(i, n);
    
    match binary_search(&arr[low..high], target) {
        Some(pos) => Some(low + pos),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_search() {
        let arr = [2, 3, 4, 10, 40];
        assert_eq!(exponential_search(&arr, &10), Some(3));
        assert_eq!(exponential_search(&arr, &3), Some(1));
        assert_eq!(exponential_search(&arr, &5), None);
    }
}
