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

