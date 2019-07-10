# Opportunities for Improvement: FFI approach
<br/>
```rust
#[no_mangle]
pub unsafe extern "C" fn sieve(n: usize, size_out: *mut size_t) -> *mut u32 {
    let mut s = sieve_impl(n);
    *size_out = s.len() as size_t;
    let rv = s.as_mut_ptr();
    mem::forget(s); // prevent rust from de-allocating this
    rv
}

#[no_mangle]
pub unsafe extern "C" fn deallocate_vec(ptr: *mut u32, len: size_t) {
    let len = len as usize;
    drop(Vec::from_raw_parts(ptr, len, len));
}
```
<br/>
Procedural macro?

```rust
#[ffi_out]
fn sieve(n : usize) -> Vec<T> {
    ...
}
```

--

# Opportunities for Improvement: FFI approach

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
Convenience wrappers?
<br/>

```python
def return_list(f):
    @functools.wraps(f)
    def wrapper_func(*args):
        l = ffi.new("size_t *")
        arr = f(*args, l)
        size = l[0]

        try:
            out = [arr[i] for i in range(size)]
        finally:
            deallocate_vec(arr, size)

        return out
    return wrapper_func

sieve = return_list(lib.sieve)
```

--

# Opportunities for Improvement: PyO3
<br/>
<img src="images/pyo3_gh_screenshot.png"
     alt="Screenshot of PyO3's github"
     style="max-height: 450px; height: 450px"
     />

<br/>
<b>Contributions welcomed:</b>: https://github.com/pyo3/pyo3
