use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    rows: usize,
    columns: usize
}

impl Matrix {
    pub fn new(data: &[f64], rows: usize, columns: usize) -> Self{
        if data.len() != (rows * columns) {
            panic!("Size of given data does not match with given dimensions")
        }

        Self{
            data: Vec::from(data),
            rows,
            columns
        }
    }

    pub fn from_column_vec(data: &[f64]) -> Self{
        Self{
            data: Vec::from(data),
            rows: {
                if data.is_empty(){
                    1
                }else{
                    data.len()
                }
            },
            columns: 1,
        }
    }

    pub fn from_row_vec(data: &[f64]) -> Self{
        Self{
            data: Vec::from(data),
            rows: 1,
            columns: {
                if data.is_empty() {
                    1
                }else{
                    data.len()
                }
            },
        }
    }

    pub fn set(&mut self, element: f64, row: usize, column: usize) -> () {
        if row > self.rows || column > self.columns {
            panic!("Entry of ({},{}) is not within the bounds of the matrix", row, column)
        }
        let index = self.get_index(row, column);

        self.data[index - 1] = element;
    }

    pub fn get(&mut self, row: usize, column: usize) -> f64{
        let index = self.get_index(row, column);
        self.data[index - 1]
    }

    fn get_index(&mut self, row: usize, column: usize) -> usize {
        (self.columns * (row - 1)) + column
    }

    //dot product of ONLY equal sized vectors
    pub fn dot_product(lhs: Vec<f64>, rhs: Vec<f64>) -> f64{
        if lhs.len() != rhs.len(){
            panic!("Vectors must be the same length")
        }

        let mut res = 0.0;

        for i in 0..lhs.len() {
            res+=lhs[i]*rhs[i];
        }

        res
    }

    pub fn get_row(&self, row: usize) -> Vec<f64>{
        if row > self.rows {
            panic!("Row out of bounds")
        }
        self.data[(row-1)*self.columns..(row)*self.columns].to_vec()
    }

    pub fn get_column(&self, column: usize) -> Vec<f64>{
        if column > self.columns {
            panic!("Column out of bounds")
        }

        self.data.iter().skip(column-1).step_by(self.columns).copied().collect()
    }


}

//Returns the iterator from the data vector
impl IntoIterator for Matrix {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

//For printing
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut counter = 0;
        let mut res : String = String::from("");

        for e in self.clone().into_iter() {
            if counter % self.columns == 0 {
                res = format!("{}[{}", res, e);
            }else if ((counter + 1) % self.columns) == 0 {
                res = format!("{}, {}]\n", res, e);
            }else{
                res = format!("{}, {}", res, e);
            }

            counter += 1;
        }

        write!(f, "{}", res)
    }
}

//OPS
impl Add for Matrix{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns || self.rows != rhs.rows {
            panic!("Matrices must have the same dimensions in order to allow for addition")
        }

        let length = self.data.len();
        let mut lhs = self.data.clone();

        for i in 0..length {
            lhs[i]+=rhs.data[i];
        }

        Self{
            data: lhs,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl Sub for Matrix{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns || self.rows != rhs.rows {
            panic!("Matrices must have the same dimensions in order to allow for addition")
        }

        let length = self.data.len();
        let mut lhs = self.data.clone();

        for i in 0..length {
            lhs[i]-=rhs.data[i];
        }

        Self{
            data: lhs,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl Mul for Matrix{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.rows {
            panic!("Column size of lhs must be equal to row size of rhs")
        }

        let mut res: Vec<f64> = Vec::new();

        for i in 1..=self.rows {
            for j in 1..=self.columns{
                res.push(Matrix::dot_product(self.get_row(i), rhs.get_column(j)));
            }
        }

        Self{
            data: res,
            rows: self.columns,
            columns: rhs.rows
        }

    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        (self.data == other.data) && (self.rows == other. rows) && (self.columns == other.columns)
    }
}

impl Eq for Matrix {

}

#[macro_export]
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::structures::matrix::Matrix;
            Matrix::new(&vec![], 0, 0)
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::structures::matrix::Matrix;
            use std::cmp;
            let data = [ $( vec![$($x),*]),* ];
            let rows = data.len();
            let cols = {
                if rows < 1 {
                    0
                }else{
                    cmp::max(data[0].len(), 0)
                }
            };
            let flat_data: Vec<f64> = data.into_iter()
                .flat_map(|row| row.into_iter())
                .cloned()
                .collect();
            Matrix::new(&flat_data, rows, cols)
        }
    }
}




