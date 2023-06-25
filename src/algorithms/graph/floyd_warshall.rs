#![allow(dead_code)]
use std::{f32::{INFINITY}};

// in-place algorithm to store in n^2 space
pub fn generate_floyd_warshall_matrix(input_adjecency_list: &Vec<&Vec<f32>>) -> Vec<Vec<f32>> {

    let num_rows = input_adjecency_list.len();
    let num_cols = input_adjecency_list.first().expect("first element has no length").len();


    if num_rows != num_cols { panic!("invalid input: provide square matrix!")}

    if num_rows == 0 { panic!("invalid input: provide square matrix!")}
    if num_cols == 0 { panic!("invalid input: provide square matrix!")}

    let mut fw_matrix: Vec<Vec<f32>> = vec![vec![INFINITY; num_cols]; num_rows];

    for k in 0..num_cols {
        for i in 0..num_cols {
            for j in 0..num_rows {
                fw_matrix[i][j] = f32::min(fw_matrix[i][j], fw_matrix[i][k] + fw_matrix[k][j]);
            }
        }
    }

    fw_matrix
}


#[cfg(test)]
mod floyd_warshall_tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let graph: Vec<Vec<f32>> = vec![ vec![ 0.0, 5.0, INFINITY, 10.0 ],
                        vec![ INFINITY, 0.0, 3.0, INFINITY ],
                        vec![ INFINITY, INFINITY, 0.0, 1.0 ],
                        vec![ INFINITY, INFINITY, INFINITY, 0.0 ] ];
    }
}
