from time import time_ns

from rocketsim import RotMat, Vec3
from rocketsim.sim import Arena, CarConfig, CarControls, GameMode, Team

print("Starting benchmark of arena config without game state...")

times = []
for _ in range(1800):
    start = time_ns()

    arena = Arena(GameMode.Soccar)

    ball = arena.get_ball()
    ball.pos.z = 1500
    arena.set_ball(ball)

    for _ in range(3):
        arena.add_car(Team.Blue, CarConfig.octane())

    for _ in range(3):
        arena.add_car(Team.Orange, CarConfig.octane())

    car_id = 1
    car = arena.get_car(car_id)
    car.pos = Vec3(0, 0, 1050)
    car.rot_mat = RotMat.from_angles(0, 1.1, 0)
    car.boost = 100

    arena.set_car(car_id, car)
    arena.set_car_controls(car_id, CarControls(boost=True))

    for i in range(arena.num_pads()):
        arena.set_pad_state(i, arena.get_pad_state(i))

    arena.step()

    arena.get_tick_count()
    arena.get_tick_rate()
    arena.get_ball()

    for id in arena.get_cars():
        arena.get_car(car_id)

    for i in range(arena.num_pads()):
        arena.get_pad_static(i)
        arena.get_pad_state(i)

    end = time_ns()
    times.append(end - start)

avg_time = sum(times) / len(times)
print(f"Average time: {round(avg_time / 1e6, 3)}ms")

print("Starting benchmark of arena config with game state...")

times = []
for _ in range(1800):
    start = time_ns()

    arena = Arena(GameMode.Soccar)

    for _ in range(3):
        arena.add_car(Team.Blue, CarConfig.octane())

    for _ in range(3):
        arena.add_car(Team.Orange, CarConfig.octane())

    game_state = arena.get_game_state()
    game_state.ball.pos.z = 1500
    game_state.cars[0].state.pos = Vec3(0, 0, 1050)
    game_state.cars[0].state.rot_mat = RotMat.from_angles(0, 1.1, 0)
    game_state.cars[0].state.boost = 100
    arena.set_car_controls(game_state.cars[0].id, CarControls(boost=True))
    arena.set_game_state(game_state)

    arena.step()

    arena.get_game_state()

    end = time_ns()
    times.append(end - start)

avg_time = sum(times) / len(times)
print(f"Average time: {round(avg_time / 1e6, 3)}ms")
