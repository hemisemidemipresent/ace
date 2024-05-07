from manim import *
import math
import numpy as np

# set frame rate
config.frame_rate = 60.0

# manim converted units (x btd6 units = 1 manim unit)
x = 40 # bigger x = more zoomed out
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
def get_v_angle(phase):
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
    return v_angle

# ------------------------------------------------------------ #
# ace configuration (initial)                                  #
# ------------------------------------------------------------ #


events = {
    # 76: '8',
}
onPath = True
reverse = 1  # 1 = Normal, -1 = Reverse
v_angle = 0  # if onPath initially true no need to set this

targeting = "Infinity"
initial_phase = 7/16*TAU
final_phase = initial_phase + TAU
# woor settings
woor_start = initial_phase + 0.35 * PI
woor_end = woor_start + 2/3 * PI
target1 = 'Circle'
target2 = 'Infinity'
woor_period = 2 # every frame?

# last_woor = woor_start - woor_period
last_woor = 0

justOnPath = False # 99.99% of the time it is False

initial_location = target(targeting, initial_phase)
# initial_location = [30/80*R, 0, 0]

if onPath:
    v_angle = get_v_angle(initial_phase)
    
i = -1
    
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
ace2 = Dot(point=initial_location, color=color)

def acemove(ace, phase, line, path, temporal_offset=0):
    global targeting
    global onPath
    global v_angle
    global justOnPath
    global i
    i += 1

    def applyEvent(phase):
        global targeting
        global onPath
        global last_woor
        global events
        global i
        if woor_start < phase and phase < woor_end and i > last_woor + woor_period:
        # if woor_start > phase and phase > woor_end and phase < last_woor - woor_period:
        # if woor_start > phase and phase > woor_end and i > last_woor + woor_period:
            if targeting == target1 and i > last_woor: #+ woor_period + 1: #bias towards circle to delay switching to infinity 
                targeting = target2
                last_woor = i
                
            elif targeting == target2:
                targeting = target1
                last_woor = i
            onPath = False
            ace.set_fill(WHITE)
        if i in events:
            event = events[i]
            onPath = False
            targeting = event 
            ace.set_fill(WHITE)
            
    applyEvent(phase)
    target_point = target(targeting, phase)

    def updatePath(new_pos, path):
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
        updatePath(target_point, path)
        return target_point

    ace_position = ace.get_center()

    

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

    new_pos = ace_position + [v_c*math.cos(v_angle), v_c*math.sin(v_angle), 0]
    updatePath(new_pos, path)
    
    # onPath check
    d_squared = (target_point[0] - new_pos[0]
                 )**2 + (target_point[1] - new_pos[1])**2
    if d_squared < threshold_distance_squared:
        onPath = True
        justOnPath = True
        # extremely scuffed method to hide the line
        line.put_start_and_end_on([999, 0, 0], [999, 1, 0])
    
    return new_pos

ace.add_updater(lambda x: x.move_to(acemove(x, phase.get_value(), line, path)))
line = Line(start=[0, 0, 0], end=[1e-9, 0, 0])




# for the ace path
path = VMobject(stroke_color=YELLOW)
path.set_points_as_corners([ace.get_center(), ace.get_center()])

# velocity arrow
varrow = Arrow(start=ace.get_center(), end=ace.get_center() + [math.cos(v_angle), math.sin(v_angle), 0])
def update_varrow(varrow):
    arrow_length = 0.5  # arrow length of 0.5
    varrow.put_start_and_end_on(ace.get_center(), ace.get_center() + [math.cos(v_angle) * arrow_length, math.sin(v_angle) * arrow_length, 0])
varrow.add_updater(update_varrow)



text = Text(str(int(phase.get_value()/DEGREES))).shift(3*RIGHT)
text.add_updater(lambda t: t.become(Text(str(int(phase.get_value()/DEGREES)))))

class Woor(Scene):
    def construct(self):
        global initial_phase
        global final_phase
        global reverse
        global onPath
        global events
        
        
        self.add(fig8)
        self.add(figinf)
        self.add(circle)
        self.add(fig8dot)
        self.add(figinfdot)
        self.add(circledot)
        
        # self.play(LaggedStart(
        #     FadeIn(fig8),
        #     FadeIn(figinf),
        #     FadeIn(circle)
        # ))
        # self.play(LaggedStart(
        #     FadeIn(fig8dot),
        #     FadeIn(figinfdot),
        #     FadeIn(circledot)
        # ))
        self.add(line)
        self.add(path)
        self.add(varrow)
        self.add(ace)
        

        # self.add(text)
        
        run_time = abs(final_phase-initial_phase)/TAU * P
        
        
        phase_change = phase.animate(rate_func=linear, run_time=run_time).set_value(final_phase)        
        # line.set_stroke(width=zoom)
        # ace_shrink = ace.animate(rate_func=linear).scale(zoom)
        
        # camera_move = self.camera.frame.animate(rate_func=linear)
        
        # self.play(LaggedStart(phase_change, rate_func=linear, run_time=run_time), rate_func=linear, run_time=run_time)
        # self.play(AnimationGroup(phase_change, camera_zoom, circledot_shrink, ace_shrink))
        
        self.play(phase_change)

        # self.wait(1)
        return
        # reverse animation
        
        ace.clear_updaters() # freeze ace position

        # reset path
        self.play(FadeOut(path))
        path.clear_points()
        path.set_points_as_corners([ace.get_center(), ace.get_center()])

        # move the circledot back
        initial_phase = phase.get_value()-reverse_phase_change
        final_phase = initial_phase - TAU
        self.play(phase.animate.set_value(initial_phase), rate_func=linear, run_time=reverse_phase_change/TAU * P)
        self.wait(1)
        
        self.add(path) # re-add path

        # manipulate state
        reverse = -reverse
        onPath = False
        events = {
            494: '8',
            472: 'Circle',
            440: 'Infinity',
            420: '8',
            390: 'Circle',
            370: 'Infinity',
            
        }


        ace.add_updater(lambda x: x.move_to(
            acemove(x, phase.get_value(), line)))

        self.play(phase.animate.set_value(final_phase), rate_func=linear, run_time=abs(final_phase-initial_phase)/TAU * P)
