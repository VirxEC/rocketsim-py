if __name__ == "__main__":
    print("Please run as unittest, for example: python -m unittest pytest.py")
    exit()

import unittest
from time import time_ns

from rocketsim import RotMat, Vec3
from rocketsim.sim import Arena, CarConfig, CarControls, GameMode, Team

class TestArena(unittest.TestCase):

    def test_tick_rate(self):
        arena = Arena(GameMode.Soccar, 120)
        self.assertAlmostEqual(arena.get_tick_rate(), 120.0, 2)

    def test_boost_pad(self):
        arena = Arena(GameMode.Soccar, 120)
        pad = arena.get_pad_static(0)
        #BoostPad { pos: Vec3 { x: -3584.0, y: 0.0, z: 73.0 }, is_big: true }
        self.assertEqual(pad.is_big, True)
        self.assertEqual(pad.pos.x, -3584.0)
        self.assertEqual(pad.pos.y, 0.0)
        self.assertEqual(pad.pos.z, 73.0)

    def test_ball(self):
        arena = Arena(GameMode.Soccar, 120)
        ball = arena.get_ball()
        ball.pos.z = 1500
        arena.set_ball(ball)
        ball2 = arena.get_ball()
        self.assertEqual(ball2.pos.x, ball.pos.x)
        self.assertEqual(ball2.pos.y, ball.pos.y)
        self.assertEqual(ball2.pos.z, 1500)

    def test_add_car(self):
        arena = Arena(GameMode.Soccar, 120)
        self.assertEqual(arena.num_cars(), 0)
        car_id = arena.add_car(Team.Blue, CarConfig.octane())
        self.assertEqual(arena.num_cars(), 1)

        car = arena.get_car(car_id)
        car.pos = Vec3(0, 0, 1050)
        car.rot_mat = RotMat.from_angles(0, 1.1, 0)
        car.boost = 100
        arena.set_car(car_id, car)
        car1 = arena.get_car(car_id)

        self.assertEqual(car1.pos.x, car.pos.x)
        self.assertEqual(car1.pos.y, car.pos.y)
        self.assertEqual(car1.pos.z, car.pos.z)

        self.assertEqual(car1.rot_mat.forward.x, car.rot_mat.forward.x)
        self.assertEqual(car1.rot_mat.forward.y, car.rot_mat.forward.y)
        self.assertEqual(car1.rot_mat.forward.z, car.rot_mat.forward.z)

        self.assertEqual(car1.rot_mat.right.x, car.rot_mat.right.x)
        self.assertEqual(car1.rot_mat.right.y, car.rot_mat.right.y)
        self.assertEqual(car1.rot_mat.right.z, car.rot_mat.right.z)

        self.assertEqual(car1.rot_mat.up.x, car.rot_mat.up.x)
        self.assertEqual(car1.rot_mat.up.y, car.rot_mat.up.y)
        self.assertEqual(car1.rot_mat.up.z, car.rot_mat.up.z)

        self.assertEqual(car1.boost, car.boost)

    def test_car_controls(self):
        arena = Arena(GameMode.Soccar, 120)
        car_id = arena.add_car(Team.Blue, CarConfig.octane())

        car = arena.get_car(car_id)
        car.pos = Vec3(0, 0, 1050)
        car.rot_mat = RotMat.from_angles(0, 1.1, 0)
        car.boost = 100
        arena.set_car(car_id, car)
        arena.set_car_controls(car_id, CarControls(boost=True))

        ticks = 7200

        start_time = time_ns()
        arena.step(ticks)
        end_time = time_ns()

        car = arena.get_car(car_id)
        self.assertEqual(car.boost, 0)

        inactive_pads = 0

        for i in range(arena.num_pads()):
            pad = arena.get_pad_state(i)
            if not pad.is_active:
                inactive_pads += 1
                pos = arena.get_pad_static(i).pos
                print(f"Pad {i} is inactive at {pos}")

        self.assertEqual(inactive_pads, 0)

        print(f"Simulated {ticks / 120}s of game time in {(end_time - start_time) / 1e6}ms real time")

    def test_goal_scored_callback(self):
        arena = Arena(GameMode.Soccar, 120)
        arena.add_car(Team.Orange, CarConfig.octane())
        ball = arena.get_ball()

        ball.pos = Vec3(0., -5119., 184.)
        ball.vel.y = -6600

        arena.set_ball(ball)

        scored_team = None
        callback_call_count = 0
        def callback(team):
            nonlocal scored_team
            nonlocal callback_call_count
            scored_team = team
            callback_call_count += 1

        arena.set_goal_scored_callback(callback)

        arena.step(2)

        self.assertEqual(scored_team, Team.Orange)
        self.assertEqual(callback_call_count, 1)

        ball2 = arena.get_ball()

        self.assertNotEqual(ball, ball2)

        arena.step(2)

        ball3 = arena.get_ball()

        self.assertNotEqual(ball3, ball2)

    def test_game_state(self):
        arena = Arena(GameMode.Soccar, 120)
        arena.add_car(Team.Orange, CarConfig.octane())

        game_state = arena.get_game_state()

        game_state.ball.pos = Vec3(0., -5119., 184.)
        game_state.ball.vel.y = -6600

        self.assertEqual(game_state.ball.pos.x, 0.)
        self.assertEqual(game_state.ball.pos.y, -5119.)
        self.assertEqual(game_state.ball.pos.z, 184.)

        self.assertEqual(game_state.ball.vel.x, 0.)
        self.assertEqual(game_state.ball.vel.y, -6600.)
        self.assertEqual(game_state.ball.vel.z, 0.)

        car = game_state.cars[0]
        car.state.pos = Vec3(0, 0, 1050)
        car.state.rot_mat = RotMat.from_angles(0, 1.1, 0)
        car.state.boost = 100

        self.assertEqual(game_state.cars[0].state.pos.x, 0)
        self.assertEqual(game_state.cars[0].state.pos.y, 0)
        self.assertEqual(game_state.cars[0].state.pos.z, 1050)

        angles = RotMat.from_angles(0, 1.1, 0)
        self.assertEqual(game_state.cars[0].state.rot_mat.forward.x, angles.forward.x)
        self.assertEqual(game_state.cars[0].state.rot_mat.forward.y, angles.forward.y)
        self.assertEqual(game_state.cars[0].state.rot_mat.forward.z, angles.forward.z)

        self.assertEqual(game_state.cars[0].state.rot_mat.right.x, angles.right.x)
        self.assertEqual(game_state.cars[0].state.rot_mat.right.y, angles.right.y)
        self.assertEqual(game_state.cars[0].state.rot_mat.right.z, angles.right.z)

        self.assertEqual(game_state.cars[0].state.rot_mat.up.x, angles.up.x)
        self.assertEqual(game_state.cars[0].state.rot_mat.up.y, angles.up.y)
        self.assertEqual(game_state.cars[0].state.rot_mat.up.z, angles.up.z)

        self.assertEqual(game_state.cars[0].state.boost, 100)

        arena.set_game_state(game_state)

        game_state2 = arena.get_game_state()

        self.assertEqual(game_state2.ball.pos.x, game_state.ball.pos.x)
        self.assertEqual(game_state2.ball.pos.y, game_state.ball.pos.y)
        # test will fail without rounding due to floating point error
        self.assertEqual(round(game_state2.ball.pos.z, 4), game_state.ball.pos.z)

        self.assertEqual(game_state2.ball.vel.x, game_state.ball.vel.x)
        self.assertEqual(game_state2.ball.vel.y, game_state.ball.vel.y)
        self.assertEqual(game_state2.ball.vel.z, game_state.ball.vel.z)

        self.assertEqual(game_state2.cars[0].state.pos.x, game_state.cars[0].state.pos.x)
        self.assertEqual(game_state2.cars[0].state.pos.y, game_state.cars[0].state.pos.y)
        self.assertEqual(game_state2.cars[0].state.pos.z, game_state.cars[0].state.pos.z)

        self.assertEqual(game_state2.cars[0].state.rot_mat.forward.x, game_state.cars[0].state.rot_mat.forward.x)
        self.assertEqual(game_state2.cars[0].state.rot_mat.forward.y, game_state.cars[0].state.rot_mat.forward.y)
        self.assertEqual(game_state2.cars[0].state.rot_mat.forward.z, game_state.cars[0].state.rot_mat.forward.z)

        self.assertEqual(game_state2.cars[0].state.rot_mat.right.x, game_state.cars[0].state.rot_mat.right.x)
        self.assertEqual(game_state2.cars[0].state.rot_mat.right.y, game_state.cars[0].state.rot_mat.right.y)
        self.assertEqual(game_state2.cars[0].state.rot_mat.right.z, game_state.cars[0].state.rot_mat.right.z)

        self.assertEqual(game_state2.cars[0].state.rot_mat.up.x, game_state.cars[0].state.rot_mat.up.x)
        self.assertEqual(game_state2.cars[0].state.rot_mat.up.y, game_state.cars[0].state.rot_mat.up.y)
        self.assertEqual(game_state2.cars[0].state.rot_mat.up.z, game_state.cars[0].state.rot_mat.up.z)

        self.assertEqual(game_state2.cars[0].state.boost, game_state.cars[0].state.boost)

        # print(repr(game_state2))
