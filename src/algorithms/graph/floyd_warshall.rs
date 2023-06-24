#[allow(dead_code)]


// in-place algorithm to store in n^2 space
pub fn generate_floyd_warshall_matrix(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let num_rows = input.len();
    let num_cols = input.first().expect("first element has no length").len();


    if num_rows != num_cols { panic!("invalid input: provide square matrix!")}

    return vec![vec![]];
}


#[cfg(test)]
mod floyd_warshall_tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
    }
}
