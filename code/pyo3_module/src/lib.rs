extern crate pyo3;

pub mod classy;
pub mod date_ex;

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

fn sieve_impl(n: usize) -> Vec<u32> {
    let mut sieve: Vec<u32> = (2..((n + 1) as u32)).collect();
    let lim: usize = ((n as f64).sqrt() + 1.0) as usize;

    for i in 2usize..lim {
        if sieve[i - 2] != 0 {
            let mut j = i * i;
            while j < n + 1 {
                sieve[j - 2] = 0;
                j += i;
            }
        }
    }

    sieve.into_iter().filter(|&x| x != 0).collect()
}

#[pyfunction]
fn sieve(py: Python, n: u32) -> &PyList {
    let list = PyList::new(py, &sieve_impl(n as usize));

    list
}

#[pyfunction]
fn print_bytes(_py: Python, x: Vec<u8>) {
    println!("{:?}", x)
}

#[pyfunction]
fn return_bytes(_py: Python) -> Vec<u8> {
    let x: Vec<u8> = vec![1, 2, 3, 4];
    x
}

#[pymodule]
fn backend(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sieve))?;
    m.add_wrapped(wrap_pyfunction!(print_bytes))?;
    m.add_wrapped(wrap_pyfunction!(return_bytes))?;

    Ok(())
}
