use std::io;

const NUM: usize = 4;

fn main() {
    let mut m = String::new();
    let mut k = String::new();
    let mut n = String::new();

    println!("Enter the dimensions of the matrices:");
    println!("Enter the number of rows of the 1st matrix (k): ");
    read_number(&mut k);
    println!("Enter the number of columns of the 1st matrix / rows of the 2nd matrix (m): ");
    read_number(&mut m);
    println!("Enter the number of columns of the 2nd matrix (n): ");
    read_number(&mut n);

    let k: usize = k.trim().parse().expect("Invalid input for k");
    let m: usize = m.trim().parse().expect("Invalid input for m");
    let n: usize = n.trim().parse().expect("Invalid input for n");

    let mut matrix1: Vec<Vec<i32>> = vec![vec![0; m]; k]; // k x m
    let mut matrix2: Vec<Vec<i32>> = vec![vec![0; n]; m]; // m x n
    let mut matrix_result: Vec<Vec<i32>> = vec![vec![0; n]; k]; // k x n

    initialize_matrix(k, m, &mut matrix1); // fill matrix1 (k x m)
    initialize_matrix(m, n, &mut matrix2); // fill matrix2 (m x n)
    multiply_matrices(&matrix1, &matrix2, &mut matrix_result); // multiply

    println!("Result of matrix multiplication ({}x{}):", k, n);
    for row in matrix_result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn read_number(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Error while reading input number!");
}

fn initialize_matrix(rows: usize, cols: usize, matrix: &mut Vec<Vec<i32>>) {
    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = (i * NUM + j) as i32;
        }
    }
}

fn multiply_matrices(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
    result: &mut Vec<Vec<i32>>,
) {
    let k = a.len(); // rows of A
    let m = a[0].len(); // columns of A
    let n = b[0].len(); // columns of B

    for i in 0..k {
        for j in 0..n {
            for l in 0..m {
                result[i][j] += a[i][l] * b[l][j];
            }
        }
    }
}
