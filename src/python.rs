use pyo3::{exceptions::PyIndexError, prelude::*};
use rocketsim_rs::{autocxx::prelude::*, cxx::UniquePtr, glam_ext::glam::Quat, sim as csim};

use crate::{
    base::{repr_bool, FromGil, IntoGil, PyDefault, RemoveGil, RotMat, Vec3},
    new_gil, new_gil_default,
    state::{BoostPad, CarInfo, GameState},
};

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Team {
    #[default]
    Blue,
    Orange,
}

impl From<Team> for csim::Team {
    #[inline]
    fn from(team: Team) -> Self {
        match team {
            Team::Blue => Self::BLUE,
            Team::Orange => Self::ORANGE,
        }
    }
}

impl From<csim::Team> for Team {
    #[inline]
    fn from(team: csim::Team) -> Self {
        match team {
            csim::Team::BLUE => Self::Blue,
            csim::Team::ORANGE => Self::Orange,
        }
    }
}

#[pymethods]
impl Team {
    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self) -> String {
        format!("Team.{self:?}")
    }
}

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub enum GameMode {
    #[default]
    Soccar,
    TheVoid,
}

impl From<GameMode> for csim::GameMode {
    #[inline]
    fn from(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Soccar => Self::SOCCAR,
            GameMode::TheVoid => Self::THE_VOID,
        }
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct BallHitInfo {
    is_valid: bool,
    relative_pos_on_ball: Py<Vec3>,
    ball_pos: Py<Vec3>,
    extra_hit_vel: Py<Vec3>,
    tick_count_when_hit: u64,
    tick_count_when_extra_impulse_applied: u64,
}

impl PyDefault for BallHitInfo {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            is_valid: false,
            relative_pos_on_ball: new_gil_default!(Vec3, py),
            ball_pos: new_gil_default!(Vec3, py),
            extra_hit_vel: new_gil_default!(Vec3, py),
            tick_count_when_hit: 0,
            tick_count_when_extra_impulse_applied: 0,
        })
    }
}

impl FromGil<csim::BallHitInfo> for BallHitInfo {
    #[inline]
    fn from_gil(py: Python, hit: csim::BallHitInfo) -> PyResult<Self> {
        Ok(Self {
            is_valid: hit.is_valid,
            relative_pos_on_ball: new_gil!(Vec3, py, hit.relative_pos_on_ball),
            ball_pos: new_gil!(Vec3, py, hit.ball_pos),
            extra_hit_vel: new_gil!(Vec3, py, hit.extra_hit_vel),
            tick_count_when_hit: hit.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: hit.tick_count_when_extra_impulse_applied,
        })
    }
}

impl RemoveGil<csim::BallHitInfo> for &BallHitInfo {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::BallHitInfo {
        csim::BallHitInfo {
            is_valid: self.is_valid,
            relative_pos_on_ball: self.relative_pos_on_ball.clone().remove_gil(py),
            ball_pos: self.ball_pos.clone().remove_gil(py),
            extra_hit_vel: self.extra_hit_vel.clone().remove_gil(py),
            tick_count_when_hit: self.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: self.tick_count_when_extra_impulse_applied,
        }
    }
}

#[pymethods]
impl BallHitInfo {
    #[new]
    #[inline]
    #[pyo3(signature = (is_valid=false, relative_pos_on_ball=None, ball_pos=None, extra_hit_vel=None, tick_count_when_hit=0, tick_count_when_extra_impulse_applied=0))]
    fn __new__(
        py: Python,
        is_valid: bool,
        relative_pos_on_ball: Option<Py<Vec3>>,
        ball_pos: Option<Py<Vec3>>,
        extra_hit_vel: Option<Py<Vec3>>,
        tick_count_when_hit: u64,
        tick_count_when_extra_impulse_applied: u64,
    ) -> PyResult<Self> {
        Ok(Self {
            is_valid,
            relative_pos_on_ball: relative_pos_on_ball.unwrap_or(new_gil_default!(Vec3, py)),
            ball_pos: ball_pos.unwrap_or(new_gil_default!(Vec3, py)),
            extra_hit_vel: extra_hit_vel.unwrap_or(new_gil_default!(Vec3, py)),
            tick_count_when_hit,
            tick_count_when_extra_impulse_applied,
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "BallHitInfo(is_valid={}, relative_pos_on_ball={}, ball_pos={}, extra_hit_vel={}, tick_count_when_hit={}, tick_count_when_extra_impulse_applied={})",
            repr_bool(self.is_valid),
            self.relative_pos_on_ball.borrow(py).__repr__(),
            self.ball_pos.borrow(py).__repr__(),
            self.extra_hit_vel.borrow(py).__repr__(),
            self.tick_count_when_hit,
            self.tick_count_when_extra_impulse_applied
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct Ball {
    pos: Py<Vec3>,
    vel: Py<Vec3>,
    ang_vel: Py<Vec3>,
}

impl PyDefault for Ball {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            pos: new_gil_default!(Vec3, py),
            vel: new_gil_default!(Vec3, py),
            ang_vel: new_gil_default!(Vec3, py),
        })
    }
}

impl FromGil<csim::BallState> for Ball {
    #[inline]
    fn from_gil(py: Python, ball: csim::BallState) -> PyResult<Self> {
        Ok(Self {
            pos: new_gil!(Vec3, py, ball.pos),
            vel: new_gil!(Vec3, py, ball.vel),
            ang_vel: new_gil!(Vec3, py, ball.ang_vel),
        })
    }
}

impl RemoveGil<csim::BallState> for Ball {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::BallState {
        csim::BallState {
            pos: self.pos.remove_gil(py),
            vel: self.vel.remove_gil(py),
            ang_vel: self.ang_vel.remove_gil(py),
        }
    }
}

impl RemoveGil<csim::BallState> for &Ball {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::BallState {
        csim::BallState {
            pos: self.pos.clone().remove_gil(py),
            vel: self.vel.clone().remove_gil(py),
            ang_vel: self.ang_vel.clone().remove_gil(py),
        }
    }
}

#[pymethods]
impl Ball {
    #[inline]
    #[new]
    fn __new__(py: Python, pos: Option<Py<Vec3>>, vel: Option<Py<Vec3>>, ang_vel: Option<Py<Vec3>>) -> PyResult<Self> {
        Ok(Self {
            pos: pos.unwrap_or(new_gil_default!(Vec3, py)),
            vel: vel.unwrap_or(new_gil_default!(Vec3, py)),
            ang_vel: ang_vel.unwrap_or(new_gil_default!(Vec3, py)),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Ball(pos={}, vel={}, ang_vel={})",
            self.pos.borrow(py).__repr__(),
            self.vel.borrow(py).__repr__(),
            self.ang_vel.borrow(py).__repr__()
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct WheelPairConfig {
    wheel_radius: f32,
    suspension_rest_length: f32,
    connection_point_offset: Py<Vec3>,
}

impl PyDefault for WheelPairConfig {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            wheel_radius: 0.0,
            suspension_rest_length: 0.0,
            connection_point_offset: new_gil!(Vec3, py, Vec3::ZERO),
        })
    }
}

impl RemoveGil<csim::WheelPairConfig> for &WheelPairConfig {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::WheelPairConfig {
        csim::WheelPairConfig {
            wheel_radius: self.wheel_radius,
            suspension_rest_length: self.suspension_rest_length,
            connection_point_offset: self.connection_point_offset.clone().remove_gil(py),
        }
    }
}

impl FromGil<csim::WheelPairConfig> for WheelPairConfig {
    #[inline]
    fn from_gil(py: Python, config: csim::WheelPairConfig) -> PyResult<Self> {
        Ok(Self {
            wheel_radius: config.wheel_radius,
            suspension_rest_length: config.suspension_rest_length,
            connection_point_offset: new_gil!(Vec3, py, config.connection_point_offset),
        })
    }
}

#[pymethods]
impl WheelPairConfig {
    #[new]
    #[inline]
    #[pyo3(signature = (wheel_radius=0., suspension_rest_length=0., connection_point_offset=None))]
    fn __new__(py: Python, wheel_radius: f32, suspension_rest_length: f32, connection_point_offset: Option<Py<Vec3>>) -> PyResult<Self> {
        Ok(Self {
            wheel_radius,
            suspension_rest_length,
            connection_point_offset: connection_point_offset.unwrap_or(new_gil!(Vec3, py, Vec3::ZERO)),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self, py: Python) -> String {
        format!(
            "WheelPairConfig(wheel_radius={}, suspension_rest_length={}, connection_point_offset={})",
            self.wheel_radius,
            self.suspension_rest_length,
            self.connection_point_offset.borrow(py).__repr__()
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct CarConfig {
    hitbox_size: Py<Vec3>,
    hitbox_pos_offset: Py<Vec3>,
    front_wheels: Py<WheelPairConfig>,
    back_wheels: Py<WheelPairConfig>,
    dodge_deadzone: f32,
}

impl PyDefault for CarConfig {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            hitbox_size: new_gil_default!(Vec3, py),
            hitbox_pos_offset: new_gil_default!(Vec3, py),
            front_wheels: new_gil_default!(WheelPairConfig, py),
            back_wheels: new_gil_default!(WheelPairConfig, py),
            dodge_deadzone: 0.,
        })
    }
}

impl FromGil<csim::CarConfig> for CarConfig {
    #[inline]
    fn from_gil(py: Python, config: csim::CarConfig) -> PyResult<Self> {
        Ok(Self {
            hitbox_size: new_gil!(Vec3, py, config.hitbox_size),
            hitbox_pos_offset: new_gil!(Vec3, py, config.hitbox_pos_offset),
            front_wheels: new_gil!(WheelPairConfig, py, config.front_wheels),
            back_wheels: new_gil!(WheelPairConfig, py, config.back_wheels),
            dodge_deadzone: config.dodge_deadzone,
        })
    }
}

impl FromGil<&'static csim::CarConfig> for CarConfig {
    #[inline]
    fn from_gil(py: Python, config: &'static csim::CarConfig) -> PyResult<Self> {
        Ok(Self {
            hitbox_size: new_gil!(Vec3, py, config.hitbox_size),
            hitbox_pos_offset: new_gil!(Vec3, py, config.hitbox_pos_offset),
            front_wheels: new_gil!(WheelPairConfig, py, config.front_wheels),
            back_wheels: new_gil!(WheelPairConfig, py, config.back_wheels),
            dodge_deadzone: config.dodge_deadzone,
        })
    }
}

impl RemoveGil<csim::CarConfig> for &CarConfig {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::CarConfig {
        csim::CarConfig {
            hitbox_size: self.hitbox_size.clone().remove_gil(py),
            hitbox_pos_offset: self.hitbox_pos_offset.clone().remove_gil(py),
            front_wheels: self.front_wheels.borrow(py).remove_gil(py),
            back_wheels: self.back_wheels.borrow(py).remove_gil(py),
            dodge_deadzone: self.dodge_deadzone,
        }
    }
}

#[pymethods]
impl CarConfig {
    #[new]
    #[inline]
    fn __new__(
        py: Python,
        hitbox_size: Option<Py<Vec3>>,
        hitbox_pos_offset: Option<Py<Vec3>>,
        front_wheels: Option<Py<WheelPairConfig>>,
        back_wheels: Option<Py<WheelPairConfig>>,
        dodge_deadzone: Option<f32>,
    ) -> PyResult<Self> {
        Ok(Self {
            hitbox_size: hitbox_size.unwrap_or(new_gil_default!(Vec3, py)),
            hitbox_pos_offset: hitbox_pos_offset.unwrap_or(new_gil_default!(Vec3, py)),
            front_wheels: front_wheels.unwrap_or(new_gil_default!(WheelPairConfig, py)),
            back_wheels: back_wheels.unwrap_or(new_gil_default!(WheelPairConfig, py)),
            dodge_deadzone: dodge_deadzone.unwrap_or(0.5),
        })
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "CarConfig(hitbox_size={}, hitbox_pos_offset={}, front_wheels={}, back_wheels={}, dodge_deadzone={})",
            self.hitbox_size.borrow(py).__repr__(),
            self.hitbox_pos_offset.borrow(py).__repr__(),
            self.front_wheels.borrow(py).__repr__(py),
            self.back_wheels.borrow(py).__repr__(py),
            self.dodge_deadzone
        )
    }

    #[inline]
    #[staticmethod]
    fn octane(py: Python) -> PyResult<Self> {
        csim::CarConfig::octane().into_gil(py)
    }

    #[inline]
    #[staticmethod]
    fn dominus(py: Python) -> PyResult<Self> {
        csim::CarConfig::dominus().into_gil(py)
    }

    #[inline]
    #[staticmethod]
    fn plank(py: Python) -> PyResult<Self> {
        csim::CarConfig::plank().into_gil(py)
    }

    #[inline]
    #[staticmethod]
    fn breakout(py: Python) -> PyResult<Self> {
        csim::CarConfig::breakout().into_gil(py)
    }

    #[inline]
    #[staticmethod]
    fn hybrid(py: Python) -> PyResult<Self> {
        csim::CarConfig::hybrid().into_gil(py)
    }

    #[inline]
    #[staticmethod]
    fn merc(py: Python) -> PyResult<Self> {
        csim::CarConfig::merc().into_gil(py)
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    #[new]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (throttle=0., steer=0., pitch=0., yaw=0., roll=0., jump=false, boost=false, handbrake=false))]
    fn new(throttle: f32, steer: f32, pitch: f32, yaw: f32, roll: f32, jump: bool, boost: bool, handbrake: bool) -> Self {
        Self {
            throttle,
            steer,
            pitch,
            yaw,
            roll,
            jump,
            boost,
            handbrake,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "CarControls(throttle={}, steer={}, pitch={}, yaw={}, roll={}, jump={}, boost={}, handbrake={})",
            self.throttle,
            self.steer,
            self.pitch,
            self.yaw,
            self.roll,
            repr_bool(self.jump),
            repr_bool(self.boost),
            repr_bool(self.handbrake)
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct Car {
    pos: Py<Vec3>,
    rot_mat: Py<RotMat>,
    vel: Py<Vec3>,
    ang_vel: Py<Vec3>,
    is_on_ground: bool,
    has_jumped: bool,
    has_double_jumped: bool,
    has_flipped: bool,
    last_rel_dodge_torque: Py<Vec3>,
    jump_time: f32,
    flip_time: f32,
    is_jumping: bool,
    air_time_since_jump: f32,
    boost: f32,
    time_spent_boosting: f32,
    is_supersonic: bool,
    supersonic_time: f32,
    handbrake_val: f32,
    is_auto_flipping: bool,
    auto_flip_timer: f32,
    auto_flip_torque_scale: f32,
    has_contact: bool,
    contact_normal: Py<Vec3>,
    other_car_id: u32,
    cooldown_timer: f32,
    is_demoed: bool,
    demo_respawn_timer: f32,
    ball_hit_info: Py<BallHitInfo>,
    last_controls: Py<CarControls>,
}

impl PyDefault for Car {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            pos: new_gil_default!(Vec3, py),
            rot_mat: new_gil_default!(RotMat, py),
            vel: new_gil_default!(Vec3, py),
            ang_vel: new_gil_default!(Vec3, py),
            is_on_ground: false,
            has_jumped: false,
            has_double_jumped: false,
            has_flipped: false,
            last_rel_dodge_torque: new_gil_default!(Vec3, py),
            jump_time: 0.0,
            flip_time: 0.0,
            is_jumping: false,
            air_time_since_jump: 0.0,
            boost: 0.0,
            time_spent_boosting: 0.0,
            is_supersonic: false,
            supersonic_time: 0.0,
            handbrake_val: 0.0,
            is_auto_flipping: false,
            auto_flip_timer: 0.0,
            auto_flip_torque_scale: 0.0,
            has_contact: false,
            contact_normal: new_gil_default!(Vec3, py),
            other_car_id: 0,
            cooldown_timer: 0.0,
            is_demoed: false,
            demo_respawn_timer: 0.0,
            ball_hit_info: new_gil_default!(BallHitInfo, py),
            last_controls: new_gil_default!(CarControls, py),
        })
    }
}

impl FromGil<csim::CarState> for Car {
    #[inline]
    fn from_gil(py: Python, car: csim::CarState) -> PyResult<Self> {
        Ok(Self {
            pos: new_gil!(Vec3, py, car.pos),
            rot_mat: new_gil!(RotMat, py, car.rot_mat),
            vel: new_gil!(Vec3, py, car.vel),
            ang_vel: new_gil!(Vec3, py, car.ang_vel),
            is_on_ground: car.is_on_ground,
            has_jumped: car.has_jumped,
            has_double_jumped: car.has_double_jumped,
            has_flipped: car.has_flipped,
            last_rel_dodge_torque: new_gil!(Vec3, py, car.last_rel_dodge_torque),
            jump_time: car.jump_time,
            flip_time: car.flip_time,
            is_jumping: car.is_jumping,
            air_time_since_jump: car.air_time_since_jump,
            boost: car.boost,
            time_spent_boosting: car.time_spent_boosting,
            is_supersonic: car.is_supersonic,
            supersonic_time: car.supersonic_time,
            handbrake_val: car.handbrake_val,
            is_auto_flipping: car.is_auto_flipping,
            auto_flip_timer: car.auto_flip_timer,
            auto_flip_torque_scale: car.auto_flip_torque_scale,
            has_contact: car.has_contact,
            contact_normal: new_gil!(Vec3, py, car.contact_normal),
            other_car_id: car.other_car_id,
            cooldown_timer: car.cooldown_timer,
            is_demoed: car.is_demoed,
            demo_respawn_timer: car.demo_respawn_timer,
            ball_hit_info: new_gil!(BallHitInfo, py, car.ball_hit_info),
            last_controls: new_gil!(CarControls, py, car.last_controls),
        })
    }
}

impl RemoveGil<csim::CarState> for &Car {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::CarState {
        csim::CarState {
            pos: self.pos.clone().remove_gil(py),
            rot_mat: self.rot_mat.borrow(py).clone().remove_gil(py),
            vel: self.vel.clone().remove_gil(py),
            ang_vel: self.ang_vel.clone().remove_gil(py),
            is_on_ground: self.is_on_ground,
            has_jumped: self.has_jumped,
            has_double_jumped: self.has_double_jumped,
            has_flipped: self.has_flipped,
            last_rel_dodge_torque: self.last_rel_dodge_torque.clone().remove_gil(py),
            jump_time: self.jump_time,
            flip_time: self.flip_time,
            is_jumping: self.is_jumping,
            air_time_since_jump: self.air_time_since_jump,
            boost: self.boost,
            time_spent_boosting: self.time_spent_boosting,
            is_supersonic: self.is_supersonic,
            supersonic_time: self.supersonic_time,
            handbrake_val: self.handbrake_val,
            is_auto_flipping: self.is_auto_flipping,
            auto_flip_timer: self.auto_flip_timer,
            auto_flip_torque_scale: self.auto_flip_torque_scale,
            has_contact: self.has_contact,
            contact_normal: self.contact_normal.clone().remove_gil(py),
            other_car_id: self.other_car_id,
            cooldown_timer: self.cooldown_timer,
            is_demoed: self.is_demoed,
            demo_respawn_timer: self.demo_respawn_timer,
            ball_hit_info: self.ball_hit_info.borrow(py).remove_gil(py),
            last_controls: self.last_controls.clone().remove_gil(py),
        }
    }
}

impl RemoveGil<csim::CarState> for Car {
    #[inline]
    fn remove_gil(self, py: Python) -> csim::CarState {
        csim::CarState {
            pos: self.pos.remove_gil(py),
            rot_mat: self.rot_mat.borrow(py).clone().remove_gil(py),
            vel: self.vel.remove_gil(py),
            ang_vel: self.ang_vel.remove_gil(py),
            is_on_ground: self.is_on_ground,
            has_jumped: self.has_jumped,
            has_double_jumped: self.has_double_jumped,
            has_flipped: self.has_flipped,
            last_rel_dodge_torque: self.last_rel_dodge_torque.remove_gil(py),
            jump_time: self.jump_time,
            flip_time: self.flip_time,
            is_jumping: self.is_jumping,
            air_time_since_jump: self.air_time_since_jump,
            boost: self.boost,
            time_spent_boosting: self.time_spent_boosting,
            is_supersonic: self.is_supersonic,
            supersonic_time: self.supersonic_time,
            handbrake_val: self.handbrake_val,
            is_auto_flipping: self.is_auto_flipping,
            auto_flip_timer: self.auto_flip_timer,
            auto_flip_torque_scale: self.auto_flip_torque_scale,
            has_contact: self.has_contact,
            contact_normal: self.contact_normal.remove_gil(py),
            other_car_id: self.other_car_id,
            cooldown_timer: self.cooldown_timer,
            is_demoed: self.is_demoed,
            demo_respawn_timer: self.demo_respawn_timer,
            ball_hit_info: self.ball_hit_info.borrow(py).remove_gil(py),
            last_controls: self.last_controls.remove_gil(py),
        }
    }
}

#[pymethods]
impl Car {
    #[new]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        pos=None,
        rot_mat=None,
        vel=None,
        ang_vel=None,
        is_on_ground=false,
        has_jumped=false,
        has_double_jumped=false,
        has_flipped=false,
        last_rel_dodge_torque=None,
        jump_time=0.,
        flip_time=0.,
        is_jumping=false,
        air_time_since_jump=0.,
        boost=0.,
        time_spent_boosting=0.,
        is_supersonic=false,
        supersonic_time=0.,
        handbrake_val=0.,
        is_auto_flipping=false,
        auto_flip_timer=0.,
        auto_flip_torque_scale=0.,
        has_contact=false,
        contact_normal=None,
        other_car_id=0,
        cooldown_timer=0.,
        is_demoed=false,
        demo_respawn_timer=0.,
        ball_hit_info=None,
        last_controls=None
    ))]
    fn __new__(
        py: Python,
        pos: Option<Py<Vec3>>,
        rot_mat: Option<Py<RotMat>>,
        vel: Option<Py<Vec3>>,
        ang_vel: Option<Py<Vec3>>,
        is_on_ground: bool,
        has_jumped: bool,
        has_double_jumped: bool,
        has_flipped: bool,
        last_rel_dodge_torque: Option<Py<Vec3>>,
        jump_time: f32,
        flip_time: f32,
        is_jumping: bool,
        air_time_since_jump: f32,
        boost: f32,
        time_spent_boosting: f32,
        is_supersonic: bool,
        supersonic_time: f32,
        handbrake_val: f32,
        is_auto_flipping: bool,
        auto_flip_timer: f32,
        auto_flip_torque_scale: f32,
        has_contact: bool,
        contact_normal: Option<Py<Vec3>>,
        other_car_id: u32,
        cooldown_timer: f32,
        is_demoed: bool,
        demo_respawn_timer: f32,
        ball_hit_info: Option<Py<BallHitInfo>>,
        last_controls: Option<Py<CarControls>>,
    ) -> PyResult<Self> {
        Ok(Self {
            pos: pos.unwrap_or(new_gil_default!(Vec3, py)),
            rot_mat: rot_mat.unwrap_or(new_gil_default!(RotMat, py)),
            vel: vel.unwrap_or(new_gil_default!(Vec3, py)),
            ang_vel: ang_vel.unwrap_or(new_gil_default!(Vec3, py)),
            is_on_ground,
            has_jumped,
            has_double_jumped,
            has_flipped,
            last_rel_dodge_torque: last_rel_dodge_torque.unwrap_or(new_gil_default!(Vec3, py)),
            jump_time,
            flip_time,
            is_jumping,
            air_time_since_jump,
            boost,
            time_spent_boosting,
            is_supersonic,
            supersonic_time,
            handbrake_val,
            is_auto_flipping,
            auto_flip_timer,
            auto_flip_torque_scale,
            has_contact,
            contact_normal: contact_normal.unwrap_or(new_gil_default!(Vec3, py)),
            other_car_id,
            cooldown_timer,
            is_demoed,
            demo_respawn_timer,
            ball_hit_info: ball_hit_info.unwrap_or(new_gil_default!(BallHitInfo, py)),
            last_controls: last_controls.unwrap_or(new_gil_default!(CarControls, py)),
        })
    }

    #[inline]
    pub fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Car(pos={}, rot_mat={}, vel={}, ang_vel={}, is_on_ground={}, has_jumped={}, has_double_jumped={}, has_flipped={}, last_rel_dodge_torque={}, jump_time={}, flip_time={}, is_jumping={}, air_time_since_jump={}, boost={}, time_spent_boosting={}, is_supersonic={}, supersonic_time={}, handbrake_val={}, is_auto_flipping={}, auto_flip_timer={}, auto_flip_torque_scale={}, has_contact={}, contact_normal={}, other_car_id={}, cooldown_timer={}, is_demoed={}, demo_respawn_timer={}, ball_hit_info={}, last_controls={})",
            self.pos.borrow(py).__repr__(),
            self.rot_mat.borrow(py).__repr__(py),
            self.vel.borrow(py).__repr__(),
            self.ang_vel.borrow(py).__repr__(),
            repr_bool(self.is_on_ground),
            repr_bool(self.has_jumped),
            repr_bool(self.has_double_jumped),
            repr_bool(self.has_flipped),
            self.last_rel_dodge_torque.borrow(py).__repr__(),
            self.jump_time,
            self.flip_time,
            repr_bool(self.is_jumping),
            self.air_time_since_jump,
            self.boost,
            self.time_spent_boosting,
            repr_bool(self.is_supersonic),
            self.supersonic_time,
            self.handbrake_val,
            repr_bool(self.is_auto_flipping),
            self.auto_flip_timer,
            self.auto_flip_torque_scale,
            repr_bool(self.has_contact),
            self.contact_normal.borrow(py).__repr__(),
            self.other_car_id,
            self.cooldown_timer,
            repr_bool(self.is_demoed),
            self.demo_respawn_timer,
            self.ball_hit_info.borrow(py).__repr__(py),
            self.last_controls.borrow(py).__repr__()
        )
    }

    #[inline]
    fn get_contacting_car(&self, py: Python, arena: &mut Arena) -> PyResult<Option<Self>> {
        Ok(if let Some(car) = self.remove_gil(py).get_contacting_car(arena.0.pin_mut()) {
            Some(car.into_gil(py)?)
        } else {
            None
        })
    }
}

#[pyclass(get_all, frozen, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoostPadStatic {
    pos: Vec3,
    is_big: bool,
}

#[pymethods]
impl BoostPadStatic {
    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(set_all, get_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoostPadState {
    pub is_active: bool,
    pub cooldown: f32,
    pub cur_locked_car_id: u32,
    pub prev_locked_car_id: u32,
}

impl From<csim::BoostPadState> for BoostPadState {
    #[inline]
    fn from(boost_pad_state: csim::BoostPadState) -> Self {
        Self {
            is_active: boost_pad_state.is_active,
            cooldown: boost_pad_state.cooldown,
            cur_locked_car_id: boost_pad_state.cur_locked_car_id,
            prev_locked_car_id: boost_pad_state.prev_locked_car_id,
        }
    }
}

impl From<BoostPadState> for csim::BoostPadState {
    #[inline]
    fn from(boost_pad_state: BoostPadState) -> Self {
        Self {
            is_active: boost_pad_state.is_active,
            cooldown: boost_pad_state.cooldown,
            cur_locked_car_id: boost_pad_state.cur_locked_car_id,
            prev_locked_car_id: boost_pad_state.prev_locked_car_id,
        }
    }
}

impl From<&BoostPadState> for csim::BoostPadState {
    #[inline]
    fn from(boost_pad_state: &BoostPadState) -> Self {
        Self {
            is_active: boost_pad_state.is_active,
            cooldown: boost_pad_state.cooldown,
            cur_locked_car_id: boost_pad_state.cur_locked_car_id,
            prev_locked_car_id: boost_pad_state.prev_locked_car_id,
        }
    }
}

#[pymethods]
impl BoostPadState {
    #[new]
    #[inline]
    #[pyo3(signature = (is_active=true, cooldown=0., cur_locked_car_id=0, prev_locked_car_id=0))]
    fn __new__(is_active: bool, cooldown: f32, cur_locked_car_id: u32, prev_locked_car_id: u32) -> Self {
        Self {
            is_active,
            cooldown,
            cur_locked_car_id,
            prev_locked_car_id,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    pub fn __repr__(&self) -> String {
        format!(
            "BoostPadState(is_active={}, cooldown={}, cur_locked_car_id={}, prev_locked_car_id={})",
            repr_bool(self.is_active),
            self.cooldown,
            self.cur_locked_car_id,
            self.prev_locked_car_id
        )
    }
}

#[pyclass(unsendable, module = "rocketsim.sim")]
#[repr(transparent)]
pub struct Arena(UniquePtr<csim::Arena>);

impl PartialEq for Arena {
    fn eq(&self, other: &Self) -> bool {
        self.0.num_cars() == other.0.num_cars()
            && self.0.num_pads() == other.0.num_pads()
            && self.0.get_tick_rate() == other.0.get_tick_rate()
            && self.0.get_tick_count() == other.0.get_tick_count()
            && self.0.get_cars().iter().zip(other.0.get_cars().iter()).all(|(a, b)| a == b)
            && self.0.iter_pad_state().zip(other.0.iter_pad_state()).all(|(a, b)| a == b)
    }
}

#[pymethods]
impl Arena {
    #[new]
    #[inline]
    #[pyo3(signature = (gamemode = GameMode::Soccar, tick_rate=120.))]
    fn __new__(gamemode: GameMode, tick_rate: f32) -> Self {
        Self(csim::Arena::new(gamemode.into(), tick_rate).within_unique_ptr())
    }

    #[inline]
    fn get_tick_rate(&self) -> f32 {
        self.0.get_tick_rate()
    }

    #[inline]
    fn get_tick_count(&self) -> u64 {
        self.0.get_tick_count()
    }

    #[inline]
    fn step(&mut self, ticks_to_simulate: Option<i32>) {
        self.0.pin_mut().step(ticks_to_simulate.unwrap_or(1));
    }

    #[inline]
    fn get_ball(&mut self, py: Python) -> PyResult<Ball> {
        self.0.pin_mut().get_ball().into_gil(py)
    }

    #[inline]
    fn set_ball(&mut self, py: Python, ball: Ball) {
        self.0.pin_mut().set_ball(ball.remove_gil(py));
    }

    #[inline]
    fn num_cars(&self) -> usize {
        self.0.num_cars()
    }

    #[inline]
    fn add_car(&mut self, py: Python, team: Team, config: &CarConfig) -> u32 {
        self.0.pin_mut().add_car(team.into(), &config.remove_gil(py))
    }

    #[inline]
    fn set_car_controls(&mut self, id: u32, controls: &CarControls) -> PyResult<()> {
        self.0.pin_mut().set_car_controls(id, controls.into()).map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn set_all_controls(&mut self, controls: Vec<(u32, CarControls)>) -> PyResult<()> {
        self.0
            .pin_mut()
            .set_all_controls(&controls.iter().map(|(id, controls)| (*id, controls.into())).collect::<Vec<_>>())
            .map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn get_cars(&self) -> Vec<u32> {
        self.0.get_cars()
    }

    #[inline]
    fn get_car(&mut self, py: Python, id: u32) -> PyResult<Car> {
        self.0.pin_mut().get_car(id).into_gil(py)
    }

    #[inline]
    fn get_ball_rotation(&self, py: Python) -> PyResult<RotMat> {
        Quat::from_array(self.0.get_ball_rotation()).into_gil(py)
    }

    #[inline]
    fn set_car(&mut self, py: Python, id: u32, car: Car) -> PyResult<()> {
        self.0.pin_mut().set_car(id, car.remove_gil(py)).map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn num_pads(&self) -> usize {
        self.0.num_pads()
    }

    #[inline]
    fn get_pad_static(&self, index: usize) -> BoostPadStatic {
        BoostPadStatic {
            pos: self.0.get_pad_pos(index).into(),
            is_big: self.0.get_pad_is_big(index),
        }
    }

    #[inline]
    fn get_pad_state(&self, index: usize) -> BoostPadState {
        self.0.get_pad_state(index).into()
    }

    #[inline]
    fn set_pad_state(&mut self, index: usize, state: &BoostPadState) {
        self.0.pin_mut().set_pad_state(index, state.into())
    }

    #[inline]
    fn get_game_state(&mut self, py: Python) -> PyResult<GameState> {
        Ok(GameState {
            tick_rate: self.0.get_tick_rate(),
            tick_count: self.0.get_tick_count(),
            ball: new_gil!(Ball, py, self.0.pin_mut().get_ball()),
            ball_rot: new_gil!(RotMat, py, Quat::from_array(self.0.pin_mut().get_ball_rotation())),
            cars: self
                .0
                .GetCars()
                .iter()
                .map(|&car_id| self.0.pin_mut().get_car_info(car_id).into_gil(py).and_then(|car: CarInfo| Py::new(py, car)))
                .collect::<Result<_, _>>()?,
            pads: self
                .0
                .iter_pads()
                .map(|pad| pad.into_gil(py).and_then(|pad: BoostPad| Py::new(py, pad)))
                .collect::<Result<_, _>>()?,
        })
    }

    #[inline]
    fn set_game_state(&mut self, py: Python, game_state: GameState) -> PyResult<()> {
        self.0
            .pin_mut()
            .set_game_state(&game_state.remove_gil(py))
            .map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn set_goal_scored_callback(&mut self, py: Python, callback: PyObject) {
        self.0.pin_mut().set_goal_scored_callback(
            |_, team, user_info| {
                Python::with_gil(|_| {
                    let team = match team {
                        csim::Team::BLUE => Team::Blue,
                        csim::Team::ORANGE => Team::Orange,
                    };

                    unsafe {
                        let callback = user_info as *const PyAny;
                        (*callback).call1((team,)).unwrap();
                    }
                })
            },
            callback.as_ref(py) as *const PyAny as usize,
        );
    }
}
