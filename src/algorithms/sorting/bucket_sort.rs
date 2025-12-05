pub fn bucket_sort(arr: &mut [f64]) {
    if arr.is_empty() {
        return;
    }

    let n = arr.len();
    let mut buckets: Vec<Vec<f64>> = vec![vec![]; n];

    for &x in arr.iter() {
        let idx = (n as f64 * x) as usize;
        if idx < n {
            buckets[idx].push(x);
        } else {
            // Handle 1.0 or greater if necessary, though standard bucket sort assumes [0, 1)
            buckets[n - 1].push(x);
        }
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut i = 0;
    for bucket in buckets {
        for x in bucket {
            arr[i] = x;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut arr = [0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.1234, 0.3434, 0.565, 0.656, 0.665, 0.897]);
    }
}
