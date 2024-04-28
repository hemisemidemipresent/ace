from manim import *

# constants from btd6 (not manim)
R = 40
v_p = 1
v_c = 1.2
threshold_distance_squared = 1.75
angle_difference = 0 #???
reverse_phase_change = PI/5
maximum_phase_change_per_frame = 0.1025

class CreateCircle(Scene):
    def construct(self):
        fig8_1 = Circle(radius=1, color=GREY)
        fig8_2 = Circle(radius=1, color=GREY)
        fig8_1.rotate(PI*2)
        fig8_1.move_to(UP)
        fig8_2.move_to(DOWN)
        fig8 = Group(fig8_1, fig8_2)
        self.add(fig8)
                
        figinf_1 = Circle(radius=1, color=GREY)
        figinf_2 = Circle(radius=1, color=GREY)
        figinf_1.move_to(RIGHT)
        figinf_2.move_to(LEFT)
        figinf = Group(figinf_1, figinf_2)
        self.add(figinf)
                
        circle = Circle(radius=2, color=GREY)
        # self.play(FadeIn(circle))
        self.add(circle)
        
        def circlephase(phase):
            return circle.point_at_angle(-phase)
        
        def fig8phase(phase):
            if phase <= PI:
                return fig8_1.point_at_angle(3/2*PI - 2*phase)
            else:
                return fig8_2.point_at_angle(-3/2*PI + 2*phase)
        
        def figinfphase(phase):
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
        
        self.add(circledot)
        self.add(fig8dot)
        self.add(figinfdot)
        
        
        targetting = "Circle"
        onPath = False
        
        def acemove(phase):
            target_point = None
            if targetting == 'Circle':
                target_point = circlephase(phase)
            elif targetting == '8':
                target_point = fig8phase(phase)
            else:
                target_point = figinfphase(phase)
            
            ace_position = ace.get_center()
            
            d_squared = (target_point[0] - ace_position[0])**2 + (target_point[1] - ace_position[1])**2
            print(d_squared)
            
            # temp
            return target_point
            
        
        
        ace = Dot(color='#ffff00')
        ace.add_updater(lambda x: x.move_to(acemove(phase.get_value())))
        self.add(ace)
        # for the ace path
        # path = VMobject()
        # path.set_points_as_corners([dot.get_center(), dot.get_center()])
        # def update_path(path):
        #     previous_path = path.copy()
        #     previous_path.add_points_as_corners([dot.get_center()])
        #     path.become(previous_path)
        
        

        self.play(phase.animate.set_value(2*PI), rate_func=rate_functions.linear, run_time=TAU*R/60)