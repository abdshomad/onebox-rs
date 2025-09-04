# three.js Animation Proof-of-Concept

This directory contains a proof-of-concept demonstrating an animated, 3D version of the `onebox-rs` system overview diagram using the [three.js](https://threejs.org/) library.

## Description

The `index.html` file sets up a WebGL scene containing a 3D representation of the system architecture.
- The main components (`onebox-client`, `onebox-server`, etc.) are represented by 3D cubes with a metallic-style material that responds to light.
- The connections are represented by simple lines.
- The entire scene slowly rotates to give a better view of the 3D space.
- Small blue spheres, representing data packets, are animated moving along the connection lines to simulate data flow from the client to the server via the WAN nodes.

This POC demonstrates how a more complex 3D visualization could be used to represent the system, offering a different perspective than traditional 2D diagrams.

## How to View

1.  **Clone or download the repository.**
2.  **Navigate to this directory** (`docs/diagrams/threejs/`).
3.  **Open the `index.html` file in a modern web browser** that supports WebGL (e.g., Chrome, Firefox, Safari, Edge).

No local server is needed, as the `three.js` library is loaded as a module from a public CDN.
