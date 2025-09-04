from manim import *

class ServerFlowchartScene(Scene):
    def construct(self):
        title = Text("Server Logic Flowchart").to_edge(UP)
        self.play(Write(title))

        # Initial setup nodes
        start = LabeledDot(Text("Start"))
        config = Rectangle(width=2.5, height=1, color=BLUE).next_to(start, DOWN, buff=0.5)
        config_text = Text("Load Config", font_size=24).move_to(config.get_center())
        config_group = VGroup(config, config_text)

        tun_setup = Rectangle(width=2.5, height=1, color=BLUE).next_to(config_group, DOWN, buff=0.5)
        tun_text = Text("Setup TUN & NAT", font_size=24).move_to(tun_setup.get_center())
        tun_group = VGroup(tun_setup, tun_text)

        socket = Rectangle(width=2.5, height=1, color=BLUE).next_to(tun_group, DOWN, buff=0.5)
        socket_text = Text("Bind UDP Socket", font_size=24).move_to(socket.get_center())
        socket_group = VGroup(socket, socket_text)

        spawn = Diamond().scale(1.5).next_to(socket_group, DOWN, buff=0.75)
        spawn_text = Text("Spawn Tasks", font_size=24).move_to(spawn.get_center())
        spawn_group = VGroup(spawn, spawn_text)

        self.play(Write(start))
        self.play(Create(Arrow(start.get_bottom(), config_group.get_top())), Write(config_group))
        self.play(Create(Arrow(config_group.get_bottom(), tun_group.get_top())), Write(tun_group))
        self.play(Create(Arrow(tun_group.get_bottom(), socket_group.get_top())), Write(socket_group))
        self.play(Create(Arrow(socket_group.get_bottom(), spawn_group.get_top())), Write(spawn_group))

        # Dispatcher and Worker model
        dispatcher = RoundedRectangle(width=3, height=4, corner_radius=0.5, color=ORANGE).next_to(spawn_group, LEFT, buff=1)
        dispatcher_text = Text("Dispatcher Task", font_size=20).next_to(dispatcher, UP)

        worker = RoundedRectangle(width=3, height=4, corner_radius=0.5, color=PURPLE).next_to(spawn_group, RIGHT, buff=1)
        worker_text = Text("Worker Task", font_size=20).next_to(worker, UP)

        self.play(Create(DashedLine(spawn_group.get_left(), dispatcher.get_right())), Create(dispatcher), Write(dispatcher_text))
        self.play(Create(DashedLine(spawn_group.get_right(), worker.get_left())), Create(worker), Write(worker_text))

        # Dispatcher logic
        disp_logic = Text("1. Receive from UDP\n2. Send to Worker", font_size=20).move_to(dispatcher.get_center())
        self.play(Write(disp_logic))

        # Worker logic
        work_logic = Text("1. Receive from Dispatcher\n2. Decrypt & Process\n3. Write to TUN", font_size=20).move_to(worker.get_center())
        self.play(Write(work_logic))

        # Animate a packet
        packet = Dot(color=YELLOW).move_to(disp_logic.get_top())
        self.play(FadeIn(packet))
        self.play(packet.animate.move_to(disp_logic.get_bottom()))
        self.play(packet.animate.move_to(work_logic.get_top()))
        self.play(packet.animate.move_to(work_logic.get_bottom()))
        self.play(FadeOut(packet))

        self.wait(3)
