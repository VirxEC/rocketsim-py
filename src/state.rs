use pyo3::{prelude::*, types::PyTuple};
use rocketsim_rs::{glam_ext::glam::Quat, CarInfo as CCarInfo, GameState as CGameState};

use crate::{
    base::{FromGil, PyDefault, RemoveGil, RotMat},
    new_gil, new_gil_default,
    python::{Ball, Car, CarConfig, Team},
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
    #[pyo3(signature = (*args, **kwargs))]
    fn __new__(py: Python, args: &PyTuple, kwargs: Option<&PyAny>) -> PyResult<Self> {
        let mut id = None;
        let mut team = None;
        let mut state = None;
        let mut config = None;

        if !args.is_empty() {
            if let Ok(val) = args.get_item(0).and_then(PyAny::extract) {
                id = Some(val);

                if let Ok(val) = args.get_item(1).and_then(PyAny::extract) {
                    team = Some(val);

                    if let Ok(val) = args.get_item(2).and_then(PyAny::extract) {
                        state = Some(val);

                        if let Ok(val) = args.get_item(3).and_then(PyAny::extract) {
                            config = Some(val);
                        }
                    }
                }
            }
        }

        if let Some(kwargs) = kwargs {
            if let Ok(val) = kwargs.get_item("id").and_then(PyAny::extract) {
                id = Some(val);
            }

            if let Ok(val) = kwargs.get_item("team").and_then(PyAny::extract) {
                team = Some(val);
            }

            if let Ok(val) = kwargs.get_item("state").and_then(PyAny::extract) {
                state = Some(val);
            }

            if let Ok(val) = kwargs.get_item("config").and_then(PyAny::extract) {
                config = Some(val);
            }
        }

        Ok(Self {
            id: id.unwrap_or(1),
            team: team.unwrap_or_default(),
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
            "CarInfo(id={}, team={:?}, state={}, config={})",
            self.id,
            self.team,
            self.state.borrow(py).__str__(),
            self.config.borrow(py).__repr__(py)
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
            cars: self.cars.into_iter().map(|car| car.borrow(py).remove_gil(py)).collect(),
            pads: Vec::new(),
        }
    }
}

#[pymethods]
impl GameState {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn __new__(py: Python, args: &PyTuple, kwargs: Option<&PyAny>) -> PyResult<Self> {
        let mut tick_count = None;
        let mut tick_rate = None;
        let mut ball = None;
        let mut ball_rot = None;
        let mut cars = None;

        if !args.is_empty() {
            if let Ok(val) = args.get_item(0).and_then(PyAny::extract) {
                tick_count = Some(val);

                if let Ok(val) = args.get_item(1).and_then(PyAny::extract) {
                    tick_rate = Some(val);

                    if let Ok(val) = args.get_item(2).and_then(PyAny::extract) {
                        ball = Some(val);

                        if let Ok(val) = args.get_item(3).and_then(PyAny::extract) {
                            ball_rot = Some(val);

                            if let Ok(val) = args.get_item(4).and_then(PyAny::extract) {
                                cars = Some(val);
                            }
                        }
                    }
                }
            }
        }

        if let Some(kwargs) = kwargs {
            if let Ok(val) = kwargs.get_item("tick_count").and_then(PyAny::extract) {
                tick_count = Some(val);
            }

            if let Ok(val) = kwargs.get_item("tick_rate").and_then(PyAny::extract) {
                tick_rate = Some(val);
            }

            if let Ok(val) = kwargs.get_item("ball").and_then(PyAny::extract) {
                ball = Some(val);
            }

            if let Ok(val) = kwargs.get_item("ball_rot").and_then(PyAny::extract) {
                ball_rot = Some(val);
            }

            if let Ok(val) = kwargs.get_item("cars").and_then(PyAny::extract) {
                cars = Some(val);
            }
        }

        Ok(Self {
            tick_rate: tick_rate.unwrap_or_default(),
            tick_count: tick_count.unwrap_or_default(),
            ball: ball.unwrap_or(new_gil_default!(Ball, py)),
            ball_rot: ball_rot.unwrap_or(new_gil_default!(RotMat, py)),
            cars: cars.unwrap_or_default(),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "GameState(tick_count={}, tick_rate={}, ball={}, ball_rot={}, cars=[{}])",
            self.tick_count,
            self.tick_rate,
            self.ball.borrow(py).__repr__(py),
            self.ball_rot.borrow(py).__repr__(py),
            self.cars.iter().map(|car| car.borrow(py).__repr__(py)).collect::<Vec<_>>().join(", "),
        )
    }
}
