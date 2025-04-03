Create a Rust program that:

Draws a planet at the center.

Launches a particle (spacecraft) with an initial velocity.

Updates the position using gravitational acceleration.


ggez::{Context, ContextBuilder, GameResult} → Used to set up the game environment.

ggez::event::{self, EventHandler} → Handles game events like updating physics and drawing objects.

ggez::graphics::{Color, DrawMode, Mesh} → Enables rendering (e.g., drawing the planet and spacecraft).

ggez::nalgebra as na → nalgebra is a math library for handling vectors and positions in 2D space.

