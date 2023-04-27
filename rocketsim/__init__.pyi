from rocketsim.sim import Ball, Car, Team


class Vec3:
    x: float
    y: float
    z: float

    def __init__(x: float, y: float, z: float) -> Vec3: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def with_x(self, x: float) -> Vec3: ...
    def with_y(self, y: float) -> Vec3: ...
    def with_z(self, z: float) -> Vec3: ...

class RotMat:
    forward: Vec3
    right: Vec3
    up: Vec3

    def __init__(forward: Vec3, right: Vec3, up: Vec3) -> RotMat: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    @staticmethod
    def from_angles(pitch: float, yaw: float, roll: float) -> RotMat: ...

class GameState:
    tick_rate: float
    tick_count: int
    ball: Ball
    ball_rot: RotMat
    cars: list[CarInfo]

class CarInfo:
    id: int
    team: Team
    state: Car
    # config: CarConfig
