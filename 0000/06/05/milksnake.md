# C FFI Bindings

```rust
use std::mem;
use std::os::raw::c_ulonglong;

type size_t = c_ulonglong;

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

#[no_mangle]
pub unsafe extern "C" fn sieve(n: usize, size_out: *mut size_t) -> *mut u32 {
    let mut s = sieve_impl(n);
    *size_out = s.len() as size_t;
    let rv = s.as_mut_ptr();
    mem::forget(s); // prevent rust from de-allocating this
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

# C FFI Bindings with Milksnake

```python
from msmodule._native import ffi, lib

def sieve(n):
    l = ffi.new("size_t *")

    # Get a C array of length l
    arr = lib.sieve(n, l)
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

In [4]: %timeit cext.sieve(100000)
827 µs ± 12.3 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [5]: %timeit pyo3_back.sieve(100000)
684 µs ± 14.9 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [6]: %timeit milksnake_back.sieve(100000)
1.2 ms ± 18.8 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```
