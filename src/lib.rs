mod base;
mod python;
mod state;

use base::*;
use pyo3::prelude::*;
use python::*;
use state::*;

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
    classes: [Arena, BoostPadState, BoostPadStatic, GameMode, Team, WheelPairConfig, CarConfig, Car, Ball, CarControls, BallHitInfo],
    submodules: []
}

pynamedmodule! {
    doc: "",
    name: rocketsim,
    funcs: [init],
    classes: [Vec3, RotMat, GameState, GameState, CarInfo, BoostPadStatic],
    submodules: [sim]
}
