use pyo3::prelude::*;
use rocketsim_rs::{glam_ext::glam::Quat, BoostPad as CBoostPad, CarInfo as CCarInfo, GameState as CGameState};

use crate::{
    base::{repr_bool, FromGil, PyDefault, RemoveGil, RotMat, Vec3},
    new_gil, new_gil_default,
    python::{Ball, BoostPadState, Car, CarConfig, Team},
};

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: Py<Car>,
    pub config: Py<CarConfig>,
}

impl FromGil<CCarInfo> for CarInfo {
    #[inline]
    fn from_gil(py: Python, info: CCarInfo) -> PyResult<Self> {
        Ok(Self {
            id: info.id,
            team: info.team.into(),
            state: new_gil!(Car, py, info.state),
            config: new_gil!(CarConfig, py, info.config),
        })
    }
}

impl RemoveGil<CCarInfo> for &CarInfo {
    #[inline]
    fn remove_gil(self, py: Python) -> CCarInfo {
        CCarInfo {
            id: self.id,
            team: self.team.into(),
            state: self.state.borrow(py).clone().remove_gil(py),
            config: self.config.borrow(py).clone().remove_gil(py),
        }
    }
}

#[pymethods]
impl CarInfo {
    #[new]
    #[inline]
    #[pyo3(signature = (id, team=Team::Blue, state=None, config=None))]
    fn __new__(py: Python, id: u32, team: Team, state: Option<Py<Car>>, config: Option<Py<CarConfig>>) -> PyResult<Self> {
        Ok(Self {
            id,
            team,
            state: state.unwrap_or(new_gil_default!(Car, py)),
            config: config.unwrap_or(new_gil_default!(CarConfig, py)),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "CarInfo(id={}, team={}, state={}, config={})",
            self.id,
            self.team.__repr__(),
            self.state.borrow(py).__repr__(py),
            self.config.borrow(py).__repr__(py)
        )
    }
}

#[pyclass(get_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Py<Vec3>,
    #[pyo3(set)]
    pub state: Py<BoostPadState>,
}

impl FromGil<CBoostPad> for BoostPad {
    #[inline]
    fn from_gil(py: Python, obj: CBoostPad) -> PyResult<Self> {
        Ok(BoostPad {
            is_big: obj.is_big,
            position: new_gil!(Vec3, py, obj.position),
            state: new_gil!(BoostPadState, py, obj.state),
        })
    }
}

impl RemoveGil<CBoostPad> for BoostPad {
    #[inline]
    fn remove_gil(self, py: Python) -> CBoostPad {
        CBoostPad {
            is_big: self.is_big,
            position: self.position.remove_gil(py),
            state: self.state.remove_gil(py),
        }
    }
}

#[pymethods]
impl BoostPad {
    #[new]
    #[inline]
    #[pyo3(signature = (is_big=false, position=None, state=None))]
    fn __new__(py: Python, is_big: bool, position: Option<Py<Vec3>>, state: Option<Py<BoostPadState>>) -> PyResult<Self> {
        Ok(Self {
            is_big,
            position: position.unwrap_or(new_gil_default!(Vec3, py)),
            state: state.unwrap_or(new_gil_default!(BoostPadState, py)),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "BoostPad(is_big={}, position={}, state={})",
            repr_bool(self.is_big),
            self.position.borrow(py).__repr__(),
            self.state.borrow(py).__repr__()
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct GameState {
    pub tick_rate: f32,
    pub tick_count: u64,
    pub ball: Py<Ball>,
    pub ball_rot: Py<RotMat>,
    pub cars: Vec<Py<CarInfo>>,
    pub pads: Vec<Py<BoostPad>>,
}

impl RemoveGil<CGameState> for GameState {
    #[inline]
    fn remove_gil(self, py: Python) -> CGameState {
        CGameState {
            tick_rate: self.tick_rate,
            tick_count: self.tick_count,
            ball: self.ball.borrow(py).clone().remove_gil(py),
            ball_rot: RemoveGil::<Quat>::remove_gil(self.ball_rot.borrow(py).clone(), py).to_array(),
            cars: self.cars.into_iter().map(|car| car.borrow(py).remove_gil(py)).collect(),
            pads: Vec::new(),
        }
    }
}

#[pymethods]
impl GameState {
    #[new]
    #[inline]
    #[pyo3(signature = (tick_count=0, tick_rate=120., ball=None, ball_rot=None, cars=Vec::new(), pads=Vec::new()))]
    fn __new__(
        py: Python,
        tick_count: u64,
        tick_rate: f32,
        ball: Option<Py<Ball>>,
        ball_rot: Option<Py<RotMat>>,
        cars: Vec<Py<CarInfo>>,
        pads: Vec<Py<BoostPad>>,
    ) -> PyResult<Self> {
        Ok(Self {
            tick_rate,
            tick_count,
            ball: ball.unwrap_or(new_gil_default!(Ball, py)),
            ball_rot: ball_rot.unwrap_or(new_gil_default!(RotMat, py)),
            cars,
            pads,
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "GameState(tick_count={}, tick_rate={}, ball={}, ball_rot={}, cars=[{}], pads=[{}])",
            self.tick_count,
            self.tick_rate,
            self.ball.borrow(py).__repr__(py),
            self.ball_rot.borrow(py).__repr__(py),
            self.cars.iter().map(|car| car.borrow(py).__repr__(py)).collect::<Vec<_>>().join(", "),
            self.pads.iter().map(|pad| pad.borrow(py).__repr__(py)).collect::<Vec<_>>().join(", ")
        )
    }
}
