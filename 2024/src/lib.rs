#[derive(Debug)]
pub enum InvalidReportError {
    CannotDampen,
}

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

fn report_diffs(report: &Vec<isize>) -> Vec<isize> {
    let mut report_orig = report.clone();
    let mut report_shifted = report.clone();

    // TODO: Use slices instead?
    report_orig.remove(report_orig.len() - 1);
    report_shifted.remove(0);

    return report_orig
        .iter()
        .zip(report_shifted.iter())
        .map(|(&o, &s)| o - s)
        .collect();
}

fn report_safety_no_dampener(report: &Vec<isize>) -> bool {
    let diffs = report_diffs(&report);

    return (diffs.iter().all(|&d| d > 0) | diffs.iter().all(|&d| d < 0))
        & (diffs.iter().map(|&d| d.abs()).all(|d| d <= 3));
}

fn dampen_report(report: &Vec<isize>) -> Result<Vec<isize>, InvalidReportError> {
    // If it's valid, just return it
    let mut dampened_report = report.clone();
    if report_safety_no_dampener(&dampened_report) {
        return Ok(dampened_report);
    }

    // Otherwise it's easier to just brute force all the variations
    for index in 0..report.len() {
        dampened_report.remove(index);
        if report_safety_no_dampener(&dampened_report) {
            return Ok(dampened_report);
        }
        dampened_report = report.clone();
    }

    Err(InvalidReportError::CannotDampen)
}

pub fn report_safety(report: &Vec<isize>, dampener: bool) -> bool {
    let mut result = report_safety_no_dampener(report);
    if !result & dampener {
        if let Ok(dampened_report) = dampen_report(report) {
            result = report_safety_no_dampener(&dampened_report);
        } else {
            result = false;
        }
    }
    return result;
}

pub fn count_safe_reports(reports: &Vec<Vec<isize>>, dampener: bool) -> usize {
    return reports
        .iter()
        .map(|r| report_safety(&r, dampener))
        .filter(|&s| s == true)
        .count();
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

    #[test]
    fn report_safety_no_dampener_works() {
        let safe_report1 = vec![7, 6, 4, 2, 1];
        let safe_report2 = vec![1, 3, 6, 7, 9];
        let unsafe_report1 = vec![1, 2, 7, 8, 9];
        let unsafe_report2 = vec![9, 7, 6, 2, 1];
        let unsafe_report3 = vec![1, 3, 2, 4, 5];
        let unsafe_report4 = vec![8, 6, 4, 4, 1];

        assert!(report_safety(&safe_report1, false));
        assert!(report_safety(&safe_report2, false));

        assert!(!report_safety(&unsafe_report1, false));
        assert!(!report_safety(&unsafe_report2, false));
        assert!(!report_safety(&unsafe_report3, false));
        assert!(!report_safety(&unsafe_report4, false));
    }

    #[test]
    fn dampen_report_works() {
        let safe_report1 = vec![7, 6, 4, 2, 1];
        let safe_report2 = vec![1, 3, 6, 7, 9];
        let safe_report_with_dampening1 = vec![1, 3, 2, 4, 5];
        let safe_report_with_dampening2 = vec![8, 6, 4, 4, 1];

        assert_eq!(dampen_report(&safe_report1).unwrap(), safe_report1);
        assert_eq!(dampen_report(&safe_report2).unwrap(), safe_report2);

        assert_eq!(
            dampen_report(&safe_report_with_dampening1).unwrap(),
            vec![1, 2, 4, 5]
        );
        assert_eq!(
            dampen_report(&safe_report_with_dampening2).unwrap(),
            vec![8, 6, 4, 1]
        );
    }

    #[test]
    fn report_safety_dampener_works() {
        let safe_report1 = vec![7, 6, 4, 2, 1];
        let safe_report2 = vec![1, 3, 6, 7, 9];
        let safe_report3 = vec![1, 3, 2, 4, 5];
        let safe_report4 = vec![8, 6, 4, 4, 1];
        let unsafe_report1 = vec![1, 2, 7, 8, 9];
        let unsafe_report2 = vec![9, 7, 6, 2, 1];

        assert!(report_safety(&safe_report1, true));
        assert!(report_safety(&safe_report2, true));
        assert!(report_safety(&safe_report3, true));
        assert!(report_safety(&safe_report4, true));

        assert!(!report_safety(&unsafe_report1, true));
        assert!(!report_safety(&unsafe_report2, true));
    }
}
