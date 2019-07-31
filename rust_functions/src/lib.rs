#[macro_use] extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(libpyrust, initlibpyrust, PyInit_libpyrust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "square", py_fn!(py, square(val: f64)))?;
    m.add(py, "calc_pi", py_fn!(py, calc_pi(val: u32)))?;
    Ok(())
});

fn square(_: Python, val: f64) -> PyResult<f64> {
    Ok(val.powf(2.0))
}

fn calc_pi(_: Python, val: u32) -> PyResult<f64> {
    let radius: u32 = val;
    let area: f64 = (radius as f64).powf(2.0);
    let mut sum: f64 = 0.0;

    for i in 0..radius {
        for j in 0..radius {
            if ((i as f64).powf(2.0) + (j as f64).powf(2.0)).powf(0.5) <= (radius as f64) {
                sum += 1.0;
            }
        }
    }

    let pi: f64 = 4.0 * sum / area;
    Ok(pi)
}
