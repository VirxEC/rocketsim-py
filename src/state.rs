use pyo3::prelude::*;
use rocketsim_rs::{glam_ext::glam::Quat, CarInfo as CCarInfo, GameState as CGameState};

use crate::{
    base::{FromGil, RemoveGil, RotMat},
    new_gil,
    python::{Ball, Car, Team},
};

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: Py<Car>,
    // pub config: CarConfig,
}

impl FromGil<CCarInfo> for CarInfo {
    #[inline]
    fn from_gil(py: Python, info: CCarInfo) -> PyResult<Self> {
        Ok(Self {
            id: info.id,
            team: info.team.into(),
            state: new_gil!(Car, py, info.state),
        })
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct GameState {
    tick_rate: f32,
    tick_count: u64,
    ball: Py<Ball>,
    ball_rot: Py<RotMat>,
    cars: Vec<Py<CarInfo>>,
}

impl FromGil<CGameState> for GameState {
    #[inline]
    fn from_gil(py: Python, state: CGameState) -> PyResult<Self> {
        Ok(Self {
            tick_rate: state.tick_rate,
            tick_count: state.tick_count,
            ball: new_gil!(Ball, py, state.ball),
            ball_rot: new_gil!(RotMat, py, Quat::from_array(state.ball_rot)),
            cars: state.cars.into_iter().flat_map(|car| CarInfo::from_gil(py, car)).flat_map(|car| Py::new(py, car)).collect(),
        })
    }
}

impl RemoveGil<CGameState> for GameState {
    #[inline]
    fn remove_gil(self, py: Python) -> CGameState {
        CGameState {
            tick_rate: self.tick_rate,
            tick_count: self.tick_count,
            ball: self.ball.borrow(py).clone().remove_gil(py),
            ball_rot: RemoveGil::<Quat>::remove_gil(self.ball_rot.borrow(py).clone(), py).to_array(),
            cars: Vec::new(),
            pads: Vec::new(),
        }
    }
}
