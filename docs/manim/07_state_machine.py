from manim import *

class StateMachineScene(Scene):
    def construct(self):
        title = Text("Link State Machine").to_edge(UP)
        self.play(Write(title))

        # Create states as nodes
        states = {
            "Unknown": LabeledDot(Text("Unknown")),
            "Up": LabeledDot(Text("Up", color=GREEN)),
            "Down": LabeledDot(Text("Down", color=RED)),
        }

        # Position states
        states["Unknown"].move_to(LEFT*4)
        states["Up"].move_to(ORIGIN)
        states["Down"].move_to(RIGHT*4)

        state_group = VGroup(*states.values())
        self.play(Create(state_group))

        # Create transitions
        unknown_to_up = Arrow(states["Unknown"].get_right(), states["Up"].get_left(), buff=0.1)
        unknown_to_up_text = Text("Successful Probe", font_size=24).next_to(unknown_to_up, UP)

        up_to_down = Arrow(states["Up"].get_right(), states["Down"].get_left(), buff=0.1)
        up_to_down_text = Text("4 Consecutive Failures", font_size=24).next_to(up_to_down, UP)

        down_to_up = Arrow(states["Down"].get_left(), states["Up"].get_right(), buff=0.1, path_arc=-1)
        down_to_up_text = Text("Successful Probe", font_size=24).next_to(down_to_up, DOWN)

        # Animate transitions
        self.play(Create(unknown_to_up), Write(unknown_to_up_text))
        self.wait(1)
        self.play(Create(up_to_down), Write(up_to_down_text))
        self.wait(1)
        self.play(Create(down_to_up), Write(down_to_up_text))

        # Highlight a path
        self.play(Indicate(states["Down"], scale_factor=1.5), Indicate(down_to_up), Indicate(states["Up"]))

        self.wait(3)
