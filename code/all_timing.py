import statistics
import timeit

from collections import defaultdict

from pomodule.backend import sieve as pyo3_sieve
from msmodule import sieve as ms_sieve
from cymodule.backend import sieve as cy_sieve
from cmod.ext import sieve as c_sieve
from pymod import sieve as py_sieve


# Test that these are all comparable

for N in [5, 1000000]:
    py_result = py_sieve(N)
    for f in [pyo3_sieve, ms_sieve, cy_sieve, c_sieve]:
        assert f(N) == py_result, (N, f)

module_globals = {
    'pyo3_sieve': pyo3_sieve,
    'ms_sieve': ms_sieve,
    'cy_sieve': cy_sieve,
    'c_sieve': c_sieve,
    'py_sieve': py_sieve,
}

def run_timing(stmt, k=5):
    timer = timeit.Timer(stmt=stmt, globals=module_globals)

    # We should run it for about 0.5 seconds each, or 10 runs, whichever is longer
    n, total_time = timer.autorange()
    if total_time < 0.5:
        n *= 2

    # Now run it k times and get mean and standard deviation
    values = []
    for _ in range(k):
        values.append(timer.timeit(n) / n)

    val_mean = statistics.mean(values)
    val_std = statistics.stdev(values)

    return val_mean, val_std, n

def get_time_range(res):
    if res < 1e-6:
        return 'ns', 1e9
    elif res < 1e-3:
        return 'μs', 1e6
    elif res < 1:
        return 'ms', 1e3
    else:
        return 's', 1

# Run speed tests
k = 5
n_vals = [1, 100, 1000, 100000, 1000000]


backends = {
    'Python': 'py_sieve',
    'Cython': 'cy_sieve',
    'Milksnake': 'ms_sieve',
    'PyO3': 'pyo3_sieve',
    'C': 'c_sieve',
}

table = []
backend_header = list(backends.keys())
table.append(['n'] + backend_header)

print("Running timings")
for n_val in n_vals:
    print(f"\nN = {n_val}\n")
    row = [n_val]
    table.append(row)
    for backend in backend_header:
        func_name = backends[backend]
        val_mean, val_std, n = run_timing(stmt=f"{func_name}({n_val})", k=k)

        unit, scaling = get_time_range(val_mean)
        val_mean, val_std = map(lambda x: x * scaling, (val_mean, val_std))

        row.append(f"{val_mean:0.1f} {unit}")

        print(f"{backend}: {val_mean:0.3f} {unit} (± {val_std:0.2f})")

# Output to convenient HTML table
html_table = "<table>"
for row in table:
    html_table += "  <tr>\n"
    for cell in row:
        html_table += f"    <td>{cell}</td>\n"
    html_table += "  </tr>\n"
html_table += "</table>\n"
title = "Sieve of Eratosthenes Performance"

HTML = f"<html><head><title>{title}</title>\n<body>\n{html_table}</body></html>"
with open("sieve_timing.html", "w") as f:
    f.write(HTML)
print("Output written to HTML file")
