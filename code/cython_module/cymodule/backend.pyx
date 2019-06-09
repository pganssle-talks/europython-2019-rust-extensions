from cpython cimport array
import array

cdef int[:] pascal_row_impl(unsigned int n):
    cdef int[:] row = array.array('i', [0] * n)

    cdef int last, curr
    cdef unsigned int i, j

    row[0] = 1

    for i in range(1, n):
        curr = 1
        for j in range(1, i + 1):
            last = curr
            curr = row[j]
            row[j] = last + curr

    return row

cpdef list pascal_row(unsigned int n):
    return list(pascal_row_impl(n))



