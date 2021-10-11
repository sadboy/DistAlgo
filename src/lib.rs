use core::ffi::c_void;
use core::mem::size_of_val;
use pyo3::types::PyString;
use std::convert::TryInto;
use std::fmt::Write;
use pyo3::prelude::*;
use pyo3::ffi;

use rustpython_parser::parser;

#[pyfunction]
fn hello(name: String) -> PyResult<String> {
    let mut buf = String::new();
    buf.write_fmt(format_args!("Bah, {}!", name)).unwrap();
    let res = helper(buf.len().try_into().unwrap()).unwrap();
    buf.write_fmt(format_args!("{}", res)).unwrap();
    Ok(buf)
}

fn helper(val: i64) -> PyResult<usize> {
    Python::with_gil(|py| -> PyResult<()> {
        let hello: &PyString = py.eval("\"Hello World!\"", None, None)?.extract()?;
        println!("Bo says: {}", hello);
        Ok(())
    })?;

    let suite = parser::parse_program("x = a + 1").unwrap();
    // let stmt = suite.get(0);
    Ok(size_of_val(&suite))
}

unsafe extern "C" fn noop_dealloc(op: *mut ffi::PyObject) {
    ffi::PyObject_GC_UnTrack(op as *mut c_void);
}

#[pymodule]
fn _da(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    unsafe {
        ffi::PyDict_Type.tp_dealloc = Some(noop_dealloc);
        ffi::PyTuple_Type.tp_dealloc = Some(noop_dealloc);
        ffi::PyList_Type.tp_dealloc = Some(noop_dealloc);
    }

    Ok(())
}
