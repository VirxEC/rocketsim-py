from time import time_ns

from rocketsim import Angle, Vec3
from rocketsim.sim import Arena, CarConfig, GameMode, Team, CarControls

if __name__ == "__main__":
    arena = Arena(GameMode.Soccar)
    print(f"Arena tick rate: {arena.get_tick_rate()}")

    ball = arena.get_ball()
    ball.pos = ball.get_pos().with_z(1500)
    arena.ball = ball
    print("Set ball state")

    car_id = arena.add_car(Team.Blue, CarConfig.Octane)
    print(f"ID of added car: {car_id}")

    car = arena.get_car(car_id)
    car.pos = Vec3(0, 0, 1050)
    car.angles = Angle(0, 1.1, 0)
    car.boost = 100
    arena.set_car(car_id, car)
    print("Set car state")

    arena.set_car_controls(car_id, CarControls(boost=True))
    print("Set car controls")

    ticks = 7200

    start_time = time_ns()
    arena.step(ticks)
    end_time = time_ns()

    inactive_pads = 0

    for i in range(arena.num_pads()):
        pad = arena.get_pad_state(i)
        if not pad.is_active:
            inactive_pads += 1
            pos = arena.get_pad_static(i).pos
            print(f"Pad {i} is inactive at {pos}")

    if inactive_pads == 0:
        print("All pads are active")

    print(f"Simulated {ticks / 120}s of game time in {(end_time - start_time) / 1e6}ms real time")
