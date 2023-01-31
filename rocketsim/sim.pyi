from enum import Enum

class GameMode(Enum):
    Soccar = 0

class Arena:
    def __init__(gamemode: GameMode, tick_rate: float) -> Arena: ...
