use std::io;
const NUM: usize = 4;
use rayon::prelude::*;


fn main(){
    let mut k: String = String::from(" ");
    let mut m: String = String::from(" ");
    let mut n: String = String::from(" ");
    println!("This Rust code is for Matrix Multiplication!");
    println!("Enter the first dimension of the 1st matrix: k: ");
    read_input(&mut k);
    println!("Enter the second dimension of the 2nd matrix: m: ");
    read_input(&mut m);
    println!("Enter the second dimension of the 2nd matrix: n: ");
    read_input(&mut n);
    let k: usize = k.trim().parse().expect("Error in converting the number!");
    let m: usize = m.trim().parse().expect("Error in converting the number!");
    let n: usize = n.trim().parse().expect("Error in converting the number!");
    println!("You entered the following numbers: {}, {} and {} as the dimensions!", k, m, n);
    let mut matrix1: Vec<Vec<i32>> = vec![vec![0;m];k];
    let mut matrix2: Vec<Vec<i32>> = vec![vec![0;n];m];
    let mut matrix_result: Vec<Vec<i32>> = vec![vec![0;n]; k];
    initialize_matrix(k, m, &mut matrix1);
    initialize_matrix(m, n, & mut matrix2);
    println!("---------Matrix Multiplication Result-----");
    multiply_matrices(&matrix1, &matrix2, &mut matrix_result);
    for i in 0..k{
        for j in 0..n{
            print!("{} ", matrix_result[i][j]);
        }
        println!(" ");
    }
}

fn read_input(x: &mut String) -> &mut String{
    io::stdin()
    .read_line(x)
    .expect("Error in reading the input number!");
    x
}

fn initialize_matrix(a: usize, b: usize, x: &mut Vec<Vec<i32>>){
    for i in 0..a{
        for j in 0..b{
            x[i][j] = (i * NUM + 2*j) as i32;
        }
    }
}

fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>) {
    let m = a[0].len();
    let n = b[0].len();

    result.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..n {
            for l in 0..m {
                row[j] += a[i][l] * b[l][j];
            }
        }
    });
}