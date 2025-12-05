pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut arr = [12, 34, 54, 2, 3];
        shell_sort(&mut arr);
        assert_eq!(arr, [2, 3, 12, 34, 54]);
    }
}
