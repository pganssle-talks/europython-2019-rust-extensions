# Parting Thoughts

<img src="external-images/could-should.jpg"
     alt="You were so preoccupied with whether or not you could,
          you didn't stop to think if you shold"
     style="width:70%; border: 5px solid; border-color: black;">

--

# Cython for backends

```python
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
```
<br/>

```python

In [1]: from cmod import ext as cext
In [2]: from cymodule import backend as cython_ext
In [3]: import pymod

In [4]: %timeit pymod.sieve(100000)
23.9 ms ± 1.12 ms per loop (mean ± std. dev. of 7 runs, 10 loops each)

In [5]: %timeit cext.sieve(100000)
866 µs ± 12 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [6]: %timeit cython_ext.sieve(100000)
3.1 ms ± 114 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)
```

--

# What to choose?

<img src="external-images/apple-choice-raquel-martinez-cropped.jpg"
     alt="A person choosing between two apples"
     style="max-height: 750px;"
     />

--

# Speed

<table>
  <tr>
    <th>n</th>
    <th>Python</th>
    <th>Cython</th>
    <th>Milksnake</th>
    <th>PyO3</th>
    <th>C</th>
  </tr>
  <tr>
    <td>1</td>
    <td>1200 ns</td>
    <td>439 ns</td>
    <td>1700 ns</td>
    <td>298 ns</td>
    <td>68 ns</td>
  </tr>
  <tr>
    <td>100</td>
    <td>12.7 μs</td>
    <td>2.8 μs</td>
    <td>3.7 μs</td>
    <td>1.1 μs</td>
    <td>0.51 μs</td>
  </tr>
  <tr>
    <td>1000</td>
    <td>156.8 μs</td>
    <td>26.7 μs</td>
    <td>15.4 μs</td>
    <td>5.8 μs</td>
    <td>6.1 μs</td>
  </tr>
  <tr>
    <td>100000</td>
    <td>23.0 ms</td>
    <td>3.0 ms</td>
    <td>1.2 ms</td>
    <td>0.724 ms</td>
    <td>0.745 ms</td>
  </tr>
  <tr>
    <td>1000000</td>
    <td>354.8 ms</td>
    <td>33.0 ms</td>
    <td>13.0 ms</td>
    <td>7.8 ms</td>
    <td>8.6 ms</td>
  </tr>
</table>

<br/>
<br/>

## Caveats

- Not necessarily a representative benchmark
- Not particularly optimized


--

# FFI vs. API

### FFI

<div style="display:flex">
<div style="width: 50%">
<h4>Pros</h4>
<ul>
    <li> More portable interface </li>
    <li> Smaller rust dependency </li>
    <li> Faster in PyPy and for certain types of interface </li>
</ul>
</div><div style="width: 50%">
<h4>Cons</h4>
<ul>
<li> Runtime dependency on `milksnake` and `cffi` </li>
<li> No support for Python-specific types (e.g. `datetime`, `list`, `tuple`) </li>
<li> May require memory management in Python </li>
<li> Interface is designed in unsafe Rust </li>
</ul>
</div>
</div>

<br>
<br>

### API
<div style="display:flex">
<div style="width: 50%">
<h4>Pros</h4>
<ul>
    <li> Safe wrappers written for most of the API </li>
    <li> No runtime dependencies </li>
    <li> Can work directly with Python containers and objects </li>
    <li> Can easily call Python functions from Rust </li>
    <li> Manages the GIL and reference counts for you </li>
</ul>
</div><div style="width: 50%">
<h4>Cons</h4>
<ul>
    <li> PyO3's API is still quite unstable </li>
    <li> Requires nightly rust </li>
    <li> Needs significant speed updates </li>
    <li> Still quite buggy </li>
</ul>
</div>
</div>

