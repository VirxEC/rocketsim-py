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

class Angle:
    pitch: float
    yaw: float
    roll: float

    def __init__(pitch: float, yaw: float, roll: float) -> Angle: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def with_pitch(self, pitch: float) -> Angle: ...
    def with_yaw(self, yaw: float) -> Angle: ...
    def with_roll(self, roll: float) -> Angle: ...
