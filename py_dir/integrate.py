#!/usr/bin/env python
from numpy import loadtxt, empty
import matplotlib.pyplot as plt


def f(x):
    return x**4 - 2 * x + 1


def main():
    N = 10
    a = 0.0
    b = 2.0
    h = (b - a) / N

    s = 0.5 * f(a) + 0.5 * f(b)
    for k in range(1, N):
        s += f(a + k * h)

    print(f"Integral result py: {h * s}")

    data = loadtxt("excercises/velocities.txt")

    t, v = data[:, 0], data[:, 1]

    h = 1
    N = len(t)

    # s = 0.5*v[0] #+ 0.5*v[-1]
    distance = empty(N)
    distance[0] = 0
    for k in range(1, N):
        distance[k] = distance[k - 1] + (v[k] + v[k - 1]) / 2

    print(f"Distance: {distance[33]}")
    plt.plot(t, distance, label="distance")
    plt.plot(t, v, label="velocity")
    plt.legend()


if __name__ == "__main__":
    main()
