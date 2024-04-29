from manim import *
import math
import numpy as np

# set frame rate
config.frame_rate = 60.0

# manim converted units (40 btd6 units = 1 manim unit)
factor = 40

R = 80/factor
r = 40/factor
v_p = 1.0/factor
v_c = 1.2 * v_p
threshold_distance_squared = 1.0/(factor**2)
reverse_phase_change = PI/5  # unchanged
max_delta_angle = 0.105  # unchanged
P = TAU*R/(v_p*60)  # period

# util functions


def constrain_to_pi(angle):
    while angle > PI:
        angle -= TAU
    while angle < -PI:
        angle += TAU
    return angle


def constrain_to_tau(angle):
    return angle % TAU

# ------------------------------------------------------------ #
# ace configuration (initial)                                  #
# ------------------------------------------------------------ #


targeting = "Circle"
onPath = False
prev_loc = None
v_angle = 0  # if onPath initially true no need to set this

fig8_1 = Circle(radius=r, color=GREY)
fig8_2 = Circle(radius=r, color=GREY)
fig8_1.rotate(PI*2)
fig8_1.move_to(UP)
fig8_2.move_to(DOWN)
fig8 = Group(fig8_1, fig8_2)

figinf_1 = Circle(radius=r, color=GREY)
figinf_2 = Circle(radius=r, color=GREY)
figinf_1.move_to(RIGHT)
figinf_2.move_to(LEFT)
figinf = Group(figinf_1, figinf_2)

circle = Circle(radius=R, color=GREY)


def circlephase(phase):
    return circle.point_at_angle(-phase)


def fig8phase(phase):
    phase = constrain_to_tau(phase)
    if phase <= PI:
        return fig8_1.point_at_angle(3/2*PI - 2*phase)
    else:
        return fig8_2.point_at_angle(-3/2*PI + 2*phase)


def figinfphase(phase):
    phase = constrain_to_tau(phase)
    if phase <= PI:
        return figinf_1.point_at_angle(PI - 2*phase)
    else:
        return figinf_2.point_at_angle(2*phase)


phase = ValueTracker(0)

circledot = Dot(color='#ff0000')
circledot.add_updater(lambda x: x.move_to(circlephase(phase.get_value())))
fig8dot = Dot(color='#00ff00')
fig8dot.add_updater(lambda x: x.move_to(fig8phase(phase.get_value())))
figinfdot = Dot(color='#0000ff')
figinfdot.add_updater(lambda x: x.move_to(figinfphase(phase.get_value())))

initial_location = [0.6, 0, 0]

ace = Dot(color='#ffff00', point=initial_location, radius=0.05)

def acemove(phase):
    global targeting
    global onPath
    global prev_loc
    global v_angle

    target_point = None
    if targeting == 'Circle':
        target_point = circlephase(phase)
    elif targeting == '8':
        target_point = fig8phase(phase)
    elif targeting == 'Infinity':
        target_point = figinfphase(phase)

    if onPath:
        if prev_loc is not None:
            delta_x = target_point[0] - prev_loc[0]
            delta_y = target_point[1] - prev_loc[1]
            v_angle = math.atan2(delta_y, delta_x)

        prev_loc = target_point
        return target_point

    # i have no idea how it gets ace from the update() scope but if it works it works
    ace_position = ace.get_center()

    # onPath check
    d_squared = (target_point[0] - ace_position[0]
                 )**2 + (target_point[1] - ace_position[1])**2
    if d_squared < threshold_distance_squared:
        onPath = True

    # move the ace
    delta_x = target_point[0] - ace_position[0]
    delta_y = target_point[1] - ace_position[1]
    angle = math.atan2(delta_y, delta_x)
    angle_diff = constrain_to_pi(angle - v_angle)

    if abs(angle_diff) < max_delta_angle:
        v_angle += np.sign(angle_diff) * angle_diff
    else:
        v_angle += np.sign(angle_diff) * max_delta_angle
    # turn off checks as an example
    # v_angle += np.sign(angle_diff) * angle_diff
    v_angle = constrain_to_pi(v_angle)

    return ace_position + [v_c*math.cos(v_angle), v_c*math.sin(v_angle), 0]

ace.add_updater(lambda x: x.move_to(acemove(phase.get_value())))


class CreateCircle(Scene):
    def construct(self):
        # self.add(fig8)
        # self.add(figinf)
        self.add(circle)
        # self.play(FadeIn(circle))
        # self.add(fig8dot)
        # self.add(figinfdot)
        self.add(circledot)

        self.add(ace)

        # for the ace path
        # path = VMobject()
        # path.set_points_as_corners([[0.6, 0, 0], [0.6, 0, 0]])
        # def update_path(path):
        #     previous_path = path.copy()
        #     previous_path.add_points_as_corners([ace.get_center()])
        #     path.become(previous_path)
        # path.add_updater(update_path)
        # self.add(path)

        self.play(phase.animate.set_value(2*PI),
                  rate_func=rate_functions.linear, run_time=P)
