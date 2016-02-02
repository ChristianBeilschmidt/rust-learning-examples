/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
	assert_eq!(mat1[0].len(), mat2.len());
	
	let height = mat1.len();
	let width = mat2[0].len();
	let iteration_length = mat1[0].len();
	let mut matrix = vec![vec![0.; width]; height];    
    
	for r in 0..height {
		for c in 0..width {
			println!("r: {}, c: {}", r, c);
			for i in 0..iteration_length {
				matrix[r][c] += mat1[r][i] * mat2[i][c];
			}
		}
	}
	
	matrix
}