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





