pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    let number_of_rows: usize = input.len();
    let number_of_cols: usize = input[0].len();
    
    if number_of_cols == 0{
        let saddle_points_empty: Vec<(usize, usize)> = Vec::new();
        return saddle_points_empty;
    }

    /*
    9 8 7
    5 3 2
    6 6 7
    */

    // row, col

    let per_row_max: Vec<(i32,i32)> = input
        .iter()
        .enumerate().map(|(x , y)| (x as i32, y.iter().position(|&r| r == y.iter().max().unwrap().clone())
        .unwrap() as i32)).collect();

    println!("Per row max {:?}", per_row_max);

    let mut array_col: Vec<u32> = Vec::new();

    let mut per_col_min: Vec<(i32,i32)> = Vec::new();
    
    for col in 0..number_of_cols {
        for row in 0..number_of_rows {
            let aux = input[row][col];
            array_col.push(aux as u32);
        }
        
        let index = array_col
            .iter()
            .position(|&r| r == array_col.iter().min().unwrap().clone())
            .unwrap();

        per_col_min.push((index as i32, col as i32));

        array_col = [].to_vec();
    }

    println!("Per col min {:?}", per_col_min);

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for vec in per_col_min {
        println!("Vec is {:?}", vec);
        let aux = per_row_max
            .iter()
            .filter(|&n| n.1 == vec.1 && n.0 == vec.0)
            .count();
        println!("Aux is {}", aux);
        if aux > 0 {
            saddle_points.push((vec.0 as usize, vec.1 as usize));
        }
    }
    return saddle_points;
}

/* is greater than or equal to every element in its row
and less than or equal to every element in its column */
