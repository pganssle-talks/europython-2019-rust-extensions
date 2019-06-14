#include <Python.h>
#include <math.h>

static PyObject* sieve_impl(PyObject* self, PyObject* max_n) {
    size_t n;
    if ((n = PyLong_AsSize_t(max_n)) == (size_t)-1 && PyErr_Occurred()) { return NULL; }

    // Populate the C array
    int* sieve = malloc((n - 1) * sizeof(int));
    if (sieve == NULL) {
        PyErr_NoMemory(); // raise MemoryError()
        return NULL;
    }

    for (size_t i = 2; i < n + 1; ++i) { sieve[i - 2] = (int)i; }

    // Sieve out composite numbers
    size_t lim = (size_t)sqrt((double)n) + 1;
    for (size_t i = 2; i < lim; ++i) {
        if (sieve[i - 2] != 0) {
            for (size_t j = (i * i); j < (n + 1); j += i) {
                sieve[j - 2] = 0;
            }
        }
    }

    // Convert to Python list
    size_t num_primes = 0;  // Calculate total size of list
    for (size_t i = 0; i < n - 1; ++i) { if (sieve[i]) { num_primes++; } }

    PyObject* rv = PyList_New(num_primes);
    if (rv == NULL) { goto cleanup; }
    PyObject * obj = NULL;
    size_t j = 0;
    for (size_t i = 0; i < n - 1; ++i) {
        if (!sieve[i]) { continue; }
        if ((obj = PyLong_FromLong(sieve[i])) == NULL || // int -> Py int
                   PyList_SetItem(rv, j++, obj)) {       // rv[i] = obj
            Py_DECREF(rv);  rv = NULL;                   // On error, remove list
            goto cleanup;
        }
    }
cleanup:
    free(sieve);
    return rv;
}

static PyMethodDef methods[] = {
    { "sieve", sieve_impl, METH_O, "Calculate all primes below n" },
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef ext = {
    PyModuleDef_HEAD_INIT,
    "ext",
    "Pascal row module",
    -1,
    methods
};

PyMODINIT_FUNC PyInit_ext(void) {
    return PyModule_Create(&ext);
}
