# FFI Bindings

```rust
use std::mem;
use std::os::raw::{c_ulonglong};

type size_t = c_ulonglong;

fn pascal_row_impl(n: usize) -> Vec<u32> {
    let mut row : Vec<u32> = Vec::with_capacity(n);
    row.resize(n, 0);       // allocate an array of 0s
    row[0] = 1;

    let mut last : u32;
    for i in 1..n {
        let mut curr : u32 = 1;
        for j in 1..(i + 1) { last = curr;
            curr = row[j];
            row[j] = last + curr;
        }
    }

    row
}

#[no_mangle]
pub unsafe extern "c" fn pascal_row(n: usize, size_out: *mut size_t) -> *mut u32 {
    let mut s = pascal_row_impl(n);
    *size_out = s.len() as size_t;
    let rv = s.as_mut_ptr();
    mem::forget(s);         // Prevent rust from de-allocating this
    rv
}
```

<br>

```rust
#[no_mangle]
pub unsafe extern "c" fn deallocate_vec(ptr: *mut u32, len: size_t) {
    let len = len as usize;
    drop(vec::from_raw_parts(ptr, len, len));
}
```
<fragment/>

--

# FFI Bindings with Milksnake

```python
from msmodule._native import ffi, lib

def pascal_row(n):
    l = ffi.new("size_t *")

    # Get a C array of length l
    arr = lib.pascal_row(n, l)
    size = l[0]

    try:
        out = [arr[i] for i in range(size)]
    finally:
        lib.deallocate_vec(arr, size)

    return out
```
<br/>

```python

In [1]: from cmod import ext as cext
In [2]: from pomodule import backend as pyo3_back
In [3]: import msmodule as milksnake_back
In [4]: %timeit cext.pascal_row(1000)
234 µs ± 1.36 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [5]: %timeit pyo3_back.pascal_row(1000)
466 µs ± 4.4 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [6]: %timeit milksnake_back.pascal_row(1000)
493 µs ± 3.34 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

