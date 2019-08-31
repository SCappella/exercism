pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let max_rows: Vec<_> = input.iter().filter_map(|row| row.iter().max()).collect();

    let width = input.get(0).map_or(0, |row| row.len());

    let min_columns: Vec<_> = (0..width)
        .filter_map(|i| input.iter().filter_map(|row| row.get(i)).min())
        .collect();

    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            // A hack that allows the below closure to not capture these variables
            let max_rows = &max_rows;
            let min_columns = &min_columns;
            row.iter().enumerate().filter_map(move |(j, elem)| {
                if elem == max_rows[i] && elem == min_columns[j] {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .collect()
}
