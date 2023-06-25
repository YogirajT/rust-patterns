#![allow(dead_code)]

// in-place algorithm to store in n^2 space
pub fn generate_floyd_warshall_matrix(
    input_adjecency_list: &mut Vec<Vec<f32>>,
) -> Vec<Vec<f32>> {
    let num_rows = input_adjecency_list.len();
    {
        let num_cols = input_adjecency_list
            .first()
            .expect("first element has no length")
            .len();

        if num_rows != num_cols {
            panic!("invalid input: provide square matrix!")
        }

        if num_rows == 0 {
            panic!("invalid input: provide non empty matrix!")
        }
    }

    for k in 0..num_rows {
        for i in 0..num_rows {
            for j in 0..num_rows {
                input_adjecency_list[i][j] = f32::min(
                    input_adjecency_list[i][j],
                    input_adjecency_list[i][k] + input_adjecency_list[k][j],
                );
            }
        }
    }

    input_adjecency_list.to_vec()
}

#[cfg(test)]
mod floyd_warshall_tests {
    use super::*;
    use std::f32::INFINITY;

    #[test]
    fn test_floyd_warshall() {
        let graph: Vec<Vec<f32>> = vec![
            vec![0.0, 5.0, INFINITY, 10.0],
            vec![INFINITY, 0.0, 3.0, INFINITY],
            vec![INFINITY, INFINITY, 0.0, 1.0],
            vec![INFINITY, INFINITY, INFINITY, 0.0],
        ];

        let mut binding = graph;
        let fw_matrix = generate_floyd_warshall_matrix(&mut binding);

        assert_eq!(
            fw_matrix,
            vec![
                vec![0.0, 5.0, 8.0, 9.0],
                vec![INFINITY, 0.0, 3.0, 4.0],
                vec![INFINITY, INFINITY, 0.0, 1.0],
                vec![INFINITY, INFINITY, INFINITY, 0.0]
            ]
        )
    }
}
