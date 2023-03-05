# Handshaker
Handshake written using Rust Bitcoin Module

The handshake is written in a unit test. Asynchornously, a peer is instantiated by connecting to the socket address, sends a "version message" to a node, and verifies the message. Then a `verack` message is exchanged.

Run `cargo test` to handshake a Bitcoin Node.