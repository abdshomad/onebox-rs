from manim import *

class ConfigSchemaScene(Scene):
    def construct(self):
        title = Text("Configuration Schema (config.toml)").to_edge(UP)
        self.play(Write(title))

        # Root node
        root = Text("config.toml", weight=BOLD).scale(0.8)
        self.play(Write(root))
        self.wait(0.5)

        # Top-level keys
        log_level = Text("log_level", font_size=32).next_to(root, DOWN, buff=1).align_to(root, LEFT)
        psk = Text("preshared_key", font_size=32).next_to(log_level, RIGHT, buff=1)

        self.play(
            Create(Line(root.get_bottom(), log_level.get_top())),
            Create(Line(root.get_bottom(), psk.get_top())),
            Write(log_level),
            Write(psk)
        )
        self.wait(0.5)

        # Client table
        client_table = Text("[client]", color=BLUE, font_size=36).next_to(log_level, DOWN, buff=1.5).shift(LEFT*2)
        self.play(Create(Line(root.get_bottom(), client_table.get_top())), Write(client_table))

        client_keys = VGroup(
            Text("server_address", font_size=28),
            Text("server_port", font_size=28),
            Text("tun_name", font_size=28),
            Text("tun_ip", font_size=28),
            Text("tun_netmask", font_size=28),
        ).arrange(DOWN, aligned_edge=LEFT).next_to(client_table, DOWN, buff=0.5)

        client_lines = VGroup()
        for key in client_keys:
            client_lines.add(DashedLine(client_table.get_bottom(), key.get_left()))

        self.play(Create(client_lines), Write(client_keys))
        self.wait(0.5)

        # Server table
        server_table = Text("[server]", color=GREEN, font_size=36).next_to(psk, DOWN, buff=1.5).shift(RIGHT*2)
        self.play(Create(Line(root.get_bottom(), server_table.get_top())), Write(server_table))

        server_keys = VGroup(
            Text("listen_address", font_size=28),
            Text("listen_port", font_size=28),
        ).arrange(DOWN, aligned_edge=LEFT).next_to(server_table, DOWN, buff=0.5)

        server_lines = VGroup()
        for key in server_keys:
            server_lines.add(DashedLine(server_table.get_bottom(), key.get_left()))

        self.play(Create(server_lines), Write(server_keys))
        self.wait(3)
