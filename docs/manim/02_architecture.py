from manim import *

class ArchitectureScene(Scene):
    def construct(self):
        title = Text("System Architecture").to_edge(UP)
        self.play(Write(title))

        # Create components
        client_box = Rectangle(width=3, height=2, color=BLUE).move_to(LEFT*4)
        client_text = Text("onebox-client").scale(0.5).move_to(client_box.get_center())
        client = VGroup(client_box, client_text)

        server_box = Rectangle(width=3, height=2, color=GREEN).move_to(RIGHT*4)
        server_text = Text("onebox-server").scale(0.5).move_to(server_box.get_center())
        server = VGroup(server_box, server_text)

        internet = Text("Public Internet").scale(0.7)
        cloud = Cloud().surround(internet)
        internet_group = VGroup(internet, cloud).move_to(RIGHT*4 + DOWN*3)

        lan_device = LabeledDot(Text("PC / Laptop"), radius=0.5).move_to(LEFT*6)

        self.play(Write(lan_device))
        self.play(Create(client))
        self.play(Create(server))
        self.play(FadeIn(internet_group))

        # Animate connections
        lan_to_client = Arrow(lan_device.get_right(), client.get_left(), buff=0.1)
        self.play(Create(lan_to_client))
        self.play(Write(Text("All LAN Traffic", font_size=20).next_to(lan_to_client, UP)))

        client_to_server1 = Arrow(client.get_right(), server.get_left(), buff=0.1, path_arc=-0.5, color=BLUE)
        client_to_server2 = Arrow(client.get_right(), server.get_left(), buff=0.1, path_arc=0.5, color=BLUE)

        self.play(Create(client_to_server1), Create(client_to_server2))
        self.play(Write(Text("Bonded Tunnel (UDP)", font_size=20).next_to(VGroup(client_to_server1, client_to_server2), UP)))

        server_to_internet = Arrow(server.get_bottom(), internet_group.get_top(), buff=0.1)
        self.play(Create(server_to_internet))
        self.play(Write(Text("Forwarded Traffic", font_size=20).next_to(server_to_internet, RIGHT)))

        self.wait(2)
