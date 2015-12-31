extern crate cpython;

use cpython::Python;
use cpython::ObjectProtocol; //for call method
use cpython::{PyModule, PyTuple};

pub struct Env {
    gil: cpython::GILGuard,
}

impl Env {
    pub fn new() -> Env {
        Env { gil: Python::acquire_gil() }
    }
}

pub struct Plot<'p> {
    py: Python<'p>,
    plt: PyModule,
}

impl<'p> Plot<'p> {
    pub fn new<'a>(env: &'a Env) -> Plot<'a> {
        let py = env.gil.python();
        let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();

        Plot { py: py, plt: plt }
    }

    pub fn interactive(&self) {
        let _ = self.plt.call(self.py, "ion", PyTuple::empty(self.py), None).unwrap();
    }

    pub fn show(&self) {
        let _ = self.plt.call(self.py, "show", PyTuple::empty(self.py), None).unwrap();
    }

    pub fn scatter(&self, x: &[f32], y: &[f32]) {
        assert!(x.len() == y.len());
        let _ = self.plt.call(self.py, "scatter", (x, y), None).unwrap();
    }

    pub fn title(&self, title: &str) {
        let _ = self.plt.call(self.py, "title", (title,), None).unwrap();
    }

    pub fn draw(&self) {
        let _ = self.plt.call(self.py, "draw", PyTuple::empty(self.py), None).unwrap();
    }

    /// Clear figure
    pub fn clf(&self) {
        let _ = self.plt.call(self.py, "clf", PyTuple::empty(self.py), None).unwrap();
    }
}

/// Convenience function for one-shot plots.
pub fn scatterplot(x: &[f32], y: &[f32]) {
    let env = Env::new();
    let plot = Plot::new(&env);

    // draw the actual data
    plot.scatter(x, y);

    // display window
    plot.show();
}
