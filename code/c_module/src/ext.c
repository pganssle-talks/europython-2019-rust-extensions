#include <Python.h>

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
    PyObject* rv = PyList_New(n);
    if (rv == NULL) {
        goto cleanup;
    }

    PyObject * obj = NULL;
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

static PyMethodDef methods[] = {
    { "pascal_row", pascal_row, METH_O, "Get the nth row of Pascal's triangle" },
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
