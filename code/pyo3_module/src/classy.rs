use pyo3::prelude::*;

#[pyclass]
struct Point {
    x: i32,
    y: i32,
}

#[pymethods]
impl Point {
    #[new]
    fn __new__(obj: &PyRawObject, x: i32, y: i32) {
        obj.init(Point { x: x, y: y });
    }

    fn norm(&self, _py: Python<'_>) -> f64 {
        ((self.x as f64).powf(2.) + (self.y as f64).powf(2.)).sqrt()
    }
}

#[pymodule]
fn classy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;

    Ok(())
}
