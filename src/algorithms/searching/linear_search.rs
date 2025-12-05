pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&arr, &5), Some(2));
        assert_eq!(linear_search(&arr, &10), None);
    }
}
