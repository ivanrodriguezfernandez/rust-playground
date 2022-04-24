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

    #[derive(Debug)]
    struct Position {
        row: i32,
        col: i32,
    }

    let mut per_row_max: Vec<Position> = Vec::new();
    let mut per_col_min: Vec<Position> = Vec::new();

    for (pos, e) in input.iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
        // Give max value from row
        
        let index = input[pos]
            .iter()
            .position(|&r| r == input[pos].iter().max().unwrap().clone())
            .unwrap();
        
        per_row_max.push(Position {
            col: index as i32,
            row: pos as i32,
        });
    }

    println!("Per row max {:?}", per_row_max);

    let mut array_col: Vec<u32> = Vec::new();

    for col in 0..number_of_cols {
        for row in 0..number_of_rows {
            let aux = input[row][col];
            array_col.push(aux as u32);
        }
        let result_min = array_col.iter().min().unwrap();
        let index = array_col
            .iter()
            .position(|&r| r == result_min.clone())
            .unwrap();

        per_col_min.push(Position {
            col: col as i32,
            row: index as i32,
        });
        array_col = [].to_vec();
    }

    println!("Per col min {:?}", per_col_min);

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for vec in per_col_min {
        println!("Vec is {:?}", vec);
        let aux = per_row_max
            .iter()
            .filter(|&n| n.col == vec.col && n.row == vec.row)
            .count();
        println!("Aux is {}", aux);
        if aux > 0 {
            saddle_points.push((vec.row as usize, vec.col as usize));
        }
    }
    return saddle_points;
}

/* is greater than or equal to every element in its row
and less than or equal to every element in its column */
