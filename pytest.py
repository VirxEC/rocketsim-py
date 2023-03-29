import unittest
from time import time_ns

from rocketsim import RotMat, Vec3
from rocketsim.sim import Arena, CarConfig, CarControls, GameMode, Team

if __name__ == "__main__":
    pass

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
        ball.pos = ball.pos.with_z(1500)
        arena.set_ball(ball)
        ball2 = arena.get_ball()
        self.assertEqual(ball2.pos.x, ball.pos.x)
        self.assertEqual(ball2.pos.y, ball.pos.y)
        self.assertEqual(ball2.pos.z, 1500)

    def test_add_car(self):
        arena = Arena(GameMode.Soccar, 120)
        self.assertEqual(arena.num_cars(), 0)
        car_id = arena.add_car(Team.Blue, CarConfig.Octane)
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
        car_id = arena.add_car(Team.Blue, CarConfig.Octane)

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
        car_id = arena.add_car(Team.Orange, CarConfig.Octane)
        ball = arena.get_ball()

        ball.pos = Vec3(0., -5119., 184.)
        ball.vel = Vec3(0., -6600., 0.)

        ball.hit_info.car_id = car_id
        ball.hit_info.ball_pos = ball.pos
        
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
        
