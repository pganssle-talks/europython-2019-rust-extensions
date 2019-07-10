# Rust API Bindings: PyO3

```rust
fn sieve_impl(n: usize) -> Vec<u32> {
    let mut sieve: Vec<u32> = (2..((n + 1) as u32)).collect();
    let lim : usize = ((n as f64).sqrt() + 1.0) as usize;

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
```

--

# Rust API Bindings: PyO3

<br/><br/>
```python
>>> from pomodule.backend import sieve
>>> sieve(5)
[2, 3, 5]

>>> sieve(20)
[2, 3, 5, 7, 11, 13, 17, 19]
```

<br>

```
In [1]: from pomodule import backend as rust
In [2]: from cmod import ext as cext
In [3]: import pymod

In [4]: %timeit pymod.sieve(100000)
23.5 ms ± 540 µs per loop (mean ± std. dev. of 7 runs, 10 loops each)

In [5]: %timeit cext.sieve(100000)
698 µs ± 24.3 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [6]: %timeit rust.sieve(100000)
687 µs ± 12.6 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

--

## FFI Layer

<div style="display: flex; justify-content: space-between;">
<div style="width: 50%">
<h3>Rust</h3>
</div>
<div style="width: 50%">
<h3>C</h3>
</div>
</div>

#### Functions
<div style="display: flex; justify-content: space-between;">
<div style="width: 50%">
<pre>
<code class="lang-rust hljs">#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyDateTime_CAPI {
    ...
    pub Date_FromDate: unsafe extern "C" fn(
        year: c_int, month: c_int, day: c_int, cls: \*mut PyTypeObject,
    ) -> \*mut PyObject
    ...
}</code></pre>
</div>
<div style="width: 50%">
<pre>
<code class="lang-C hljs">

typedef struct {
    ...
    PyObject \*(\*Date_FromDate)(
        int, int, int, PyTypeObject\*
    );
    ...
} PyDateTime_CAPI;</code></pre>
</div>
</div>

<br>

#### Data Structures
<div style="display: flex; justify-content: space-between;">
<div style="width: 50%">
<pre>
<code class="lang-rust hljs">#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyDateTime_Date {
    pub ob_base: PyObject,
    pub hashcode: Py_hash_t,
    pub hastzinfo: c_char,
    pub data: [c_uchar; _PyDateTime_DATE_DATASIZE],
}</code></pre>
</div>
<div style="width: 50%">
<pre>
<code class="lang-C hljs">
typedef struct
{
    PyObject_HEAD
    Py_hash_t hashcode;
    char hastzinfo;             /\* boolean flag \*/
    unsigned char data[_PyDateTime_DATE_DATASIZE];
} PyDateTime_Date;</code></pre>
</div>
</div>

<br>

#### Macros
<div style="display: flex; justify-content: space-between;">
<div style="width: 50%">
<pre>
<code class="lang-rust hljs">#[repr(C)]
pub unsafe fn PyDate_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, PyDateTimeAPI.DateType) as c_int
}</code></pre>
</div>
<div style="width: 50%">
<pre>
<code class="lang-C hljs">
#define PyDate_Check(op) \
    PyObject_TypeCheck(op, PyDateTimeAPI->DateType)

</code></pre>
</div>
</div>

--

# PyO3: How it works

## Safe Rust Layer
```rust
impl PyDate {
    pub fn new<'p>(py: Python<'p>, year: i32, month: u8, day: u8) -> PyResult<&'p PyDate> {
        unsafe {
            let ptr = (PyDateTimeAPI.Date_FromDate)(
                year,
                c_int::from(month),
                c_int::from(day),
                PyDateTimeAPI.DateType,
            );
            py.from_owned_ptr_or_err(ptr)
        }
    }

    ...
}
```
<br/>

<div style="display: flex; justify-content: space-around;">
<img src="images/PyDateAPI.png"
    alt="PyDate API showing 'new' and 'from_timestamp'"
    style="border-color:black; border: 2px solid;"/>

<img src="images/PyDateAccessTrait.png"
    alt="Showing the PyDateAccess trait to get the individual components"
    style="border-color:black; border: 2px solid;"/>

</div>
