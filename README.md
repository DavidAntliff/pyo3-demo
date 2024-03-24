# PyO3 Demonstration

A simple Python project that makes use of a Rust library, via [PyO3](https://github.com/PyO3/pyo3).

Requires [maturin](https://github.com/PyO3/maturin).

## Known Issues

Complications around `cargo test`, described in `Cargo.toml`. May be a symptom of
using [pyenv](https://github.com/pyenv/pyenv)?

## Building

This repository uses pyenv to manage Python 3.12.2:

```
$ env PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install 3.12.2 --keep
```

The "kept" sources are retained in `$HOME/git/github/pyenv/pyenv.git/versions/3.12.2/lib` and this
is (unfortunately) reflected in `.cargo/config.toml` (for `cargo test` workaround).

Create a virtualenv using the new Python version:

```
$ pyenv local 3.12.2
$ python -m venv venv
$ . venv/bin/activate
```

Install maturin and build the Rust library:

```
$ pip install maturin
```

```
$ (cd rust_lib && maturin develop)
```

This creates the Python package and automatically installs it into the venv.

Now run the Python demo program:

```
$ python pyo3_demo/pyo3_demo.py
```

## Testing

To run the Rust library's Python tests:

```
$ pytest -sv pyo3_demo/
```

To run the Rust library tests:

```
$ (cd rust_lib && cargo test)
```
