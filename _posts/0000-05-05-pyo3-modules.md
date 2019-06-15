# PyO3: Making a Module

```rust
use pyo3::prelude::*;
use pyo3::types{PyDate};
use pyo3::wrap_pyfunction;

use date_impl::to_timestamp;

#[pyfunction]
fn seconds_before(py: Python, d: &PyDate, seconds: i64) -> PyResult<Py<PyDate>> {
    let timestamp = to_timestamp(&d);

    PyDate::from_timestamp(py, timestamp - seconds)
}

#[pymodule]
fn date_ex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(seconds_before))?;

    Ok(())
}
```

<br/>

```python
>>> from pomodule.date_ex import seconds_before
>>> from datetime import date
>>> seconds_before(date(2019, 2, 20), int(1e6))
datetime.date(2019, 2, 9)
```
<fragment/>

<br/>

```python
>>> seconds_before(date(1, 1, 1), int(1e6))
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: year 0 is out of range
```
<fragment/>

--

# PyO3: Making a class

```rust
use pyo3::prelude::*;

#[pyclass]
struct Point {
    x: i32,
    y: i32,
}


#[pymethods]
impl Point {
    #[new]
    fn __new__(obj: &PyRawObject, x: i32, y: i32) -> PyResult<()> {
        obj.init(|| Point { x: x, y: y })
    }

    fn norm(&self, py: Python<'_>) -> f64 {
        ((self.x as f64).powf(2.) + (self.y as f64).powf(2.)).sqrt()
    }
}
```

<br/>
```rust
#[pymodule]
fn classy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;

    Ok(())
}
```
<fragment/>
<br>

```python
>>> from pomodule.classy import Point
>>> p = Point(3, 4)
>>> p
<Point object at 0x7f4a27c18270>
>>> Point(3, 4).norm()
5.0
```
<fragment/>

