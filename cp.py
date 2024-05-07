from manim import *
import math
import numpy as np

# set frame rate
config.frame_rate = 60.0

# manim converted units (x btd6 units = 1 manim unit)
x = 30
R = 80/x
r = 40/x
v_p = 1.0/x
v_c = 1.2 * v_p
threshold_distance_squared = 1.0/(x**2)
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


fig8_1 = Circle(radius=r, color=GREY)
fig8_2 = Circle(radius=r, color=GREY)
fig8_1.rotate(PI*2)
fig8_1.move_to(r*UP)
fig8_2.move_to(r*DOWN)
fig8 = Group(fig8_1, fig8_2)

figinf_1 = Circle(radius=r, color=GREY)
figinf_2 = Circle(radius=r, color=GREY)
figinf_1.move_to(r*RIGHT)
figinf_2.move_to(r*LEFT)
figinf = Group(figinf_1, figinf_2)

circle = Circle(radius=R, color=GREY)
CPcircle = Circle(radius=90/80*R, color=GREY_D).move_to([0,0,0])

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

def cpphase(phase):
    return CPcircle.point_at_angle(-phase)

def target(targeting, phase):
    if targeting == 'Circle':
        target_point = circlephase(phase)
    elif targeting == '8':
        target_point = fig8phase(phase)
    elif targeting == 'Infinity':
        target_point = figinfphase(phase)
    elif targeting == 'CP':
        target_point = cpphase(phase)
    return target_point

# ------------------------------------------------------------ #
# ace configuration (initial)                                  #
# ------------------------------------------------------------ #

targeting = "Infinity"

# done in degrees lol
events = {

}
onPath = True
reverse = 1  # 1 = Normal, -1 = Reverse
v_angle = 0  # if onPath initially true no need to set this except the first frame will be fucked
initial_phase = 0
final_phase = PI

initial_location = target(targeting, initial_phase)


# more code

phase = ValueTracker(initial_phase)

circledot = Dot(color=RED)
circledot.add_updater(lambda x: x.move_to(circlephase(phase.get_value())))
fig8dot = Dot(color=GREEN)
fig8dot.add_updater(lambda x: x.move_to(fig8phase(phase.get_value())))
figinfdot = Dot(color=BLUE)
figinfdot.add_updater(lambda x: x.move_to(figinfphase(phase.get_value())))
CPcircledot = Dot(color=PURPLE)
CPcircledot.add_updater(lambda x: x.move_to(cpphase(phase.get_value())))
if onPath:
    color = YELLOW
else:
    color = WHITE
ace = Dot(point=initial_location, color=color)

def acemove(ace, phase, line):
    global targeting
    global onPath
    global v_angle

    target_point = target(targeting, phase)

    def applyEvent(phase):
        global targeting
        global onPath
        global events
        phase_deg = int(phase / DEGREES)
        if phase_deg in events:
            event = events[phase_deg]
            onPath = False
            ace.set_fill(WHITE)
            targeting = event
            del events[phase_deg]
    if onPath:
        phi = constrain_to_tau(phase)
        if targeting == 'Circle' or targeting == 'CP':
            v_angle = -PI/2 - phi
        elif targeting == '8':
            if phi < PI:
                v_angle = PI - 2*phi
            else:
                v_angle = 2*(phi-PI) - PI
        elif targeting == 'Infinity':
            if phi < PI:
                v_angle = PI/2 - 2*phi
            else:
                v_angle = 2*phi + PI/2
        if reverse == -1:
            v_angle -= PI

        applyEvent(phase)
        return target_point

    ace_position = ace.get_center()

    # onPath check
    d_squared = (target_point[0] - ace_position[0]
                 )**2 + (target_point[1] - ace_position[1])**2
    if d_squared < threshold_distance_squared:
        onPath = True
        ace.set_fill(YELLOW)  # visual indicator
        # extremely scuffed method to hide the line by making it effectively 0 length
        line.put_start_and_end_on([0, 0, 0], [1e-9, 0, 0])

    # move the ace
    delta_x = target_point[0] - ace_position[0]
    delta_y = target_point[1] - ace_position[1]
    angle = math.atan2(delta_y, delta_x)
    angle_diff = constrain_to_pi(angle - v_angle)

    if abs(angle_diff) < max_delta_angle:
        v_angle += angle_diff
    else:
        v_angle += np.sign(angle_diff) * max_delta_angle
    # v_angle = constrain_to_pi(v_angle)

    line.put_start_and_end_on(ace_position, target_point)

    applyEvent(phase)
        
    return ace_position + [v_c*math.cos(v_angle), v_c*math.sin(v_angle), 0]


# for the ace path
path = VMobject(stroke_color=YELLOW)
path.set_points_as_corners([ace.get_center(), ace.get_center()])
def update_path(path):
    previous_path = path.copy()
    previous_path.add_points_as_corners([ace.get_center()])
    path.become(previous_path)
path.add_updater(update_path)

# velocity arrow
varrow = Arrow(start=ace.get_center(), end=ace.get_center() + [math.cos(v_angle), math.sin(v_angle), 0])
def update_varrow(varrow):
    arrow_length = 0.5  # arrow length of 0.5
    varrow.put_start_and_end_on(ace.get_center(), ace.get_center() + [math.cos(v_angle) * arrow_length, math.sin(v_angle) * arrow_length, 0])
varrow.add_updater(update_varrow)

line = Line(start=[0, 0, 0], end=[1e-9, 0, 0])

ace.add_updater(lambda x: x.move_to(acemove(x, phase.get_value(), line)))

class CP(Scene):
    def construct(self):
        global initial_phase
        global final_phase
        global reverse
        global onPath
        global targeting

        self.add(fig8)
        self.add(figinf)
        self.add(circle)
        self.add(CPcircle)
        # self.play(FadeIn(circle))
        self.add(fig8dot)
        self.add(figinfdot)
        self.add(circledot)
        # self.add(CPcircledot)

        self.add(line)
        self.add(path)
        self.add(varrow)
        self.add(ace)

        self.play(phase.animate.set_value(final_phase), rate_func=linear, run_time=abs(final_phase-initial_phase)/TAU * P)
        
        # switch to CP
        self.remove(circledot, fig8dot, figinfdot)
        self.add(CPcircledot)
        ace.set_z_index(1)
        fig8.set_color(GREY_D)
        figinf.set_color(GREY_D)
        circle.set_color(GREY_D)
        CPcircle.set_color(GREY)
        
        path.clear_points()
        path.set_points_as_corners([ace.get_center(), ace.get_center()])
        
        targeting = 'CP'
        onPath = False
        ace.set_fill(WHITE)
        
        initial_phase = final_phase
        final_phase = initial_phase + PI
        self.play(phase.animate.set_value(final_phase), rate_func=linear, run_time=abs(final_phase-initial_phase)/TAU * (P * 90/80))
        