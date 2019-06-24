# distutils: language = c++
from libcpp.vector cimport vector
import math

cdef vector[int] sieve_impl(unsigned int n):
    cdef unsigned int i, j
    cdef vector[int] sieve = range(2, n + 1)

    cdef int lim = int(math.sqrt(n)) + 1
    for i in range(2, lim):
        if sieve[i - 2] != 0:
            j = i * i
            while j < n + 1:
                sieve[j - 2] = 0
                j += i

    return [x for x in sieve if x != 0]

cpdef list sieve(unsigned int n):
    return list(sieve_impl(n))

cpdef list sieve_novector(unsigned int n):
    cdef unsigned int i, j
    cdef list sieve = list(range(2, n + 1))

    cdef int lim = int(math.sqrt(n)) + 1
    for i in range(2, lim):
        if sieve[i - 2] != 0:
            j = i * i
            while j < n + 1:
                sieve[j - 2] = 0
                j += i

    return [x for x in sieve if x != 0]
