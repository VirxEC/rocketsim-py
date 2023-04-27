use pyo3::prelude::*;
use rocketsim_rs::{glam_ext::glam::Quat, CarInfo as CCarInfo, GameState as CGameState};

use crate::{
    base::RotMat,
    python::{Ball, Car, Team},
};

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: Car,
    // pub config: CarConfig,
}

impl From<CCarInfo> for CarInfo {
    #[inline]
    fn from(info: CCarInfo) -> Self {
        Self {
            id: info.id,
            team: info.team.into(),
            state: info.state.into(),
        }
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Debug)]
pub struct GameState {
    tick_rate: f32,
    tick_count: u64,
    ball: Ball,
    ball_rot: RotMat,
    cars: Vec<CarInfo>,
}

impl From<CGameState> for GameState {
    #[inline]
    fn from(state: CGameState) -> Self {
        Self {
            tick_rate: state.tick_rate,
            tick_count: state.tick_count,
            ball: state.ball.into(),
            ball_rot: Quat::from_array(state.ball_rot).into(),
            cars: state.cars.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<GameState> for CGameState {
    #[inline]
    fn from(state: GameState) -> Self {
        Self {
            tick_rate: state.tick_rate,
            tick_count: state.tick_count,
            ball: state.ball.into(),
            ball_rot: Quat::from(state.ball_rot).to_array(),
            cars: Vec::new(),
            pads: Vec::new(),
        }
    }
}
