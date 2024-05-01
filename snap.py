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


def target(targeting, phase):
    if targeting == 'Circle':
        target_point = circlephase(phase)
    elif targeting == '8':
        target_point = fig8phase(phase)
    elif targeting == 'Infinity':
        target_point = figinfphase(phase)
    return target_point

# ------------------------------------------------------------ #
# ace configuration (initial)                                  #
# ------------------------------------------------------------ #

targeting = "Circle"

# done in degrees lol
events = {
    # 365: '8',
    # 385: 'Circle',
    # 400: 'Infinity',
    # 420: '8',
    # 430: 'Circle',
    # 440: 'Infinity',
    # 455: '8',
    # 470: 'Circle',
    # 505: 'Infinity'
}
onPath = False
reverse = 1  # 1 = Normal, -1 = Reverse
v_angle = 0  # if onPath initially true no need to set this except the first frame will be fucked
initial_phase = 0 * DEGREES
final_phase = initial_phase + 1.2 - 4* 0.0125

# almost always False
justOnPath = False

# initial_location = target(targeting, initial_phase)
initial_location = [30/80*R, 0, 0]


# more code
phase = ValueTracker(initial_phase)

circledot = Dot(color=RED)
circledot.add_updater(lambda x: x.move_to(circlephase(phase.get_value())))
fig8dot = Dot(color=GREEN)
fig8dot.add_updater(lambda x: x.move_to(fig8phase(phase.get_value())))
figinfdot = Dot(color=BLUE)
figinfdot.add_updater(lambda x: x.move_to(figinfphase(phase.get_value())))
if onPath:
    color = YELLOW
else:
    color = WHITE
ace = Dot(point=initial_location, color=color)

def acemove(ace, phase, line):
    global targeting
    global onPath
    global v_angle
    global justOnPath
    
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
            
    def updatePath(new_pos):
        global path
        global justOnPath

        previous_path = path.copy()
        previous_path.add_points_as_corners([new_pos])
        path.become(previous_path)
        
    if onPath:
        if justOnPath:
            ace.set_fill(YELLOW)  # visual indicator
            justOnPath = False
        phi = constrain_to_tau(phase)
        if targeting == 'Circle':
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
        updatePath(target_point)
        return target_point

    ace_position = ace.get_center()

    # onPath check
    

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
    new_pos = ace_position + [v_c*math.cos(v_angle), v_c*math.sin(v_angle), 0]
    updatePath(new_pos)
    
    d_squared = (target_point[0] - new_pos[0]
                 )**2 + (target_point[1] - new_pos[1])**2
    if d_squared < threshold_distance_squared:
        onPath = True
        justOnPath = True
        # extremely scuffed method to hide the line
        line.put_start_and_end_on([999, 0, 0], [999, 1, 0])
    
    return new_pos
ace.add_updater(lambda x: x.move_to(acemove(x, phase.get_value(), line)))


# for the ace path
path = VMobject(stroke_color=YELLOW)
path.set_points_as_corners([ace.get_center(), ace.get_center()])
def update_path(path):
    previous_path = path.copy()
    previous_path.add_points_as_corners([ace.get_center()])
    path.become(previous_path)
# path.add_updater(update_path)

# velocity arrow
varrow = Arrow(start=ace.get_center(), end=ace.get_center() + [math.cos(v_angle), math.sin(v_angle), 0])
def update_varrow(varrow):
    arrow_length = 0.5  # arrow length of 0.5
    varrow.put_start_and_end_on(ace.get_center(), ace.get_center() + [math.cos(v_angle) * arrow_length, math.sin(v_angle) * arrow_length, 0])
varrow.add_updater(update_varrow)

line = Line(start=[0, 0, 0], end=[1e-9, 0, 0])


text = Text(str(int(phase.get_value()/DEGREES))).shift(3*RIGHT)
text.add_updater(lambda t: t.become(Text(str(int(phase.get_value()/DEGREES)))))


class SnapOnPath(MovingCameraScene):
    def construct(self):
        global initial_phase
        global final_phase
        global reverse
        global onPath
        global events
        
        self.camera.frame.save_state()
        
        # self.add(fig8)
        # self.add(figinf)
        self.add(circle)
        # self.add(fig8dot)
        # self.add(figinfdot)
        self.add(circledot)

        # self.add(line)
        self.add(path)
        # self.add(varrow)
        self.add(ace)
        # self.add(text)
        
        destination = target('Circle', final_phase)
        run_time = abs(final_phase-initial_phase)/TAU * P
        
        zoom = 0.03
        
        phase_change = phase.animate(rate_func=linear, run_time=run_time).set_value(final_phase)
        camera_zoom = self.camera.frame.animate(rate_func=linear).scale(zoom).move_to(destination)
        
        circledot.scale(zoom)
        ace.scale(zoom)
        path.set_stroke(width=zoom*5)
        
        # line.set_stroke(width=zoom)
        # ace_shrink = ace.animate(rate_func=linear).scale(zoom)
        
        # camera_move = self.camera.frame.animate(rate_func=linear)
        
        # self.play(LaggedStart(phase_change, rate_func=linear, run_time=run_time), rate_func=linear, run_time=run_time)
        # self.play(AnimationGroup(phase_change, camera_zoom, circledot_shrink, ace_shrink))
        
        self.play(AnimationGroup(phase_change, camera_zoom))
        self.wait(1)
        
        ace.clear_updaters()
        circledot.clear_updaters()
        
        detection_circle = Circle(radius=R/80, stroke_width=zoom*5, color=BLUE).move_to(circledot.get_center())
        self.play(GrowFromCenter(detection_circle))
        self.play(detection_circle.animate.set_stroke(color='#ff0000'))
        self.play(FadeOut(detection_circle))
        
        phi = phase.get_value() + 1.0/80
        circledot.move_to(circlephase(phi))
        ace.move_to(acemove(ace, phi, line))
        self.wait(1)
        
        detection_circle = Circle(radius=R/80, stroke_width=zoom*5, color=BLUE).move_to(circledot.get_center())
        self.play(GrowFromCenter(detection_circle))
        self.play(detection_circle.animate.set_stroke(color='#00ff00'))
        self.play(FadeOut(detection_circle))
        
        
        phi += 1/80
        circledot.move_to(circlephase(phi))
        ace.move_to(acemove(ace, phi, line))
        
        self.wait(1)
        self.play(Restore(self.camera.frame))
        detection_circle = Circle(radius=R/80, stroke_width=1, color=YELLOW).move_to(circledot.get_center())
        self.play(GrowFromCenter(detection_circle))
        self.wait(1)
        
        # self.wait(1)
        return