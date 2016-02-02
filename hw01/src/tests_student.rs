#![cfg(test)]

use problem2::mat_mult;

#[test]
fn test_mat_mult_custom() {
    let mat1: Vec<Vec<f32>> = vec![vec![4., 3.]];
    let mat2: Vec<Vec<f32>> = vec![vec![2.], vec![1.]];
    let result = mat_mult(&mat1, &mat2);
    assert_eq!(result[0][0], 4.*2. + 3.*1.);
}