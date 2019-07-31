import time
import libpyrust as rf
import numpy as np

N=5000

start = time.time()
rust_pi = rf.calc_pi(N)
stop = time.time()

print("rust_pi = {}, {} seconds".format(rust_pi, stop - start))

def calc_pi(N=1000):
    radius = float(N);
    area = radius ** 2.0
    sum = 0.0

    for i in range(int(radius)):
        for j in range(int(radius)):
            if (i**2.0 + j**2.0)**0.5 <= radius:
                sum += 1.0

    pi = 4.0 * sum / area
    return pi

start = time.time()
python_pi = calc_pi(N)
stop = time.time()

print("python_pi = {}, {} seconds".format(python_pi, stop - start))
