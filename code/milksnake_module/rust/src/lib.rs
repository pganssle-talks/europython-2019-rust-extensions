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

#[no_mangle]
pub unsafe extern "C" fn deallocate_vec(ptr: *mut u32, len: size_t) {
    let len = len as usize;
    drop(Vec::from_raw_parts(ptr, len, len));
}
