use matrix::Matrix;

fn main() {
    let input_matrix = Matrix::random(7,4);
    let transposed = input_matrix.transpose();
    println!("INPUT:\n{input_matrix}\nTRANSPOSED:\n{transposed}")
}