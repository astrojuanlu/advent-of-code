pub fn total_distance(left: Vec<usize>, right: Vec<usize>) -> usize {
    if left.len() != right.len() {
        panic!("Vectors must be the same length");
    }

    let mut sorted_left = left.clone();
    let mut sorted_right = right.clone();
    sorted_left.sort();
    sorted_right.sort();

    let mut distance: usize = 0;
    for index in 0..left.len() {
        distance = distance + sorted_left[index].abs_diff(sorted_right[index]);
    }

    return distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_distance_works() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let result = compute_total_distance(left, right);
        assert_eq!(result, 11);
    }
}
