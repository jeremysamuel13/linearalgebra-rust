mod structures;
use structures::matrix;

fn main() {
    use crate::structures::matrix::Matrix;
    /*let x = Matrix::new(&*vec![2.0, 3.0, 3.0, 2.0], 2, 2);
    let y = Matrix::new(&*vec![4.0, 1.0, 4.0, 3.0], 2, 2);
    //let z = x*y;  //MATRIX MULT
    let a = x - y;*/

    let y = matrix![1.0];
    println!("{:?}", y);
}
