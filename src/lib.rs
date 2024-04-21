use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};
mod error;
pub use error::{Error, Result};
#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeroes(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let data = Matrix::zeroes(rows, cols)
            .data
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|_| rng.gen::<f64>() * 2.0 - 1.0)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Matrix::from(data)
    }
    pub fn multiply(&self, other: &Matrix) -> Result<Self> {
        if self.cols != other.rows {
            return Err(Error::InvalidMatrixSize);
        }
        let mut res = Matrix::zeroes(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum
            }
        }
        Ok(res)
    }
    pub fn dot_operation(&self, other: &Matrix, operation: Operation) -> Result<Self> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(Error::InvalidMatrixSize);
        }
        let mut res = Matrix::zeroes(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = match operation {
                    Operation::Add => self.data[i][j] + other.data[i][j],
                    Operation::Subtract => self.data[i][j] - other.data[i][j],
                    Operation::Multiply => self.data[i][j] * other.data[i][j],
                };
            }
        }
        Ok(res)
    }
    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            self.data
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(function).collect())
                .collect::<Vec<Vec<f64>>>(),
        )
    }
    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::zeroes(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }
}
impl From<Vec<Vec<f64>>> for Matrix {
    fn from(value: Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: value.len(),
            cols: value[0].len(),
            data: value,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let spaces = if self.data.iter().any(|row| row.iter().any(|v| *v < 0.0)) {
            6
        } else {
            5
        };
        write!(
            f,
            "Matrix {}x{}:\n┍ {}┑\n{}\n┕ {}┙",
            self.rows,
            self.cols,
            " ".repeat(self.cols * spaces),
            self.data
                .iter()
                .map(|row| {
                    "│ ".to_string()
                        + &row
                            .iter()
                            .map(|value| {
                                if *value > 0.0 {
                                    format!(" {value:.2?} ")
                                } else {
                                    format!("{value:.2?} ")
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("")
                        + "│"
                })
                .collect::<Vec<String>>()
                .join("\n"),
            " ".repeat(self.cols * spaces),
        )
    }
}
pub enum Operation {
    Add,
    Subtract,
    Multiply,
}
