from enum import Enum
from typing import Optional

from rocketsim import Angle, Vec3

class Team(Enum):
    Blue = 0
    Orange = 1

class GameMode(Enum):
    Soccar = 0

class Ball:
    pos: Vec3
    vel: Vec3
    angvel: Vec3

    def __str__(self) -> str: ...

    def get_pos(self) -> Vec3: ...
    def get_vel(self) -> Vec3: ...
    def get_angvel(self) -> Vec3: ...

class CarControls:
    throttle: float
    steer: float
    pitch: float
    yaw: float
    roll: float
    jump: bool
    boost: bool
    handbrake: bool

    def __init__(throttle: float = 0, steer: float = 0, pitch: float = 0, yaw: float = 0, roll: float = 0, jump: bool = False, boost: bool = False, handbrake: bool = False) -> CarControls: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Car:
    pos: Vec3
    vel: Vec3
    angles: Angle
    angvel: Vec3
    last_rel_dodge_torque: Vec3
    boost: float
    time_spent_boosting: float
    is_on_ground: bool
    is_supersonic: bool
    supersonic_time: float
    is_jumping: bool
    has_jumped: bool
    has_double_jumped: bool
    has_flipped: bool
    jump_timer: float
    flip_timer: float
    air_time_since_jump: float
    is_auto_flipping: bool
    auto_flip_timer: float
    auto_flip_torque_scale: float
    has_contact: bool
    handbrake_val: float
    last_controls: CarControls

    def __str__(self) -> str: ...
    def get_pos(self) -> Vec3: ...
    def get_vel(self) -> Vec3: ...
    def get_angles(self) -> Angle: ...
    def get_angvel(self) -> Vec3: ...
    def get_last_rel_dodge_torque(self) -> Vec3: ...

class CarConfig(Enum):
    Octane = 0
    Dominus = 1
    Plank = 2
    Breakout = 3
    Hybrid = 4
    Merc = 5

class BoostPad:
    pos: Vec3
    is_big: bool

    def __str__(self) -> str: ...

class BoostPadState:
    id: int
    is_active: bool
    cooldown: float

    def __init__(id: int, is_active: bool, cooldown: float) -> BoostPadState: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Arena:
    ball: Ball

    def __init__(gamemode: Optional[GameMode] = GameMode.Soccar, tick_rate: Optional[float] = 120) -> Arena: ...
    def get_tick_rate(self) -> float: ...
    def get_ball(self) -> Ball: ...
    def num_cars(self) -> int: ...
    def get_car_from_index(self, index: int) -> Car: ...
    def get_car_id_from_index(self, index: int) -> int: ...
    def add_car(self, team: Team, config: CarConfig) -> int: ...
    def get_car(self, id: int) -> Car: ...
    def set_car(self, id: int, car: Car): ...
    def set_car_controls(self, id: int, controls: CarControls): ...
    def num_pads(self) -> int: ...
    def get_pad_static(self, id: int) -> BoostPad: ...
    def get_pad_state(self, id: int) -> BoostPadState: ...
    def step(self, ticks_to_simulate: int = 1): ...
