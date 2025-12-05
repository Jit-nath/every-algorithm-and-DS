use std::cmp::min;

pub fn jump_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    let mut step = (n as f64).sqrt() as usize;
    let mut prev = 0;

    while &arr[min(step, n) - 1] < target {
        prev = step;
        step += (n as f64).sqrt() as usize;
        if prev >= n {
            return None;
        }
    }

    while &arr[prev] < target {
        prev += 1;
        if prev == min(step, n) {
            return None;
        }
    }

    if &arr[prev] == target {
        return Some(prev);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_search() {
        let arr = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];
        assert_eq!(jump_search(&arr, &55), Some(10));
        assert_eq!(jump_search(&arr, &144), Some(12));
        assert_eq!(jump_search(&arr, &611), None);
    }
}
