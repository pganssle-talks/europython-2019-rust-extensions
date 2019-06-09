use std::mem;
use std::os::raw::c_ulonglong;

type size_t = c_ulonglong;

fn pascal_row_impl(n: usize) -> Vec<u32> {
    let mut row: Vec<u32> = Vec::with_capacity(n);
    row.resize(n, 0); // allocate an array of 0s
    row[0] = 1;

    let mut last: u32;
    for i in 1..n {
        let mut curr: u32 = 1;
        for j in 1..(i + 1) {
            last = curr;
            curr = row[j];
            row[j] = last + curr;
        }
    }

    row
}

#[no_mangle]
pub unsafe extern "C" fn pascal_row(n: usize, size_out: *mut size_t) -> *mut u32 {
    let mut s = pascal_row_impl(n);
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
