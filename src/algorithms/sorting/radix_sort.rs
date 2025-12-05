pub fn radix_sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1;

    while max_val / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [usize], exp: usize) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &x in arr.iter() {
        count[(x / exp) % 10] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &x in arr.iter().rev() {
        let idx = (x / exp) % 10;
        output[count[idx] - 1] = x;
        count[idx] -= 1;
    }

    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, [2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
