/// This file contains the support for loading the bot as a library in python to get around
/// replit's limitations
use cpython::{py_fn, PyNone, PyResult, Python};

cpython::py_module_initializer!(libyarbot, |py, m| {
    m.add(py, "__doc__", "Run the bot with the start function")?;
    #[allow(clippy::manual_strip)]
    m.add(py, "start", py_fn!(py, main_py()))?;
    Ok(())
});

#[allow(clippy::unnecessary_wraps)] //Required by the py_fn macro
pub fn main_py(_: Python) -> PyResult<PyNone> {
    #[allow(clippy::main_recursion)] //Fine because this function will be executed from python
    crate::main();
    Ok(PyNone)
}
