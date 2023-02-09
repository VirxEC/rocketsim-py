use pyo3::{exceptions::PyIndexError, prelude::*};
use rocketsim_rs::{autocxx::prelude::*, cxx::UniquePtr, sim as csim, Angle as CAngle, Vec3 as CVec3};
use std::fmt;

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug)]
pub enum Team {
    Blue,
    Orange,
}

impl From<Team> for csim::car::Team {
    #[inline]
    fn from(team: Team) -> Self {
        match team {
            Team::Blue => Self::BLUE,
            Team::Orange => Self::ORANGE,
        }
    }
}

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub enum GameMode {
    #[default]
    Soccar,
}

impl From<GameMode> for csim::arena::GameMode {
    #[inline]
    fn from(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Soccar => Self::SOCCAR,
        }
    }
}

#[pyclass(unsendable, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug, Default)]
pub struct Ball {
    pos: Vec3,
    vel: Vec3,
    angvel: Vec3,
}

impl From<&csim::ball::BallState> for Ball {
    #[inline]
    fn from(ball: &csim::ball::BallState) -> Self {
        Self {
            pos: ball.pos.clone().into(),
            vel: ball.vel.clone().into(),
            angvel: ball.angvel.clone().into(),
        }
    }
}

impl From<UniquePtr<csim::ball::BallState>> for Ball {
    #[inline]
    fn from(ball: UniquePtr<csim::ball::BallState>) -> Self {
        Self {
            pos: ball.pos.clone().into(),
            vel: ball.vel.clone().into(),
            angvel: ball.angvel.clone().into(),
        }
    }
}

#[pymethods]
impl Ball {
    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn get_pos(&self) -> Vec3 {
        self.pos.clone()
    }

    #[inline]
    fn get_vel(&self) -> Vec3 {
        self.vel.clone()
    }

    #[inline]
    fn get_angvel(&self) -> Vec3 {
        self.angvel.clone()
    }
}

#[pyclass(unsendable, module = "rocketsim.sim")]
#[repr(transparent)]
pub struct CarConfig(&'static csim::car::CarConfig);

impl From<&CarConfig> for &'static csim::car::CarConfig {
    #[inline]
    fn from(config: &CarConfig) -> Self {
        config.0
    }
}

#[pymethods]
impl CarConfig {
    #[staticmethod]
    fn octane() -> Self {
        Self(csim::car::CarConfig::octane())
    }

    #[staticmethod]
    fn dominus() -> Self {
        Self(csim::car::CarConfig::dominus())
    }

    #[staticmethod]
    fn plank() -> Self {
        Self(csim::car::CarConfig::plank())
    }

    #[staticmethod]
    fn breakout() -> Self {
        Self(csim::car::CarConfig::breakout())
    }

    #[staticmethod]
    fn hybrid() -> Self {
        Self(csim::car::CarConfig::hybrid())
    }

    #[staticmethod]
    fn merc() -> Self {
        Self(csim::car::CarConfig::merc())
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Angle {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl From<CAngle> for Angle {
    #[inline]
    fn from(angles: CAngle) -> Self {
        Self {
            pitch: angles.pitch,
            yaw: angles.yaw,
            roll: angles.roll,
        }
    }
}

impl From<Angle> for CAngle {
    #[inline]
    fn from(angles: Angle) -> Self {
        Self {
            pitch: angles.pitch,
            yaw: angles.yaw,
            roll: angles.roll,
        }
    }
}

#[pymethods]
impl Angle {
    #[new]
    fn __new__(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self {
            pitch,
            yaw,
            roll,
        }
    }

    #[inline]
    fn with_pitch(&self, pitch: f32) -> Self {
        Self {
            pitch,
            yaw: self.yaw,
            roll: self.roll,
        }
    }

    #[inline]
    fn with_yaw(&self, yaw: f32) -> Self {
        Self {
            pitch: self.pitch,
            yaw,
            roll: self.roll,
        }
    }

    #[inline]
    fn with_roll(&self, roll: f32) -> Self {
        Self {
            pitch: self.pitch,
            yaw: self.yaw,
            roll,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("Angle({}, {}, {})", self.pitch, self.yaw, self.roll)
    }
}

#[pyclass(unsendable, module = "rocketsim.sim")]
#[repr(transparent)]
pub struct Car(UniquePtr<csim::car::CarState>);

impl From<UniquePtr<csim::car::CarState>> for Car {
    #[inline]
    fn from(car: UniquePtr<csim::car::CarState>) -> Self {
        Self(car)
    }
}

impl<'a> From<&'a Car> for &'a csim::car::CarState {
    #[inline]
    fn from(car: &'a Car) -> Self {
        &car.0
    }
}

impl fmt::Debug for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Car")
            .field("pos", &self.0.pos)
            .field("vel", &self.0.vel)
            .field("angles", &self.0.angles)
            .field("angvel", &self.0.angvel)
            .field("boost", &self.0.boost)
            .field("is_on_ground", &self.0.isOnGround)
            .field("is_supersonic", &self.0.isSupersonic)
            .field("is_jumping", &self.0.isJumping)
            .field("has_jumped", &self.0.hasJumped)
            .field("has_double_jumped", &self.0.hasDoubleJumped)
            .field("has_flipped", &self.0.hasFlipped)
            .finish()
    }
}

#[pymethods]
impl Car {
    #[inline]
    fn get_pos(&self) -> Vec3 {
        self.0.pos.clone().into()
    }

    #[setter(pos)]
    #[inline]
    fn set_pos(&mut self, pos: Vec3) {
        self.0.pos = pos.into();
    }

    #[inline]
    fn get_vel(&self) -> Vec3 {
        self.0.vel.clone().into()
    }

    #[setter(vel)]
    #[inline]
    fn set_vel(&mut self, vel: Vec3) {
        self.0.vel = vel.into();
    }

    #[inline]
    fn get_angles(&self) -> Angle {
        self.0.angles.clone().into()
    }

    #[setter(angles)]
    #[inline]
    fn set_angles(&mut self, angles: Angle) {
        self.0.angles = angles.into();
    }

    #[inline]
    fn get_angvel(&self) -> Vec3 {
        self.0.angvel.clone().into()
    }

    #[setter(angvel)]
    #[inline]
    fn set_angvel(&mut self, angvel: Vec3) {
        self.0.angvel = angvel.into();
    }

    #[getter(boost)]
    #[inline]
    fn get_boost(&self) -> f32 {
        self.0.boost
    }

    #[setter(boost)]
    #[inline]
    fn set_boost(&mut self, boost: f32) {
        self.0.pin_mut().boost = boost;
    }

    #[getter(is_on_ground)]
    #[inline]
    fn is_on_ground(&self) -> bool {
        self.0.isOnGround
    }

    #[getter(is_supersonic)]
    #[inline]
    fn is_supersonic(&self) -> bool {
        self.0.isSupersonic
    }

    #[getter(is_jumping)]
    #[inline]
    fn is_jumping(&self) -> bool {
        self.0.isJumping
    }

    #[getter(has_jumped)]
    #[inline]
    fn has_jumped(&self) -> bool {
        self.0.hasJumped
    }

    #[getter(has_double_jumped)]
    #[inline]
    fn has_double_jumped(&self) -> bool {
        self.0.hasDoubleJumped
    }

    #[getter(has_flipped)]
    #[inline]
    fn has_flipped(&self) -> bool {
        self.0.hasFlipped
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(unsendable, module = "rocketsim.sim")]
#[repr(transparent)]
pub struct Arena(UniquePtr<csim::arena::Arena>);

#[pymethods]
impl Arena {
    #[new]
    #[inline]
    fn __new__(gamemode: GameMode, tick_rate: Option<f32>) -> Self {
        Self(csim::arena::Arena::new(gamemode.into(), tick_rate.unwrap_or(120.)).within_unique_ptr())
    }

    #[inline]
    fn get_tick_rate(&mut self) -> f32 {
        self.0.pin_mut().GetTickRate()
    }

    #[inline]
    fn step(&mut self, ticks_to_simulate: Option<u32>) {
        self.0.pin_mut().Step(c_int(ticks_to_simulate.unwrap_or(1) as i32));
    }

    #[inline]
    fn get_ball(&self) -> Ball {
        self.0.get_ball_state().into()
    }

    #[setter(ball)]
    fn set_ball(&mut self, ball: Ball) {
        let mut ball_state = self.0.get_ball_state();
        ball_state.pos = ball.pos.into();
        ball_state.vel = ball.vel.into();
        ball_state.angvel = ball.angvel.into();
        self.0.pin_mut().set_ball_state(&ball_state);
    }

    #[inline]
    fn add_car(&mut self, team: Team, config: &CarConfig) -> u32 {
        self.0.pin_mut().add_car(team.into(), config.into())
    }

    #[inline]
    fn get_car(&mut self, id: u32) -> PyResult<Car> {
        match self.0.pin_mut().get_car_state_from_id(id) {
            Ok(car) => Ok(car.into()),
            Err(e) => Err(PyIndexError::new_err(e.to_string())),
        }
    }

    #[inline]
    fn set_car(&mut self, id: u32, car: &Car) -> PyResult<()> {
        self.0.pin_mut().set_car_state(id, car.into()).map_err(|e| PyIndexError::new_err(e.to_string()))
    }
}

#[pyclass(unsendable, module = "rocketsim")]
#[repr(transparent)]
pub struct Vec3(UniquePtr<CVec3>);

impl From<&CVec3> for Vec3 {
    #[inline]
    fn from(vec3: &CVec3) -> Self {
        Self(vec3.clone())
    }
}

impl From<UniquePtr<CVec3>> for Vec3 {
    #[inline]
    fn from(vec3: UniquePtr<CVec3>) -> Self {
        Self(vec3)
    }
}

impl From<Vec3> for UniquePtr<CVec3> {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        vec3.0
    }
}

impl Clone for Vec3 {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Self(CVec3::default())
    }
}

impl fmt::Debug for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3").field("x", &self.0.x()).field("y", &self.0.y()).field("z", &self.0.z()).finish()
    }
}

impl Vec3 {
    #[inline]
    pub fn as_cvec3(&self) -> &CVec3 {
        &self.0
    }
}

#[pymethods]
impl Vec3 {
    #[new]
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(CVec3::new1(&x, &y, &z).within_unique_ptr())
    }

    #[getter(x)]
    #[inline]
    fn get_x(&self) -> f32 {
        *self.0.x()
    }

    #[inline]
    fn with_x(&mut self, x: f32) -> Self {
        Self::new(x, self.get_y(), self.get_z())
    }

    #[setter(x)]
    #[inline]
    fn set_x(&mut self, x: f32) {
        self.0.pin_mut().setX(x);
    }

    #[getter(y)]
    #[inline]
    fn get_y(&self) -> f32 {
        *self.0.y()
    }

    #[inline]
    fn with_y(&mut self, y: f32) -> Self {
        Self::new(self.get_x(), y, self.get_z())
    }

    #[setter(y)]
    #[inline]
    fn set_y2(&mut self, y: f32) {
        self.0.pin_mut().setY(y);
    }

    #[getter(z)]
    #[inline]
    fn get_z(&self) -> f32 {
        *self.0.z()
    }

    #[inline]
    fn with_z(&self, z: f32) -> Self {
        Self::new(self.get_x(), self.get_y(), z)
    }

    #[setter(z)]
    #[inline]
    fn set_z(&mut self, z: f32) {
        self.0.pin_mut().setZ(z);
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("Vec3({}, {}, {})", self.0.x(), self.0.y(), self.0.z())
    }
}
