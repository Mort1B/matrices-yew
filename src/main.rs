use matrix::Matrix;

mod matrix;

fn main() {
    let matrix = vec![
        vec![1, 2, -1, 1],
        vec![-1, 0, 2, -2],
        vec![3, -1, 1, 1],
        vec![2, 0, -1, 2],
    ];

    let matrix_ops = Matrix::new(matrix);

    println!("{:?}", matrix_ops.det());

    for row in matrix_ops.transpose() {
        println!("{:?}", row)
    }
}
