# Pascal's Triangle: Python Version

```python
def pascal_row(n):
    row = [0] * n
    row[0] = 1

    for i in range(1, n):
        curr = 1
        for j in range(1, i + 1):
            last = curr
            curr = row[j - 1]
            row[j - 1] = last + curr

    return row
```

## Output:

```
>>> for r in range(1, 8):
...     rowstr = ', '.join(map(str, pascal_row(r)))
...     print(f"{rowstr:^40}")
...
                   1
                  1, 1
                1, 2, 1
               1, 3, 3, 1
             1, 4, 6, 4, 1
           1, 5, 10, 10, 5, 1
         1, 6, 15, 20, 15, 6, 1
 ```

Notes:

Here is a simple program for generating Pascal's Triangle in Python, row-by-row,
in a list that is updated in place. As you can see, the code is pretty simple,
and when I print out the rows it does the right thing.

--

# C API

```C
static PyObject* pascal_row(PyObject* self, PyObject* n_rows) {
    size_t n;
    if ((n = PyLong_AsSize_t(n_rows)) == (size_t)-1 && PyErr_Occurred()) {
        return NULL;
    }

    int* row = calloc(n, sizeof(int));
    if (row == NULL) {
        PyErr_NoMemory(); // raise MemoryError()
        return NULL;
    }

    // Populate the C array containing the Pascal's triangle row
    row[0] = 1;
    int curr = 0, last = 0;
    for (size_t i = 2; i <= n; ++i) {
        curr = row[0];

        for (size_t j = 1; j <= i; ++j) {
            last = curr;
            curr = row[j];
            row[j] = last + curr;
        }
    }

    // Convert to Python list
    PyObject* rv = PyList_New(n), *obj = NULL;
    if (rv == NULL) { goto cleanup; }

    for (size_t i = 0; i < n; ++i) {
        if ((obj = PyLong_FromLong(row[i])) == NULL || // int -> Py int
                PyList_SetItem(rv, i, obj)) {          // rv[i] = obj
            Py_DECREF(rv);          // On error, remove list
            rv = NULL;
            goto cleanup;
        }
    }

cleanup:
    free(row);
    return rv;
}
```

Notes:

Here's the same program, written with the C API.

--

# C API: Performance benefits

```
In [1]: from cmod import ext, purepy
In [2]: %timeit ext.pascal_row(500)
81.8 µs ± 843 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)

In [3]: %timeit purepy.pascal_row(500)
11.2 ms ± 101 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)

In [4]: 11.2 / 0.082
Out[4]: 136.58536585365852
```

<br/>
<br/>
<table>
    <tr>
        <td>N</td>
        <td>Pure Python</td>
        <td>C Extension</td>
        <td>Ratio</td>
    </tr>
    <tr>
        <td>1</td>
        <td>401 ns</td>
        <td>159 ns</td>
        <td>2.5</td>
    </tr>
    <tr>
        <td>10</td>
        <td>5.18 μs</td>
        <td>221 ns</td>
        <td>23</td>
    </tr>
    <tr>
        <td>100</td>
        <td>396 μs</td>
        <td>5.4 μs</td>
        <td>73</td>
    </tr>
    <tr>
        <td>1000</td>
        <td>47.4 ms</td>
        <td>302 μs</td>
        <td>157</td>
    </tr>
</table>


--

# C API: Downsides

- Manual memory management
- Manual reference counting (`Py_INCREF`, `Py_DECREF`)
- No memory safety

```C
    for (size_t i = 2; i <= n; ++i) {
        curr = row[0];

        for (size_t j = 1; j <= i; ++j) {
            last = curr;
            curr = row[j];
            row[j] = last + curr;
        }
    }
```
<!-- .element class="fragment" -->

Notes:

Oops, that `<=` should be a `<`, we're actually reading and writing past the end of this array
