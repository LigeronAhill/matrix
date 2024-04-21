use matrix::{Matrix, Result};

fn main() -> Result<()> {
    let (m1, m2) = (Matrix::random(3, 7), Matrix::random(7, 9));
    let r = m1.multiply(&m2)?;
    println!("{m1}\nMultiply:\n{m2}\nResult:\n{r}");
    Ok(())
}
