from manim import *

class LinkFailoverScene(Scene):
    def construct(self):
        # Title
        title = Text("Link Failover Workflow").to_edge(UP)
        self.play(Write(title))
        self.wait(1)

        # Create Client and Server objects
        client = LabeledDot(Text("Client", color=BLACK), radius=1).move_to(LEFT*4)
        server = LabeledDot(Text("Server", color=BLACK), radius=1).move_to(RIGHT*4)

        client_server_group = VGroup(client, server)
        self.play(Create(client_server_group))
        self.wait(1)

        # Create two links
        link1 = Line(client.get_right(), server.get_left(), color=GREEN, buff=0.1)
        link2 = Line(client.get_right(), server.get_left(), color=GREEN, buff=0.1).shift(DOWN*1.5)
        link1_text = Text("Link 1 (Healthy)", font_size=24).next_to(link1, UP)
        link2_text = Text("Link 2 (Healthy)", font_size=24).next_to(link2, UP)

        links_group = VGroup(link1, link2, link1_text, link2_text)
        self.play(Create(links_group))
        self.wait(1)

        # Animate normal probes
        self.play(Write(Text("Normal Health Probing...", font_size=24).to_edge(DOWN)))
        for _ in range(2):
            probe1 = Dot(color=YELLOW).move_to(client.get_center())
            probe2 = Dot(color=YELLOW).move_to(client.get_center())
            self.play(
                AnimationGroup(
                    probe1.animate.move_to(server.get_center()),
                    probe2.animate.move_to(server.get_center()).shift(DOWN*1.5),
                    lag_ratio=0.2
                )
            )
            self.play(
                AnimationGroup(
                    probe1.animate.move_to(client.get_center()),
                    probe2.animate.move_to(client.get_center()).shift(DOWN*1.5),
                    lag_ratio=0.2
                )
            )
        self.wait(1)

        # Link 2 fails
        self.play(Write(Text("Link 2 fails!", color=RED, font_size=24).next_to(link2_text, DOWN)))
        self.play(link2.animate.set_color(RED), link2_text.animate.set_color(RED))
        self.wait(1)

        # Animate failing probes on Link 2
        failure_counter = 0
        counter_text = Text(f"Consecutive Failures: {failure_counter}", font_size=24).to_edge(DOWN)
        self.play(Transform(self.mobjects[-1], counter_text))

        for i in range(3):
            failure_counter += 1
            probe1 = Dot(color=YELLOW).move_to(client.get_center())
            failing_probe = Dot(color=RED).move_to(client.get_center())

            self.play(
                probe1.animate.move_to(server.get_center()),
                failing_probe.animate.move_to(client.get_center() + RIGHT*2).shift(DOWN*1.5)
            )
            self.play(FadeOut(failing_probe))
            self.play(probe1.animate.move_to(client.get_center()))

            new_counter_text = Text(f"Consecutive Failures: {failure_counter}", font_size=24).to_edge(DOWN)
            self.play(Transform(counter_text, new_counter_text))
            self.wait(0.5)

        # Mark link as down
        self.play(FadeOut(link2), FadeOut(link2_text))
        final_text = Text("Link 2 marked as DOWN. Traffic routed via Link 1 only.", color=ORANGE, font_size=24).to_edge(DOWN)
        self.play(Transform(counter_text, final_text))
        self.wait(2)
