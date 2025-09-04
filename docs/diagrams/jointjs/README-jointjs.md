# JointJS Animation Proof-of-Concept

This directory contains a proof-of-concept demonstrating an animated version of the `onebox-rs` system overview diagram using the [JointJS](https://www.jointjs.com/) library.

## Description

The `index.html` file sets up a simple diagram with four main components:
- `onebox-client`
- `WAN 1`
- `WAN 2`
- `onebox-server`

It then uses JavaScript to animate small blue circles (representing data packets or "tokens") traveling along the links connecting these components. This simulates the flow of data from the client, across the two WAN links, to the server.

The animation is achieved using the `sendToken()` method provided by JointJS, inspired by the "Tokens" demo on their official website.

## How to View

1.  **Clone or download the repository.**
2.  **Navigate to this directory** (`docs/diagrams/jointjs/`).
3.  **Open the `index.html` file in a modern web browser** (e.g., Chrome, Firefox, Safari, Edge).

No local server is needed, as all dependencies (JointJS, jQuery, Lodash, Backbone) are loaded from public CDNs.
