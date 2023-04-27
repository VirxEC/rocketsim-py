use pyo3::prelude::*;
use rocketsim_rs::{
    glam_ext::glam::{Mat3, Mat3A, Quat, Vec3 as GVec3},
    math::{Angle, RotMat as CRotMat, Vec3 as CVec3},
};

#[pyfunction]
#[inline]
pub fn init(collision_meshes_folder: Option<&str>) {
    rocketsim_rs::init(collision_meshes_folder);
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RotMat {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl From<Quat> for RotMat {
    #[inline]
    fn from(quat: Quat) -> Self {
        Mat3::from_quat(quat).into()
    }
}

impl From<Mat3> for RotMat {
    #[inline]
    fn from(mat: Mat3) -> Self {
        Self {
            forward: mat.x_axis.into(),
            right: mat.y_axis.into(),
            up: mat.z_axis.into(),
        }
    }
}

impl From<CRotMat> for RotMat {
    #[inline]
    fn from(rot_mat: CRotMat) -> Self {
        Self {
            forward: rot_mat.forward.into(),
            right: rot_mat.right.into(),
            up: rot_mat.up.into(),
        }
    }
}

impl From<RotMat> for CRotMat {
    #[inline]
    fn from(rot_mat: RotMat) -> Self {
        Self {
            forward: rot_mat.forward.into(),
            right: rot_mat.right.into(),
            up: rot_mat.up.into(),
        }
    }
}

#[pymethods]
impl RotMat {
    #[new]
    #[inline]
    fn __new__(forward: Vec3, right: Vec3, up: Vec3) -> Self {
        Self { forward, right, up }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("RotMat({}, {}, {})", self.forward.__repr__(), self.right.__repr__(), self.up.__repr__())
    }

    #[inline]
    #[staticmethod]
    fn from_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        CRotMat::from(Mat3A::from_quat(Quat::from(Angle { pitch, yaw, roll }))).into()
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

#[pymethods]
impl Vec3 {
    #[new]
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn with_x(&mut self, x: f32) -> Self {
        Self::new(x, self.y, self.z)
    }

    #[inline]
    fn with_y(&mut self, y: f32) -> Self {
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
    fn __repr__(&self) -> String {
        format!("Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}
