use matrix::{Matrix, Operation, Result};

fn main() -> Result<()>{
    let (m1, m2) = (Matrix::random(4,4), Matrix::random(4,4));
    let sum = m1.dot_operation(&m2, Operation::Add)?;
    let sub = m1.dot_operation(&m2, Operation::Subtract)?;
    let multi = m1.dot_operation(&m2, Operation::Multiply)?;
    println!("INPUT:\n{m1}\n{m2}\nSUM:\n{sum}\nSUB:\n{sub}\nMULTI:\n{multi}");
    Ok(())
}