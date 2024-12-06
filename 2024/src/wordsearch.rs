use ndarray::prelude::*;
use std::cmp::min;

type Table = Array2<String>;

fn iter_diagonals(table: &Table) -> Vec<Vec<String>> {
    // We build diagonals in two parts: the upper part and the lower part
    // The upper part always starts in the first row and goes towards the last column
    // The lower part always starts in the first column and goes towards the last row
    // The main diagonal is considered the first element of the upper part
    let mut diagonals: Vec<Vec<String>> = Vec::new();
    for jj in 0..table.ncols() {
        let mut diagonal: Vec<String> = Vec::new();
        for ii in 0..min(table.nrows(), table.ncols() - jj) {
            diagonal.push(table[(ii, ii + jj)].clone());
        }
        diagonals.push(diagonal);
    }
    for ii in 1..table.nrows() {
        let mut diagonal: Vec<String> = Vec::new();
        for jj in 0..min(table.ncols(), table.nrows() - ii) {
            diagonal.push(table[(ii + jj, jj)].clone());
        }
        diagonals.push(diagonal);
    }
    // Now the cross diagonals
    for jj in 0..table.ncols() {
        let mut diagonal: Vec<String> = Vec::new();
        for ii in 0..min(table.nrows(), jj + 1) {
            diagonal.push(table[(ii, jj - ii)].clone());
        }
        diagonals.push(diagonal);
    }
    for ii in 1..table.nrows() {
        let mut diagonal: Vec<String> = Vec::new();
        for jj in 0..(table.nrows() - ii) {
            diagonal.push(table[(ii + jj, table.ncols() - 1 - jj)].clone());
        }
        diagonals.push(diagonal);
    }
    return diagonals;
}

fn find_word_slice(slice: &[String], word: &str) -> usize {
    if slice.len() < word.len() {
        return 0;
    }
    let mut count: usize = 0;
    for ii in 0..(slice.len() - word.len() + 1) {
        let slice_str = slice[ii..(ii + word.len())].concat();
        if slice_str == word {
            count += 1;
        }
    }
    return count;
}

fn find_word_slice_2way(slice: &[String], word: &str) -> usize {
    return find_word_slice(slice, word)
        + find_word_slice(slice, &word.chars().rev().collect::<String>());
}

pub fn find_word_all_directions(table: &Table, word: &str) -> usize {
    let mut total_count = 0;
    for row in table.rows() {
        total_count += find_word_slice_2way(&row.to_vec(), word);
    }
    for col in table.columns() {
        total_count += find_word_slice_2way(&col.to_vec(), word);
    }
    for diag in iter_diagonals(&table) {
        total_count += find_word_slice_2way(&diag, word);
    }
    return total_count;
}

pub fn parse_input_04(contents: String) -> Table {
    let lines = contents.lines();
    let mut table_data: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        table_data.push(row);
    }

    return Table::from_shape_vec((table_data.len(), table_data[0].len()), table_data.concat())
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_word_slice_works() {
        let simple_slice: Vec<String> = vec!["X", "M", "A", "S", "X", "M", "A", "S"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let result = find_word_slice(&simple_slice, "XMAS");
        assert_eq!(result, 2);
    }

    #[test]
    fn iter_diagonals_works() {
        let table = array![
            ["a", "b", "c", "d"],
            ["e", "f", "g", "h"],
            ["i", "j", "k", "l"]
        ]
        .map(|&s| s.to_string());
        let expected_diagonals = vec![
            vec!["a", "f", "k"],
            vec!["b", "g", "l"],
            vec!["c", "h"],
            vec!["d"],
            vec!["e", "j"],
            vec!["i"],
            vec!["a"],
            vec!["b", "e"],
            vec!["c", "f", "i"],
            vec!["d", "g", "j"],
            vec!["h", "k"],
            vec!["l"],
        ];
        let diagonals = iter_diagonals(&table);
        assert_eq!(diagonals, expected_diagonals);
    }

    #[test]
    fn find_word_all_directions_works() {
        // MMMSXXMASM
        // MSAMXMSMSA
        // AMXSXMAAMM
        // MSAMASMSMX
        // XMASAMXAMM
        // XXAMMXXAMA
        // SMSMSASXSS
        // SAXAMASAAA
        // MAMMMXMMMM
        // MXMXAXMASX
        let table = array![
            ["M", "M", "M", "S", "X", "X", "M", "A", "S", "M"],
            ["M", "S", "A", "M", "X", "M", "S", "M", "S", "A"],
            ["A", "M", "X", "S", "X", "M", "A", "A", "M", "M"],
            ["M", "S", "A", "M", "A", "S", "M", "S", "M", "X"],
            ["X", "M", "A", "S", "A", "M", "X", "A", "M", "M"],
            ["X", "X", "A", "M", "M", "X", "X", "A", "M", "A"],
            ["S", "M", "S", "M", "S", "A", "S", "X", "S", "S"],
            ["S", "A", "X", "A", "M", "A", "S", "A", "A", "A"],
            ["M", "A", "M", "M", "M", "X", "M", "M", "M", "M"],
            ["M", "X", "M", "X", "A", "X", "M", "A", "S", "X"],
        ]
        .map(|&s| s.to_string());
        let result = find_word_all_directions(&table, "XMAS");
        assert_eq!(result, 18);
    }
}
