def pascal_row(n):
    row = [0] * n
    row[0] = 1

    for i in range(1, n):
        curr = 1
        for j in range(1, i + 1):
            last = curr
            curr = row[j]
            row[j] = last + curr

    return row

