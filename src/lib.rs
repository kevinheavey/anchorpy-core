use std::collections::HashMap;

use idl::create_idl_mod;
use pyo3::prelude::*;

pub mod idl;

#[pymodule]
fn anchorpy_core(py: Python, m: &PyModule) -> PyResult<()> {
    let idl_mod = create_idl_mod(py)?;
    let submodules = [idl_mod];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("anchorpy_core.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        m.add_submodule(submod)?;
    }
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
