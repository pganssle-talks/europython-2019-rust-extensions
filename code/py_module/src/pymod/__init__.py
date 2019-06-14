import math

def sieve(n):
    numbers = list(range(2, n + 1))

    for i in range(2, int(math.sqrt(n) + 1)):
        if numbers[i - 2] != 0:
            for j in range(i * i, n + 1, i):
                numbers[j - 2] = 0

    return [x for x in numbers if x != 0]
