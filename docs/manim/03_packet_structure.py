from manim import *

class PacketScene(Scene):
    def construct(self):
        title = Text("Packet Encapsulation").to_edge(UP)
        self.play(Write(title))

        # Original IP Packet
        ip_packet = Rectangle(width=5, height=1, color=BLUE)
        ip_text = Text("Original IP Packet (from TUN)").scale(0.5)
        ip_group = VGroup(ip_packet, ip_text).move_to(UP*2)
        self.play(Create(ip_group))
        self.wait(1)

        # onebox Header
        onebox_header = Rectangle(width=3, height=1, color=YELLOW)
        onebox_text = Text("onebox Header").scale(0.5)
        onebox_group = VGroup(onebox_header, onebox_text).next_to(ip_group, DOWN, buff=1)
        self.play(FadeIn(onebox_group))
        self.wait(1)

        # Encapsulation
        self.play(
            ip_group.animate.next_to(onebox_header, RIGHT, buff=0),
            FadeOut(onebox_text),
            FadeOut(ip_text)
        )
        payload_group = VGroup(onebox_header, ip_packet)
        self.play(payload_group.animate.center())

        encryption_box = SurroundingRectangle(ip_packet, buff=0, color=RED)
        encryption_text = Text("Encrypted", color=RED).scale(0.5).next_to(encryption_box, DOWN)
        self.play(Create(encryption_box), Write(encryption_text))
        self.wait(1)

        # UDP Header
        udp_header = Rectangle(width=2, height=1.2, color=GREEN)
        udp_text = Text("UDP Hdr").scale(0.5)
        udp_group = VGroup(udp_header, udp_text).next_to(payload_group, LEFT, buff=0)

        final_packet = VGroup(udp_group, payload_group, encryption_box, encryption_text)
        self.play(Create(udp_group))
        self.play(final_packet.animate.center())

        final_text = Text("Final UDP Datagram sent over WAN").scale(0.7).to_edge(DOWN)
        self.play(Write(final_text))
        self.wait(2)
