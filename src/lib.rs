extern crate cpython;

use cpython::Python;
use cpython::ObjectProtocol; //for call method
use cpython::{PyModule, PyTuple};

pub fn scatterplot(x: &[f32], y: &[f32]) {
    assert!(x.len() == y.len());

    let gil = Python::acquire_gil();
    let py = gil.python();

    let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();

    let r = plt.call(py, "scatter", (x, y), None).unwrap();
    println!("{:?}", r);

    let r = plt.call(py, "show", PyTuple::empty(py), None).unwrap();
    println!("{:?}", r);
}
