use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn new_list(value: i32, len: usize) -> PyResult<Vec<i32>> {
    let v = vec![value; len];
    Ok(v)
}

#[pyfunction]
fn fibonacci(n: usize) -> PyResult<usize> {
    Ok(match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2)? + fibonacci(n - 1)?,
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pyo3_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(new_list, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0).unwrap(), 0);
        assert_eq!(fibonacci(1).unwrap(), 1);
        assert_eq!(fibonacci(2).unwrap(), 1);
        assert_eq!(fibonacci(3).unwrap(), 2);
        assert_eq!(fibonacci(19).unwrap(), 4181);
    }
}
