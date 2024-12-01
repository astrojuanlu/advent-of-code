pub fn total_distance(left: &Vec<usize>, right: &Vec<usize>) -> usize {
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

pub fn similarity_score(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut score: usize = 0;
    for left_element in left.iter() {
        let num_times = right.iter().filter(|r_el| r_el == &left_element).count();
        let increment = left_element * num_times;
        score = score + increment
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_distance_works() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let result = total_distance(&left, &right);
        assert_eq!(result, 11);
    }

    #[test]
    fn similarity_score_works() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let result = similarity_score(&left, &right);
        assert_eq!(result, 31);
    }
}
