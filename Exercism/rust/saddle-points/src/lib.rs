pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let number_of_cols: usize = input[0].len();

    if number_of_cols == 0 {
        let saddle_points_empty: Vec<(usize, usize)> = Vec::new();
        return saddle_points_empty;
    }

    /* is greater than or equal to every element in its row
    and less than or equal to every element in its column */

    // row, col

    let mut per_row_max: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .map(|(x, y)| {
            (
                x as usize,
                y.iter()
                    .position(|&r| r == y.iter().max().unwrap().clone())
                    .unwrap() as usize,
            )
        })
        .collect();

    println!("Per row max {:?}", per_row_max);

    let mut per_col_min: Vec<(usize, usize)> = Vec::new();

    for y in 0..number_of_cols {
        let mut min = input[0][y];
        let mut mins: Vec<(usize, usize)> = Vec::new();

        for x in 0..input.len() {
            let val = input[x][y];
            if val < min {
                min = val;
                mins = vec![(x, y)];
            } else if val == min {
                mins.push((x, y));
            }
        }
        for tuple in mins {
            per_col_min.push(tuple);
        }
    }

    println!("Per col min {:?}", per_col_min);

    per_row_max.retain(|x| per_col_min.contains(x));
    per_row_max
}
