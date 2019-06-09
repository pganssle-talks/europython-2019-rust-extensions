extern crate pyo3;

pub mod date_ex;
pub mod classy;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyBytes};
use pyo3::wrap_pyfunction;

fn pascal_row_impl(n: usize) -> Vec<u32> {
    let mut row : Vec<u32> = Vec::with_capacity(n);
    row.resize(n, 0);       // Allocate an array of 0s
    row[0] = 1;

    let mut last : u32;
    for i in 1..n {
        let mut curr : u32 = 1;
        for j in 1..(i + 1) {
            last = curr;
            curr = row[j];
            row[j] = last + curr;
        }
    }

    row
}


#[pyfunction]
fn pascal_row(py: Python, n: usize) -> PyObject {
    let list = PyList::new(py, &pascal_row_impl(n));

    list.to_object(py)
}

#[pyfunction]
fn print_bytes(_py: Python, x: Vec<u8>) {
    println!("{:?}", x)
}

#[pyfunction]
fn return_bytes(_py: Python) -> Vec<u8> {
    let x : Vec<u8> = vec![1, 2, 3, 4];
    x
}


#[pymodule]
fn backend(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(pascal_row))?;
    m.add_wrapped(wrap_pyfunction!(print_bytes))?;
    m.add_wrapped(wrap_pyfunction!(return_bytes))?;

    Ok(())
}
