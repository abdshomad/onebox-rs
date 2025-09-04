from manim import *

class ClientFlowchartScene(Scene):
    def construct(self):
        title = Text("Client Logic Flowchart").to_edge(UP)
        self.play(Write(title))

        # Define flowchart nodes
        nodes = {
            "start": (Circle().scale(0.5), "Start"),
            "cli": (Rectangle(width=2.5, height=1), "Parse CLI Args"),
            "config": (Rectangle(width=2.5, height=1), "Load Config"),
            "discover": (Rectangle(width=2.5, height=1), "Discover WANs"),
            "tun": (Rectangle(width=2.5, height=1), "Create TUN"),
            "route": (Rectangle(width=2.5, height=1), "Set Default Route"),
            "handshake": (Rectangle(width=2.5, height=1), "Handshake"),
            "spawn": (Diamond().scale(1.5), "Spawn Tasks"),
        }

        # Create Manim objects from definitions
        mobjects = {}
        for name, (shape, text) in nodes.items():
            shape.set_fill(BLUE, opacity=0.5).set_stroke(BLUE_E, width=4)
            if isinstance(shape, Diamond):
                shape.set_fill(GREEN, opacity=0.5).set_stroke(GREEN_E, width=4)
            label = Text(text, font_size=24).move_to(shape.get_center())
            mobjects[name] = VGroup(shape, label)

        # Position nodes
        mobjects["start"].to_edge(UP, buff=1.5)
        mobjects["cli"].next_to(mobjects["start"], DOWN, buff=0.5)
        mobjects["config"].next_to(mobjects["cli"], DOWN, buff=0.5)
        mobjects["discover"].next_to(mobjects["config"], DOWN, buff=0.5)
        mobjects["tun"].next_to(mobjects["discover"], DOWN, buff=0.5)
        mobjects["route"].next_to(mobjects["tun"], DOWN, buff=0.5)
        mobjects["handshake"].next_to(mobjects["route"], DOWN, buff=0.5)
        mobjects["spawn"].next_to(mobjects["handshake"], DOWN, buff=0.75)

        # Animate the main flow
        self.play(Write(mobjects["start"]))
        path = ["start", "cli", "config", "discover", "tun", "route", "handshake", "spawn"]
        for i in range(len(path) - 1):
            start_node = mobjects[path[i]]
            end_node = mobjects[path[i+1]]
            self.play(
                Create(Arrow(start_node.get_bottom(), end_node.get_top(), buff=0.1)),
                Write(end_node)
            )

        # Animate spawning tasks
        task_upstream = Text("Upstream Task (TUN -> UDP)", font_size=24).next_to(mobjects["spawn"], LEFT, buff=1).shift(UP*0.5)
        task_downstream = Text("Downstream Task (UDP -> TUN)", font_size=24).next_to(mobjects["spawn"], RIGHT, buff=1).shift(UP*0.5)
        task_prober = Text("Health Prober Tasks", font_size=24).next_to(mobjects["spawn"], LEFT, buff=1).shift(DOWN*0.5)
        task_status = Text("Status Socket Task", font_size=24).next_to(mobjects["spawn"], RIGHT, buff=1).shift(DOWN*0.5)

        self.play(
            Create(DashedLine(mobjects["spawn"].get_left(), task_upstream.get_right())),
            Write(task_upstream)
        )
        self.play(
            Create(DashedLine(mobjects["spawn"].get_right(), task_downstream.get_left())),
            Write(task_downstream)
        )
        self.play(
            Create(DashedLine(mobjects["spawn"].get_left(), task_prober.get_right())),
            Write(task_prober)
        )
        self.play(
            Create(DashedLine(mobjects["spawn"].get_right(), task_status.get_left())),
            Write(task_status)
        )

        self.wait(3)
