mod structures;

#[cfg(test)]
mod tests {
    use crate::structures::matrix::Matrix;

    #[test]
    fn matrix_addition() {
        let a = 12.5;
        let b = 11.0;
        let c = 5.5;
        let d = 15.97;

        //2x2 matrices
        let matrix_a = Matrix::new(&*vec![a, b, c ,d], 2, 2);
        let matrix_b = Matrix::new(&*vec![d, c, b, a], 2, 2);

        assert_eq!(matrix_a+matrix_b, Matrix::new(&*vec![a+d, b+c, c+b ,d+a], 2, 2))
    }

    #[test]
    fn matrix_subtraction() {
        let a = 12.5;
        let b = 11.0;
        let c = 5.5;
        let d = 15.97;

        //2x2 matrices
        let matrix_a = Matrix::new(&*vec![a, b, c ,d], 2, 2);
        let matrix_b = Matrix::new(&*vec![d, c, b, a], 2, 2);

        assert_eq!(matrix_a-matrix_b, Matrix::new(&*vec![a-d, b-c, c-b ,d-a], 2, 2))
    }

    #[test]
    fn matrix_multiplication() {
        let a = 12.5;
        let b = 11.0;
        let c = 5.5;
        let d = 15.97;

        //2x2 matrices
        let matrix_a = Matrix::new(&*vec![a, b, c ,d], 2, 2);
        let matrix_b = Matrix::new(&*vec![d, c, b, a], 2, 2);

        assert_eq!(matrix_a*matrix_b, Matrix::new(&*vec![(a*d + b*b), (a*c + a*b), (c*d + b*d) ,
                                                         (c*c + a*d)], 2, 2))
    }
}



