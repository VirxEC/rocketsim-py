use pyo3::{exceptions::PyIndexError, prelude::*, types::PyTuple};
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

#[pyclass(module = "rocketsim.sim")]
pub enum CarConfig {
    Octane,
    Dominus,
    Plank,
    Breakout,
    Hybrid,
    Merc,
}

impl From<&CarConfig> for &'static csim::car::CarConfig {
    #[inline]
    fn from(config: &CarConfig) -> Self {
        match config {
            CarConfig::Octane => csim::car::CarConfig::octane(),
            CarConfig::Dominus => csim::car::CarConfig::dominus(),
            CarConfig::Plank => csim::car::CarConfig::plank(),
            CarConfig::Breakout => csim::car::CarConfig::breakout(),
            CarConfig::Hybrid => csim::car::CarConfig::hybrid(),
            CarConfig::Merc => csim::car::CarConfig::merc(),
        }
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CarControls {
    throttle: f32,
    steer: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    jump: bool,
    boost: bool,
    handbrake: bool,
}

impl From<csim::CarControls> for CarControls {
    #[inline]
    fn from(controls: csim::CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

impl From<&CarControls> for csim::CarControls {
    #[inline]
    fn from(controls: &CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

impl From<CarControls> for csim::CarControls {
    #[inline]
    fn from(controls: CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

#[pymethods]
impl CarControls {
    const NAMES: [&str; 8] = ["throttle", "steer", "pitch", "yaw", "roll", "jump", "boost", "handbrake"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len()];

        if let Ok(args) = args.get_item(0).and_then(PyAny::extract::<Vec<f32>>) {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else if let Ok(args) = args.extract::<Vec<f32>>() {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter()) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter()) {
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
                    *a = Some(x);
                }
            }
        }

        Self {
            throttle: vec[0].unwrap_or_default(),
            steer: vec[1].unwrap_or_default(),
            pitch: vec[2].unwrap_or_default(),
            yaw: vec[3].unwrap_or_default(),
            roll: vec[4].unwrap_or_default(),
            jump: vec[5].unwrap_or_default() as u8 != 0,
            boost: vec[6].unwrap_or_default() as u8 != 0,
            handbrake: vec[7].unwrap_or_default() as u8 != 0,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
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
        Self { pitch, yaw, roll }
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
            .field("last_rel_dodge_torque", &self.0.lastRelDodgeTorque)
            .field("boost", &self.0.boost)
            .field("is_on_ground", &self.0.isOnGround)
            .field("is_supersonic", &self.0.isSupersonic)
            .field("is_jumping", &self.0.isJumping)
            .field("has_jumped", &self.0.hasJumped)
            .field("has_double_jumped", &self.0.hasDoubleJumped)
            .field("has_flipped", &self.0.hasFlipped)
            .field("jump_timer", &self.0.jumpTimer)
            .field("flip_timer", &self.0.flipTimer)
            .field("air_time_space_jump", &self.0.airTimeSpaceJump)
            .field("handbrake_val", &self.0.handbrakeVal)
            .field("last_controls", &self.0.lastControls)
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

    #[inline]
    fn get_last_rel_dodge_torque(&self) -> Vec3 {
        self.0.lastRelDodgeTorque.clone().into()
    }

    #[setter(last_rel_dodge_torque)]
    #[inline]
    fn set_last_rel_dodge_torque(&mut self, last_rel_dodge_torque: Vec3) {
        self.0.lastRelDodgeTorque = last_rel_dodge_torque.into();
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

    #[setter(is_supersonic)]
    #[inline]
    fn set_is_supersonic(&mut self, is_supersonic: bool) {
        self.0.pin_mut().isSupersonic = is_supersonic;
    }

    #[getter(is_jumping)]
    #[inline]
    fn is_jumping(&self) -> bool {
        self.0.isJumping
    }

    #[setter(is_jumping)]
    #[inline]
    fn set_is_jumping(&mut self, is_jumping: bool) {
        self.0.isJumping = is_jumping;
    }

    #[getter(has_jumped)]
    #[inline]
    fn has_jumped(&self) -> bool {
        self.0.hasJumped
    }

    #[setter(has_jumped)]
    #[inline]
    fn set_has_jumped(&mut self, has_jumped: bool) {
        self.0.hasJumped = has_jumped;
    }

    #[getter(has_double_jumped)]
    #[inline]
    fn has_double_jumped(&self) -> bool {
        self.0.hasDoubleJumped
    }

    #[setter(has_double_jumped)]
    #[inline]
    fn set_has_double_jumped(&mut self, has_double_jumped: bool) {
        self.0.hasDoubleJumped = has_double_jumped;
    }

    #[getter(has_flipped)]
    #[inline]
    fn has_flipped(&self) -> bool {
        self.0.hasFlipped
    }

    #[setter(has_flipped)]
    #[inline]
    fn set_has_flipped(&mut self, has_flipped: bool) {
        self.0.hasFlipped = has_flipped;
    }

    #[getter(jump_timer)]
    #[inline]
    fn get_jump_timer(&self) -> f32 {
        self.0.jumpTimer
    }

    #[setter(jump_timer)]
    #[inline]
    fn set_jump_timer(&mut self, jump_timer: f32) {
        self.0.pin_mut().jumpTimer = jump_timer;
    }

    #[getter(flip_timer)]
    #[inline]
    fn get_flip_timer(&self) -> f32 {
        self.0.flipTimer
    }

    #[setter(flip_timer)]
    #[inline]
    fn set_flip_timer(&mut self, flip_timer: f32) {
        self.0.pin_mut().flipTimer = flip_timer;
    }

    #[getter(air_time_space_jump)]
    #[inline]
    fn get_air_time_space_jump(&self) -> f32 {
        self.0.airTimeSpaceJump
    }

    #[setter(air_time_space_jump)]
    #[inline]
    fn set_air_time_space_jump(&mut self, air_time_space_jump: f32) {
        self.0.pin_mut().airTimeSpaceJump = air_time_space_jump;
    }

    #[getter(handbrake_val)]
    #[inline]
    fn get_handbrake_val(&self) -> f32 {
        self.0.handbrakeVal
    }

    #[setter(handbrake_val)]
    #[inline]
    fn set_handbrake_val(&mut self, handbrake_val: f32) {
        self.0.pin_mut().handbrakeVal = handbrake_val;
    }

    #[getter(last_controls)]
    #[inline]
    fn get_last_controls(&self) -> CarControls {
        self.0.lastControls.into()
    }

    #[setter(last_controls)]
    #[inline]
    fn set_last_controls(&mut self, last_controls: CarControls) {
        self.0.lastControls = last_controls.into();
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
    fn set_car_controls(&mut self, id: u32, controls: &CarControls) -> PyResult<()> {
        self.0.pin_mut().set_car_controls(id, &controls.into()).map_err(|e| PyIndexError::new_err(e.to_string()))
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
