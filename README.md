![image](https://user-images.githubusercontent.com/36944229/219303954-7267bce1-b7c5-4f15-881c-b9545512e65b.png)

**Python bindings for the [RocketSim](https://github.com/ZealanL/RocketSim) project**

Compatible with Python 3.7+, Windows and Linux.

Produces just 2 wheels, and include IDE type hints!

# Using these bindings

Download them from PyPi with `pip install rocketsim` or from the [latest release](https://github.com/VirxEC/rocketsim-py/releases)!

Here's what the files do in the latest release:
 - `rocketsim.zip` - Unzip this into your project and have Windows & Linux support for Python 3.7+!
 - `rocketsim-X.X.X-cp37-abi3-win_amd64.whl` - The Windows-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Windows.
 - `rocketsim-X.X.X-cp37-abi3-manylinux_2_28_x86_64.whl` - The Linux-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Linux.
 - `rocketsim-X.X.X.tar.gz` - The minimal source code of this project. Unzip and it contains all the files you need to build the bindings yourself, for your platform and architecture. Beware that this doesn't included any of the required tools/dependencies to build the bindings!

# Example

```python
from rocketsim import RotMat, Vec3
from rocketsim.sim import Arena, CarConfig, CarControls, GameMode, Team

if __name__ == "__main__":
    arena = Arena(GameMode.Soccar)
    print(f"Arena tick rate: {arena.get_tick_rate()}")

    car_id = arena.add_car(Team.Blue, CarConfig.octane())
    print(f"ID of added car: {car_id}")

    game_state = arena.get_game_state()
    print("Got game state")

    game_state.ball.pos.z = 1500
    print("Configured ball state")

    car = game_state.cars[0].state
    car.pos = Vec3(0, 0, 1050)
    car.rot_mat = RotMat.from_angles(0, 1.1, 0)
    car.boost = 100
    print("Configured car state")

    arena.set_game_state(game_state)
    print("Set game state")

    arena.set_car_controls(car_id, CarControls(boost=True))
    print("Set car controls")

    ticks = 7200

    arena.step(ticks)

    inactive_pads = 0

    for i in range(arena.num_pads()):
        pad = arena.get_pad_state(i)
        if not pad.is_active:
            inactive_pads += 1
            pos = arena.get_pad_static(i).pos
            print(f"Pad {i} is inactive at {pos}")

    if inactive_pads == 0:
        print("All pads are active")
```
