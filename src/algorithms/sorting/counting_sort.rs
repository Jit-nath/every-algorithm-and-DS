pub fn counting_sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let mut count = vec![0; max_val + 1];
    let mut output = vec![0; arr.len()];

    for &x in arr.iter() {
        count[x] += 1;
    }

    for i in 1..=max_val {
        count[i] += count[i - 1];
    }

    for &x in arr.iter().rev() {
        output[count[x] - 1] = x;
        count[x] -= 1;
    }

    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = [4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }
}
