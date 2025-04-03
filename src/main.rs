use std::{io, result};
use std::f64::consts::E;
use clap::{Arg, Command};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use serde_json;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use nalgebra as na;
use mint::Point2;

#[derive(Serialize, Deserialize, Debug)]
struct Planet {
    mass: f64,
    radius: f64,
}

#[derive(Debug)]
struct OrbitalSimulation {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    acceleration: na::Vector2<f32>,
    gravity: f32,
    planet_radius: f32,
    planet_position: na::Point2<f32>,
    zoom_scale: f32,
}

impl OrbitalSimulation {
    fn new() -> Self {
        // Scale factor: 1 pixel = 10,000 meters
        let scale_factor = 0.0001; // 1/10000
        let earth_radius = 6_371_000.0 * scale_factor; // Earth's radius in meters, scaled to pixels
        
        Self {
            position: na::Point2::new(400.0, 300.0 - earth_radius - 10.0),
            velocity: na::Vector2::new(0.0, 100.0),
            acceleration: na::Vector2::zeros(),
            gravity: 9.81,
            planet_radius: earth_radius,
            planet_position: na::Point2::new(400.0, 300.0),
            zoom_scale: 1.0,
        }
    }
}

impl EventHandler for OrbitalSimulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update velocity with gravity (negative because gravity pulls down)
        self.velocity.y += -self.gravity * 0.016;
        
        // Update position with velocity (need to negate y for screen coordinates)
        // Scale position change to match our planet scale
        self.position.y -= (self.velocity.y * 0.016) * 0.0001;

        // Calculate distance to planet center
        let distance = (self.position - self.planet_position).norm();
        
        // Stop simulation if object hits planet or goes off screen
        if distance <= self.planet_radius || self.position.y >= 600.0 {
            self.velocity.y = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Apply zoom to positions and sizes
        let scaled_planet_radius = self.planet_radius * self.zoom_scale;
        let center_x = 400.0;
        let center_y = 300.0;
        
        // Calculate positions relative to center, apply zoom, then translate back
        let scaled_planet_pos = Point2 {
            x: center_x,
            y: center_y,
        };
        
        let relative_x = self.position.x - center_x;
        let relative_y = self.position.y - center_y;
        let scaled_pos = Point2 {
            x: center_x + relative_x * self.zoom_scale,
            y: center_y + relative_y * self.zoom_scale,
        };

        // Draw planet
        let planet = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            scaled_planet_pos,
            scaled_planet_radius,
            2.0,
            Color::BLUE,
        )?;

        // Draw projectile
        let projectile = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            scaled_pos,
            5.0 * self.zoom_scale,
            2.0,
            Color::WHITE,
        )?;

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&planet, graphics::DrawParam::default());
        canvas.draw(&projectile, graphics::DrawParam::default());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        // Adjust zoom based on mouse wheel
        let zoom_speed = 0.1;
        if y > 0.0 {
            self.zoom_scale *= 1.0 + zoom_speed;
        } else if y < 0.0 {
            self.zoom_scale *= 1.0 - zoom_speed;
        }
        
        // Clamp zoom scale to reasonable values
        self.zoom_scale = self.zoom_scale.clamp(0.1, 10.0);
        
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Orbital Escape Velocity", "dev")
        .build()
        .expect("Failed to create ggez context");

    let game = OrbitalSimulation::new();
    event::run(ctx, event_loop, game)
}