use pyo3::prelude::*;

mod filter;

#[pymodule(name = "rustafx")]
mod py_rustafx {
    use super::*;

    #[pymodule_export]
    pub use filter::py_filter;
}
