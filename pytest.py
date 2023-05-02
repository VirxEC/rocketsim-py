if __name__ == "__main__":
    print("Please run as unittest, for example: python -m unittest pytest.py")
    exit()

import unittest
from time import time_ns

from rocketsim import RotMat, Vec3
from rocketsim.sim import (Arena, Ball, BallHitInfo, BoostPadState, CarConfig, CarControls,
                           GameMode, Team, WheelPairConfig)


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

        game_state.ball.pos = Vec3(0., -5119, 184)
        game_state.ball.vel.y = -6600

        self.assertEqual(game_state.ball.pos.x, 0.)
        self.assertEqual(game_state.ball.pos.y, -5119)
        self.assertEqual(game_state.ball.pos.z, 184)

        self.assertEqual(game_state.ball.vel.x, 0)
        self.assertEqual(game_state.ball.vel.y, -6600)
        self.assertEqual(game_state.ball.vel.z, 0)

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

    def init_test(self):
        vec = Vec3(1, z=3)
        self.assertEqual(vec.x, 1)
        self.assertEqual(vec.y, 0)
        self.assertEqual(vec.z, 3)

        rotmat = RotMat(Vec3(1, 2, 3), up=Vec3(7, 8, 9))
        self.assertEqual(rotmat.forward.x, 1)
        self.assertEqual(rotmat.forward.y, 2)
        self.assertEqual(rotmat.forward.z, 3)

        self.assertEqual(rotmat.right.x, 0)
        self.assertEqual(rotmat.right.y, 0)
        self.assertEqual(rotmat.right.z, 0)

        self.assertEqual(rotmat.up.x, 7)
        self.assertEqual(rotmat.up.y, 8)
        self.assertEqual(rotmat.up.z, 9)

        pad = BoostPadState(False, 2, prev_locked_car_id=1)
        self.assertEqual(pad.is_active, False)
        self.assertEqual(pad.cooldown, 2)
        self.assertEqual(pad.cur_locked_car_id, 0)
        self.assertEqual(pad.prev_locked_car_id, 1)

        # ball_hit_info = BallHitInfo()

        ball = Ball(Vec3(1, 2, 3), ang_vel=Vec3(4, 5, 6))
        self.assertEqual(ball.pos.x, 1)
        self.assertEqual(ball.pos.y, 2)
        self.assertEqual(ball.pos.z, 3)

        self.assertEqual(ball.vel.x, 0)
        self.assertEqual(ball.vel.y, 0)
        self.assertEqual(ball.vel.z, 0)

        self.assertEqual(ball.ang_vel.x, 4)
        self.assertEqual(ball.ang_vel.y, 5)
        self.assertEqual(ball.ang_vel.z, 6)

        controls = CarControls(0.5, 0, 0.6, boost=True, jump=True)

        self.assertEqual(controls.throttle, 0.5)
        self.assertEqual(controls.steer, 0)
        self.assertEqual(controls.pitch, 0.6)
        self.assertEqual(controls.boost, True)
        self.assertEqual(controls.jump, True)

        # car = Car()

        wheel_pair_config = WheelPairConfig(25, connection_point_offset=Vec3(1, 2, 3))
        self.assertEqual(wheel_pair_config.radius, 25)
        self.assertEqual(wheel_pair_config.connection_point_offset.x, 1)
        self.assertEqual(wheel_pair_config.connection_point_offset.y, 2)
        self.assertEqual(wheel_pair_config.connection_point_offset.z, 3)

        car_config = CarConfig(Vec3(1, 2, 3), Vec3(), dodge_deadzone=0.75)

        self.assertEqual(car_config.hitbox_size.x, 1)
        self.assertEqual(car_config.hitbox_size.y, 2)
        self.assertEqual(car_config.hitbox_size.z, 3)

        self.assertEqual(car_config.hitbox_pos_offset.x, 0)
        self.assertEqual(car_config.hitbox_pos_offset.y, 0)
        self.assertEqual(car_config.hitbox_pos_offset.z, 0)

        self.assertEqual(car_config.dodge_deadzone, 0.75)

        static = BoostPadState(False, 2, prev_locked_car_id=1)

        self.assertEqual(static.is_active, False)
        self.assertEqual(static.cooldown, 2)
        self.assertEqual(static.cur_locked_car_id, 0)
        self.assertEqual(static.prev_locked_car_id, 1)

