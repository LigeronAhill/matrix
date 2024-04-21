use matrix::Matrix;

fn main() {
    let input = Matrix::random(3,3);
    let f = |v:f64| -> f64 {
        v * 0.5
    };
    let mapped = input.map(&f);
    println!("INPUT:\n{input}\nMAPPED:\n{mapped}") 
}