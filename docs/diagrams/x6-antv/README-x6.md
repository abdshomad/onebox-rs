# X6 AntV Animation Proof-of-Concept

This directory contains a proof-of-concept demonstrating an animated version of the `onebox-rs` system overview diagram using the [X6 AntV](https://x6.antv.antgroup.com/) library.

## Description

The `index.html` file sets up a simple diagram with four main components:
- `onebox-client`
- `WAN 1`
- `WAN 2`
- `onebox-server`

It then uses JavaScript to simulate the flow of data from the client to the server. Since the `sendToken` API is deprecated in X6, this animation is achieved by sequentially highlighting and un-highlighting the edges of the graph to create a "blinking" path effect.

## How to View

1.  **Clone or download the repository.**
2.  **Navigate to this directory** (`docs/diagrams/x6-antv/`).
3.  **Open the `index.html` file in a modern web browser** (e.g., Chrome, Firefox, Safari, Edge).

No local server is needed, as the X6 library is loaded from a public CDN.
