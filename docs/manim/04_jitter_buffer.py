from manim import *

class JitterBufferScene(Scene):
    def construct(self):
        title = Text("Jitter Buffer Logic").to_edge(UP)
        self.play(Write(title))

        # Jitter Buffer visual
        buffer = Brace(Line(LEFT*4, RIGHT*4), sharpness=1).shift(DOWN*2)
        buffer_text = Text("Jitter Buffer (Sorted Map)").scale(0.5).next_to(buffer, DOWN)
        self.play(Create(buffer), Write(buffer_text))

        # Packets arriving out of order
        packets_in = {
            1: LabeledDot(Text("Seq 1")),
            3: LabeledDot(Text("Seq 3")),
            2: LabeledDot(Text("Seq 2")),
            5: LabeledDot(Text("Seq 5")),
            4: LabeledDot(Text("Seq 4")),
        }

        arrival_text = Text("Packets arrive out of order...").to_edge(UP, buff=1.5)
        self.play(Write(arrival_text))

        anims = []
        for seq, p in packets_in.items():
            p.move_to(UP*2 + LEFT*6 + RIGHT*2*seq)
            anims.append(FadeIn(p))
        self.play(AnimationGroup(*anims, lag_ratio=0.2))
        self.wait(1)

        # Insert into buffer (which sorts them automatically)
        self.play(Transform(arrival_text, Text("...and are placed in the buffer.").to_edge(UP, buff=1.5)))

        sorted_packets = VGroup(*[packets_in[i] for i in sorted(packets_in.keys())])
        self.play(sorted_packets.animate.arrange(RIGHT, buff=0.5).next_to(buffer, UP))
        self.wait(1)

        # Drain buffer in order
        next_seq = 1
        next_seq_tracker = Variable(next_seq, Text("next_expected_seq"), var_type=Integer).to_edge(LEFT)
        self.play(Transform(arrival_text, Text("Packets are drained sequentially.").to_edge(UP, buff=1.5)), Create(next_seq_tracker))

        for seq in sorted(packets_in.keys()):
            packet_to_drain = packets_in[seq]
            self.play(Indicate(packet_to_drain, color=GREEN))
            self.play(packet_to_drain.animate.shift(UP*2))
            self.play(FadeOut(packet_to_drain))

            next_seq += 1
            self.play(next_seq_tracker.tracker.animate.set_value(next_seq))
            self.wait(0.5)

        self.wait(2)
