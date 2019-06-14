# cython: profile=True
from cpython cimport array
import array
import math

cdef int[:] sieve_impl(unsigned int n):
    cdef unsigned int i, j
    cdef int[:] sieve = array.array('i', [0] * (n - 1))

    cdef int lim = int(math.sqrt(n)) + 1
    for i in range(2, n + 1):
        sieve[i - 2] = i

    for i in range(2, lim):
        if sieve[i - 2] != 0:
            j = i * i
            while j < n + 1:
                sieve[j - 2] = 0
                j += i

    i = 0
    len_sieve = len(sieve)
    while i < len_sieve and sieve[i] != 0:
        i += 1

    j = i + 1
    while j < len(sieve):
        if sieve[j] != 0:
            sieve[i], sieve[j] = sieve[j], sieve[i]
            i += 1
        j += 1

    sieve = sieve[0:i]

    return sieve

cpdef list sieve(unsigned int n):
    return list(sieve_impl(n))



