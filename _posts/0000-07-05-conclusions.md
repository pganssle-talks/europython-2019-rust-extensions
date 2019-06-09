# Parting Thoughts

<img src="external-images/could-should.jpg"
     alt="You were so preoccupied with whether or not you could,
          you didn't stop to think if you shold"
     style="width:70%; border: 5px solid; border-color: black;">

--

# Cython for backends

```python
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
```
<br/>

```python
In [1]: from cmod import ext as cext
In [2]: from cymodule import backend as cy_ext
In [3]: %timeit cext.pascal_row(1000)
230 µs ± 4.88 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [4]: %timeit cy_ext.pascal_row(1000)
445 µs ± 2.88 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

--

# What to choose?

<img src="external-images/apple-choice-raquel-martinez-cropped.jpg"
     alt="A person choosing between two apples"
     style="max-height: 750px;"
     />

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

