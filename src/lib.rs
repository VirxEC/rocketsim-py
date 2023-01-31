use pyo3::prelude::*;
use rocketsim_rs::{autocxx::prelude::*, cxx::UniquePtr, sim as csim};

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

#[pyclass]
#[derive(Clone, Copy, Debug, Default)]
enum GameMode {
    #[default]
    Soccar,
}

impl From<GameMode> for csim::arena::GameMode {
    fn from(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Soccar => Self::SOCCAR,
        }
    }
}

#[pyclass(unsendable)]
#[repr(transparent)]
struct Arena(UniquePtr<csim::arena::Arena>);

#[pymethods]
impl Arena {
    #[new]
    fn __new__(gamemode: GameMode, tick_rate: f32) -> Self {
        Self(csim::arena::Arena::new(gamemode.into(), tick_rate).within_unique_ptr())
    }
}

pynamedmodule! {
    doc: "",
    name: sim,
    funcs: [],
    classes: [Arena, GameMode],
    submodules: []
}

pynamedmodule! {
    doc: "",
    name: rocketsim,
    funcs: [],
    classes: [],
    submodules: [sim]
}
