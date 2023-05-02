from enum import Enum
from typing import Callable, Optional, Tuple

from rocketsim import GameState, RotMat, Vec3

class Team(Enum):
    Blue = 0
    Orange = 1

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GameMode(Enum):
    Soccar = 0
    TheVoid = 1

class BallHitInfo:
    relative_pos_on_ball: Vec3
    ball_pos: Vec3
    extra_hit_vel: Vec3
    tick_count_when_hit: int
    tick_count_when_extra_impulse_applied: int

    def __init__(
        relative_pos_on_ball: Vec3=Vec3(),
        ball_pos: Vec3=Vec3(),
        extra_hit_vel: Vec3=Vec3(),
        tick_count_when_hit: int=0,
        tick_count_when_extra_impulse_applied: int=0
    ) -> BallHitInfo: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Ball:
    pos: Vec3
    vel: Vec3
    ang_vel: Vec3

    def __init__(pos: Vec3 = Vec3(), vel: Vec3 = Vec3(), ang_vel: Vec3 = Vec3()) -> Ball: ...
    def __str__(self) -> str: ...

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
    rot_mat: RotMat
    vel: Vec3
    ang_vel: Vec3
    is_on_ground: bool
    has_jumped: bool
    has_double_jumped: bool
    has_flipped: bool
    last_rel_dodge_torque: Vec3
    jump_time: float
    flip_time: float
    is_jumping: bool
    air_time_since_jump: float
    boost: float
    time_spent_boosting: float
    is_supersonic: bool
    supersonic_time: float
    handbrake_val: float
    is_auto_flipping: bool
    auto_flip_timer: float
    auto_flip_torque_scale: float
    has_contact: bool
    contact_normal: Vec3
    other_car_id: int
    cooldown_timer: float
    is_demoed: bool
    demo_respawn_timer: float
    ball_hit_info: BallHitInfo
    last_controls: CarControls

    def __init__(
        pos: Vec3=Vec3(),
        rot_mat: RotMat=RotMat(),
        vel: Vec3=Vec3(),
        ang_vel: Vec3=Vec3(),
        is_on_ground: bool=False,
        has_jumped: bool=False,
        has_double_jumped: bool=False,
        has_flipped: bool=False,
        last_rel_dodge_torque: Vec3=Vec3(),
        jump_time: float=0,
        flip_time: float=0,
        is_jumping: bool=False,
        air_time_since_jump: float=0,
        boost: float=0,
        time_spent_boosting: float=0,
        is_supersonic: bool=False,
        supersonic_time: float=0,
        handbrake_val: float=0,
        is_auto_flipping: bool=False,
        auto_flip_timer: float=0,
        auto_flip_torque_scale: float=0,
        has_contact: bool=False,
        contact_normal: Vec3=Vec3(),
        other_car_id: int=0,
        cooldown_timer: float=0,
        is_demoed: bool=False,
        demo_respawn_timer: float=0,
        ball_hit_info: BallHitInfo=BallHitInfo(),
        last_controls: CarControls=CarControls()
    ) -> Car: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def get_contacting_car(self, arena: Arena) -> Optional[Car]: ...

class WheelPairConfig:
    wheel_radius: float
    suspension_rest_length: float
    connection_point_offset: Vec3

    def __init__(wheel_radius: float = 0, suspension_rest_length: float = 0, connection_point_offset: Vec3 = Vec3()) -> WheelPairConfig: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class CarConfig:
    hitbox_size: Vec3
    hitbox_pos_offset: Vec3
    front_wheel: WheelPairConfig
    rear_wheel: WheelPairConfig
    dodge_deadzone: float

    def __init__(
        hitbox_size: Vec3 = Vec3(),
        hitbox_pos_offset: Vec3 = Vec3(),
        front_wheel: WheelPairConfig = WheelPairConfig(),
        back_wheel: WheelPairConfig = WheelPairConfig(),
        dodge_deadzone: float = 0.5
    ) -> CarConfig: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    @staticmethod
    def octane() -> CarConfig: ...
    @staticmethod
    def dominus() -> CarConfig: ...
    @staticmethod
    def breakout() -> CarConfig: ...
    @staticmethod
    def hybrid() -> CarConfig: ...
    @staticmethod
    def plank() -> CarConfig: ...
    @staticmethod
    def merc() -> CarConfig: ...

class BoostPadStatic:
    pos: Vec3
    is_big: bool

    def __str__(self) -> str: ...

class BoostPadState:
    is_active: bool
    cooldown: float
    cur_locked_car_id: int
    prev_locked_car_id: int

    def __init__(is_active: bool=True, cooldown: float=0, cur_locked_car_id: int=0, prev_locked_car_id: int=0) -> BoostPadState: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Arena:
    def __init__(gamemode: GameMode=GameMode.Soccar, tick_rate: float=120) -> Arena: ...
    def get_tick_rate(self) -> float: ...
    def get_tick_count(self) -> int: ...
    def step(self, ticks_to_simulate: int = 1): ...

    def get_ball(self) -> Ball: ...
    def set_ball(self, ball: Ball): ...
    def get_ball_rotation(self) -> RotMat: ...

    def num_cars(self) -> int: ...
    def add_car(self, team: Team, config: CarConfig) -> int: ...
    def get_cars(self, id: int) -> list[int]: ...
    def get_car(self, id: int) -> Car: ...
    def set_car(self, id: int, car: Car): ...
    def set_car_controls(self, id: int, controls: CarControls): ...
    def set_all_controls(self, controls: list[Tuple[int, CarControls]]): ...

    def num_pads(self) -> int: ...
    def get_pad_static(self, index: int) -> BoostPadStatic: ...
    def get_pad_state(self, index: int) -> BoostPadState: ...
    def set_pad_state(self, index: int, boost_pad: BoostPadState): ...

    def get_game_state(self) -> GameState: ...
    def set_game_state(self, game_state: GameState): ...
    def set_goal_scored_callback(self, callback: Callable[[Team], None]): ...
