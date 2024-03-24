from pyo3_example import sum_as_string, new_list, fibonacci


def main():
    print(sum_as_string(1, 2))

    li = new_list(42, 7)
    print(li)

    n = 20
    print("The first {n} Fibonacci numbers are:")
    for i in range(1, n + 1):
        print(f"{i}: {fibonacci(i)}")


if __name__ == "__main__":
    main()
