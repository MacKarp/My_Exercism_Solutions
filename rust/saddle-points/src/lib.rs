pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    // if first row is empty, return empty Vector
    if input[0].is_empty() {
        return result;
    };

    // search rows with maximum value
    let row_max: Vec<u64> = input.iter().map(|row| *row.iter().max().unwrap()).collect();

    // search columns with minimum value
    let col_min: Vec<u64> = input[0]
        .iter()
        .enumerate()
        .map(|(c, _)| input.iter().map(|row| row[c]).min().unwrap())
        .collect();

    // check if saddle points
    for (i, row_vec) in input.iter().enumerate() {
        for (j, &elem) in row_vec.iter().enumerate() {
            if elem == row_max[i] && elem == col_min[j] {
                result.push((i, j));
            }
        }
    }
    result
}
