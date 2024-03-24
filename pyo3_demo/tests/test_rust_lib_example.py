from rust_lib_example import sum_as_string, new_list, fibonacci


def test_sum_as_string():
    assert sum_as_string(1, 2) == "3"


def test_new_list():
    assert new_list(42, 7) == [42] * 7


def test_fibonacci():
    assert fibonacci(0) == 0
    assert fibonacci(1) == 1
    assert fibonacci(2) == 1
    assert fibonacci(3) == 2
    assert fibonacci(19) == 4181
