from time import time_ns

from rocketsim import RotMat, Vec3
from rocketsim.sim import Arena, CarConfig, CarControls, GameMode, Team

print("Starting benchmark of arena config...")

times = []
for _ in range(1800):
    start = time_ns()
    arena = Arena(GameMode.Soccar)
    ball = arena.get_ball()
    ball.pos.z = 1500
    arena.set_ball(ball)
    car_id = arena.add_car(Team.Blue, CarConfig.octane())
    car = arena.get_car(car_id)
    car.pos = Vec3(0, 0, 1050)
    car.rot_mat = RotMat.from_angles(0, 1.1, 0)
    car.boost = 100
    arena.set_car(car_id, car)
    arena.set_car_controls(car_id, CarControls(boost=True))
    arena.step()
    for i in range(arena.num_pads()):
        arena.get_pad_static(i)
        arena.get_pad_state(i)
    end = time_ns()
    times.append(end - start)

avg_time = sum(times) / len(times)
print(f"Average time: {round(avg_time / 1e6, 3)}ms")
