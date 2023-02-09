mod python;

use pyo3::prelude::*;
use python::*;

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*], submodules: [$($submodule_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        #[allow(unused_variables)]
        #[allow(redundant_semicolons)]
        fn $name(py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(pyo3::wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            $(m.add_wrapped(pyo3::wrap_pymodule!($submodule_name))?);*;
            Ok(())
        }
    };
}

pynamedmodule! {
    doc: "",
    name: sim,
    funcs: [],
    classes: [Arena, GameMode, Team, CarConfig, Car, Ball, CarControls],
    submodules: []
}

pynamedmodule! {
    doc: "",
    name: rocketsim,
    funcs: [],
    classes: [Vec3, Angle],
    submodules: [sim]
}
