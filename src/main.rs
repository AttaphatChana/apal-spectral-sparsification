#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra_sparse::{coo::CooMatrix, csr::CsrMatrix, csc::CscMatrix};

// struct Original {
//     degree: CscMatrix<f32>,
//     adj:CscMatrix<f32>,
//     laplacian: CscMatrix<f32>,
//
// }


fn edge_to_coo(file: &str, n:usize) -> CooMatrix<i32> {
    let f = File::open(file);
    let reader = BufReader::new(f.unwrap());
    let mut lines = reader.lines();
    let mut coo:CooMatrix<i32> = CooMatrix::new(n, n);
    for line in lines.by_ref() {
        let line = line.unwrap();
        let line = line.split_ascii_whitespace().collect::<Vec<_>>();
        let i = line[0].parse::<usize>().unwrap();
        let j = line[1].parse::<usize>().unwrap();
        coo.push(i,j,1);
        coo.push(j,i,1);

    }
    return coo;
}

fn laplacian(coo:&CooMatrix<i32>) -> CscMatrix<i32> {
    let mut degree_matrix = CooMatrix::new(coo.nrows(), coo.ncols());
    let mut adj_matrix = CscMatrix::from(coo);
    for col in adj_matrix.col_iter().enumerate().take(coo.ncols()) {
        let (pos,degree) = col;
        let degree = degree.nnz() as i32;
        degree_matrix.push(pos,pos,degree)

    }
    let mut degree_matrix:CscMatrix<i32> = CscMatrix::from(&degree_matrix);
    let laplace = degree_matrix - adj_matrix;

    return laplace;

}
fn approx_rf(coo:&CooMatrix<i32>, n:usize) -> CooMatrix<f64> {

    //todo: JL transform for fast direct calculation
}

fn main() {
    //use nalgebra_sparse::{coo::CooMatrix, csr::CsrMatrix, csc::CscMatrix};

    // Initialize a matrix with all zeros (no explicitly stored entries).
    let mut coo = CooMatrix::new(4, 4);
    // Or initialize it with a set of triplets
    coo = CooMatrix::try_from_triplets(4, 4, vec![1, 2], vec![0, 1], vec![3.0, 4.0]).unwrap();


    // Push a few triplets
    coo.push(2, 0, 1.0);
    coo.push(0, 1, 2.0);

    // Convert to other matrix formats
    let csr = CsrMatrix::from(&coo);
    let csc = CscMatrix::from(&coo);
    let coo = edge_to_coo("./edge.txt", 5);
    let laplace = laplacian(&coo);
    // println!("{:?}", csc);
    // println!("{:?}", csr);
    println!("{:?}", coo);
    println!("{:?}", laplace);

}