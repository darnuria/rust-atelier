extern crate csv;
extern crate ndarray;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use ndarray::{arr2, Array2};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use ndarray_csv::Array2Reader;

/// Temporary high lvl doc:
/// 
/// Show a minimal exemple for reusing some rust part in order to expose
/// it to python.
/// 
/// Construct two dimentionnal matrices from csv.
/// Then expose the dot product.


// TODO: Change signature for handling error instead of panic.
fn parse_arr2(path: impl AsRef<Path>) -> Array2<i32> {
    let file = match File::open(path.as_ref()) {
        Err(why) => panic!("I had some issues with opening {}", path.as_ref().display()),
        Ok(file) => file,
    };

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let array: Array2<i32> = reader
        .deserialize_array2((16, 16))
        .expect("Whoups still panicking!");

    array
}

/// Wrapping «manually» some function of a Rust crate for python.
#[pyclass(module = "pytrix")]
struct PyTrix {
    handle: Array2<i32>,
}

#[pymethods]
impl PyTrix {
    #[new]
    fn new(obj: &PyRawObject, csv_filename: String) {
        obj.init(PyTrix {
            handle: parse_arr2(csv_filename),
        })
    }

    fn mul(&self, py: Python<'_>, other: &PyTrix) -> PyResult<PyTrix> {
        let product = self.handle.dot(&other.handle);
        let pythonized = PyTrix { handle: product };
        Ok(pythonized)
    }
}

#[pymodule]
fn pytrix(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTrix>()?;
    Ok(())
}
