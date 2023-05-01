use pyo3::{prelude::*, types::PyTuple, PyClass};
use rocketsim_rs::{
    glam_ext::glam::{Mat3, Mat3A, Quat, Vec3 as GVec3},
    math::{Angle, RotMat as CRotMat, Vec3 as CVec3},
};

pub trait PyDefault
where
    Self: Sized,
{
    fn py_default(py: Python) -> PyResult<Self>;
}

impl<T> PyDefault for T
where
    T: Default,
{
    fn py_default(_py: Python) -> PyResult<Self> {
        Ok(Self::default())
    }
}

pub trait FromGil<T>: Sized {
    fn from_gil(py: Python, obj: T) -> PyResult<Self>;
}

pub trait RemoveGil<T> {
    fn remove_gil(self, py: Python) -> T;
}

impl<T, U> RemoveGil<T> for Py<U>
where
    U: PyClass + Copy,
    T: From<U>,
{
    #[inline]
    fn remove_gil(self, py: Python) -> T {
        T::from(*self.borrow(py))
    }
}

#[macro_export]
macro_rules! new_gil {
    ($t:ty, $py:expr, $e:expr) => {
        Py::new($py, <$t>::from_gil($py, $e)?)?
    };
}

#[macro_export]
macro_rules! new_gil_default {
    ($t:ty, $py:expr) => {
        Py::new($py, <$t>::from_gil($py, <$t>::py_default($py)?)?)?
    };
}

pub trait IntoGil<T>: Sized {
    fn into_gil(self, py: Python) -> PyResult<T>;
}

impl<T, U> FromGil<U> for T
where
    T: From<U>,
{
    #[inline]
    fn from_gil(_py: Python, obj: U) -> PyResult<Self> {
        Ok(Self::from(obj))
    }
}

impl<T, U> IntoGil<U> for T
where
    U: FromGil<T>,
{
    #[inline]
    fn into_gil(self, py: Python) -> PyResult<U> {
        U::from_gil(py, self)
    }
}

#[pyfunction]
#[inline]
pub fn init(collision_meshes_folder: Option<&str>) {
    rocketsim_rs::init(collision_meshes_folder);
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct RotMat {
    pub forward: Py<Vec3>,
    pub right: Py<Vec3>,
    pub up: Py<Vec3>,
}

impl FromGil<CRotMat> for RotMat {
    #[inline]
    fn from_gil(py: Python, mat: CRotMat) -> PyResult<Self> {
        Ok(Self {
            forward: new_gil!(Vec3, py, mat.forward),
            right: new_gil!(Vec3, py, mat.right),
            up: new_gil!(Vec3, py, mat.up),
        })
    }
}

impl RemoveGil<CRotMat> for RotMat {
    #[inline]
    fn remove_gil(self, py: Python) -> CRotMat {
        CRotMat {
            forward: self.forward.remove_gil(py),
            right: self.right.remove_gil(py),
            up: self.up.remove_gil(py),
        }
    }
}

impl FromGil<Mat3> for RotMat {
    #[inline]
    fn from_gil(py: Python, mat: Mat3) -> PyResult<Self> {
        Ok(Self {
            forward: new_gil!(Vec3, py, mat.x_axis),
            right: new_gil!(Vec3, py, mat.y_axis),
            up: new_gil!(Vec3, py, mat.z_axis),
        })
    }
}

impl RemoveGil<Mat3> for RotMat {
    #[inline]
    fn remove_gil(self, py: Python) -> Mat3 {
        Mat3::from_cols(self.forward.remove_gil(py), self.right.remove_gil(py), self.up.remove_gil(py))
    }
}

impl FromGil<Quat> for RotMat {
    #[inline]
    fn from_gil(py: Python, quat: Quat) -> PyResult<Self> {
        Self::from_gil(py, Mat3::from_quat(quat))
    }
}

impl RemoveGil<Quat> for RotMat {
    #[inline]
    fn remove_gil(self, py: Python) -> Quat {
        Quat::from_mat3(&self.remove_gil(py))
    }
}

#[pymethods]
impl RotMat {
    #[new]
    #[inline]
    fn __new__(py: Python, forward: Vec3, right: Vec3, up: Vec3) -> PyResult<Self> {
        Ok(Self {
            forward: new_gil!(Vec3, py, forward),
            right: new_gil!(Vec3, py, right),
            up: new_gil!(Vec3, py, up),
        })
    }

    #[inline]
    #[staticmethod]
    pub fn identity(py: Python) -> PyResult<Self> {
        Ok(Self {
            forward: new_gil!(Vec3, py, Vec3::X),
            right: new_gil!(Vec3, py, Vec3::Y),
            up: new_gil!(Vec3, py, Vec3::Z),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "RotMat({}, {}, {})",
            self.forward.borrow(py).__repr__(),
            self.right.borrow(py).__repr__(),
            self.up.borrow(py).__repr__()
        )
    }

    #[inline]
    #[staticmethod]
    fn from_angles(py: Python, pitch: f32, yaw: f32, roll: f32) -> PyResult<Self> {
        CRotMat::from(Mat3A::from_quat(Quat::from(Angle { pitch, yaw, roll }))).into_gil(py)
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<GVec3> for Vec3 {
    #[inline]
    fn from(vec3: GVec3) -> Self {
        Self { x: vec3.x, y: vec3.y, z: vec3.z }
    }
}

impl From<Vec3> for GVec3 {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        Self { x: vec3.x, y: vec3.y, z: vec3.z }
    }
}

impl From<CVec3> for Vec3 {
    #[inline]
    fn from(vec3: CVec3) -> Self {
        Self { x: vec3.x, y: vec3.y, z: vec3.z }
    }
}

impl From<Vec3> for CVec3 {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        CVec3 {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            _w: 0.,
        }
    }
}

impl Vec3 {
    const NAMES: [&'static str; 3] = ["x", "y", "z"];

    pub const ZERO: Self = Self { x: 0., y: 0., z: 0. };
    pub const X: Self = Self { x: 1., y: 0., z: 0. };
    pub const Y: Self = Self { x: 0., y: 1., z: 0. };
    pub const Z: Self = Self { x: 0., y: 0., z: 1. };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[pymethods]
impl Vec3 {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn __new__(args: &PyTuple, kwargs: Option<&PyAny>) -> PyResult<Self> {
        let mut vals = [None; Self::NAMES.len()];

        for (arg, val) in args.iter().zip(vals.iter_mut()) {
            if let Ok(item) = arg.extract() {
                *val = Some(item);
            }
        }

        if let Some(kwargs) = kwargs {
            for (name, val) in Self::NAMES.iter().zip(vals.iter_mut()) {
                if let Ok(item) = kwargs.get_item(name).and_then(PyAny::extract) {
                    *val = Some(item);
                }
            }
        }

        Ok(Self {
            x: vals[0].unwrap_or_default(),
            y: vals[1].unwrap_or_default(),
            z: vals[2].unwrap_or_default(),
        })
    }

    #[inline]
    fn with_x(&self, x: f32) -> Self {
        Self::new(x, self.y, self.z)
    }

    #[inline]
    fn with_y(&self, y: f32) -> Self {
        Self::new(self.x, y, self.z)
    }

    #[inline]
    fn with_z(&self, z: f32) -> Self {
        Self::new(self.x, self.y, z)
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self) -> String {
        format!("Vec3(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}
